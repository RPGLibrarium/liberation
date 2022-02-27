use std::sync::Arc;
use jsonwebtoken::DecodingKey;
use log::{debug, error, info, Metadata, warn};
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures::future::{LocalBoxFuture};
use futures::FutureExt;
use jsonwebtoken::{Algorithm, decode, Validation};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::app::AppState;
use crate::Authenticator::Keycloak;
use crate::error::UserFacingError as UE;
use crate::error::InternalError as IE;
use crate::InternalError;
use crate::keycloak::RealmMetadata;

pub enum Authenticator {
    Keycloak {
        keycloak_url: String,
        realm: String,
        public_key: Arc<Mutex<DecodingKey>>,
    },
    Static { public_key: DecodingKey },
}

impl Authenticator {
    /// Creates a new authenticator which updates the key from keycloak periodically. Use the
    /// JoinHandle to stop the key updates.
    pub async fn with_rotating_keys(keycloak_url: String, realm: String, renewal_interval_s: u64) -> (Self, Option<JoinHandle<()>>) {
        use actix_web::rt::spawn;
        use std::time::Duration;
        use tokio::time;

        let public_key = Arc::new(Mutex::new(
            RealmMetadata::fetch_new(&keycloak_url, &realm).await
                .expect("Fetching fist realm metadata failed.")
                .decoding_key()
                .expect("Decoding first public key from keycloak failed.")
        ));

        let authenticator = Keycloak {
            keycloak_url: keycloak_url.to_string(),
            realm: realm.to_string(),
            public_key: public_key.clone(),
        };

        info!("Starting Keycloak Worker.");
        let worker = spawn(async move {
            let mut interval = time::interval(Duration::from_secs(renewal_interval_s));

            // Wrapping this in a function makes futures easier to handle
            async fn fetch_key(keycloak_url: &str, realm: &str) -> Result<DecodingKey, InternalError> {
                info!("Updating rotating keys from keycloak.");
                RealmMetadata::fetch_new(keycloak_url, realm).await?
                    .decoding_key()
            }

            loop {
                interval.tick().await;
                match fetch_key(&keycloak_url, &realm).await {
                    Ok(new_key) => {
                        debug!("Setting new key in shared mutable state.");
                        let mut lock = public_key.lock().await;
                        *lock = new_key;
                    }
                    Err(e) => error!("Could not public key: {:?}", e)
                }
            }
        });
        (authenticator, Some(worker))
    }

    /// Creates a new authenticator which uses a static key.
    pub fn with_static_key(static_key: String) -> Self {
        let static_key = DecodingKey::from_rsa_pem(static_key.as_bytes())
            .expect("The static key is not a valid pem key.");
        Authenticator::Static { public_key: static_key }
    }

    pub async fn verify_token(&self, token: &str) -> Result<Authentication, UE> {
        let key = match &self {
            Authenticator::Keycloak { public_key, .. } => (*public_key.lock().await).clone(),
            Authenticator::Static { public_key, .. } => public_key.clone(),
        };

        // TODO: require correct audience, subject, and issuer.
        // the jwt library checks exp and
        let validated = decode::<Claims>(&token, &key, &Validation::new(Algorithm::RS256))
            .map_err(|e| {
                UE::BadToken(format!("validation failed for token '{}' with error {}", token, e))
            })?;

        Ok(Authentication::from(validated.claims))
    }
}

type ExternalAccountId = String;
type ExternalGuildId = String;

pub mod roles {
    pub const ACCOUNTS_READ: &'static str = "liberation:accounts:read";
    pub const ACCOUNTS_EDIT: &'static str = "liberation:accounts:edit";
    pub const ACCOUNTS_DELETE: &'static str = "liberation:accounts:delete";
    pub const LIBRARIAN_ROLE_PREFIX: &'static str = "liberation:librarian:";
    pub const MEMBER_ROLE: &'static str = "liberation:member";
    pub const RPGSYSTEMS_EDIT: &'static str = "liberation:rpgsystems:edit";
    pub const RPGSYSTEMS_CREATE: &'static str = "liberation:rpgsystems:create";
    // pub const RPGSYSTEMS_DELETE: &'static str = "liberation:rpgsystems:delete";
    pub const TITLES_EDIT: &'static str = "liberation:titles:edit";
    pub const TITLES_CREATE: &'static str = "liberation:titles:create";
    // pub const TITLES_DELETE: &'static str = "liberation:titles:delete";
    pub const USERS_READ: &'static str = "liberation:users:read";
    pub const GUILDS_CREATE: &'static str = "liberation:guilds:create";
    pub const GUILDS_READ: &'static str = "liberation:guilds:read";
    pub const GUILDS_EDIT: &'static str = "liberation:guilds:edit";
    pub const BOOKS_CREATE: &'static str = "liberation:books:create";
    pub const BOOKS_READ: &'static str = "liberation:books:read";
    pub const BOOKS_EDIT: &'static str = "liberation:books:edit";
    pub const BOOKS_DELETE: &'static str = "liberation:books:edit";
}

/// Content of a JWT.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    /// Audience
    // pub aud: String,
    /// Expires at (UTC timestamp)
    exp: usize,
    /// Issued at (UTC timestamp)
    iat: usize,
    /// Issuer
    iss: String,
    /// Not Before (UTC timestamp)
    nbf: Option<usize>,
    /// Subject (the userid from keycloak)
    pub sub: ExternalAccountId,
    // Keycloak specific properties
    /// Roles (added manually in keycloak)
    pub realm_access: RealmAccess,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RealmAccess {
    pub roles: Vec<String>,
}

/// Represents a valid authentication.
#[derive(Debug)]
pub enum Authentication {
    Authorized {
        roles: Vec<String>,
        external_account_id: ExternalAccountId,
        full_name: String,
        given_name: String,
        family_name: String,
        email: String,
    },
    Anonymous,
}

impl Authentication {
    /// Returns an error, when a role is not fulfilled.
    pub fn requires_role(&self, required_role: &str) -> Result<(), UE> {
        match self {
            Authentication::Authorized { roles, .. }
            if roles.into_iter().any(|role| role == required_role) => Ok(()),
            Authentication::Authorized { roles, .. } => {
                warn!("Authentication failed! Role '{}' was required, but only '{:?}' were given.", required_role, roles);
                Err(UE::YouShallNotPass)
            }
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    /// Finds roles prefixed with a given prefix and returns their suffixes. Drops all roles
    /// which don't match the prefix.
    /// Returns an error, if prefixed role is found.
    pub fn requires_prefixed_role(&self, prefix: &str) -> Result<Vec<String>, UE> {
        match self {
            Authentication::Authorized { roles, .. } => {
                let suffixes: Vec<String> = roles.into_iter()
                    .filter_map(|role| role.strip_prefix(&prefix))
                    .map(str::to_string)
                    .collect();
                if suffixes.is_empty() {
                    warn!("Authentication failed! Role prefixed by '{}' was required, but only '{:?}' were given.", prefix, roles);
                    Err(UE::YouShallNotPass)
                } else { Ok(suffixes) }
            }
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    /// Utility function to require librarian privileges for any guild.
    pub fn requires_any_librarian(&self) -> Result<Vec<ExternalAccountId>, UE> {
        self.requires_prefixed_role(roles::LIBRARIAN_ROLE_PREFIX)
    }

    /// Utility function to require librarian privileges for a certain guild.
    pub fn requires_librarian(&self, required_guild_id: &ExternalGuildId) -> Result<(), UE> {
        if self.requires_any_librarian()?.contains(required_guild_id) {
            Ok(())
        } else { Err(UE::YouShallNotPass) }
    }

    /// Utility function to require member privileges for any member account.
    pub fn requires_any_member(&self) -> Result<ExternalAccountId, UE> {
        self.requires_role(roles::MEMBER_ROLE)?;
        match self {
            Authentication::Authorized { external_account_id, .. } => Ok(external_account_id.clone()),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_nothing(&self) -> Result<(), UE> { Ok(()) }
}

impl From<Claims> for Authentication {
    fn from(claims: Claims) -> Self {
        Authentication::Authorized {
            roles: claims.realm_access.roles,
            external_account_id: claims.sub,
            full_name: claims.name,
            given_name: claims.given_name,
            family_name: claims.family_name,
            email: claims.email,
        }
    }
}

impl FromRequest for Authentication {
    type Error = UE;
    type Future = LocalBoxFuture<'static, Result<Authentication, UE>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        // Futures are weird. Not sure if there is better way of doing this, but I would like to
        // make use of the early returns.
        // Found a very similar issue: https://github.com/actix/actix-web/discussions/2182
        // It appears as if this a restriction of rust traits which don't support async functions.
        async move {
            if let Some(header) = req.headers().get("Authorization") {
                let raw_token = header.to_str().map_err(|_| UE::BadToken("not a valid string".to_string()))?;

                let unvalidated = if raw_token.starts_with("Bearer ") {
                    raw_token.replacen("Bearer ", "", 1)
                } else {
                    return Err(UE::BadToken(raw_token.to_string()));
                };

                let authenticator = &req.app_data::<Data<AppState>>()
                    .ok_or(UE::Internal(IE::MissingAppState))?
                    .authenticator;

                authenticator.verify_token(&unvalidated).await
            } else { Ok(Authentication::Anonymous) }
        }.boxed_local()
    }
}

