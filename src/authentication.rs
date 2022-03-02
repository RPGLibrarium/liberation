use crate::app::AppState;
use crate::error::InternalError as IE;
use crate::error::UserFacingError as UE;
use crate::keycloak::RealmMetadata;
use crate::Authentication::Keycloak;
use crate::InternalError;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::{decode, Algorithm, Validation};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub enum Authentication {
    Keycloak {
        keycloak_url: String,
        realm: String,
        public_key: Arc<Mutex<DecodingKey>>,
    },
    Static {
        public_key: DecodingKey,
    },
}

impl Authentication {
    /// Creates a new authenticator which updates the key from keycloak periodically. Use the
    /// JoinHandle to stop the key updates.
    pub async fn with_rotating_keys(
        keycloak_url: String,
        realm: String,
        renewal_interval_s: u64,
    ) -> (Self, Option<JoinHandle<()>>) {
        use actix_web::rt::spawn;
        use std::time::Duration;
        use tokio::time;

        let public_key = Arc::new(Mutex::new(
            RealmMetadata::fetch_new(&keycloak_url, &realm)
                .await
                .expect("Fetching fist realm metadata failed.")
                .decoding_key()
                .expect("Decoding first public key from keycloak failed."),
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
            async fn fetch_key(
                keycloak_url: &str,
                realm: &str,
            ) -> Result<DecodingKey, InternalError> {
                info!("Updating rotating keys from keycloak.");
                RealmMetadata::fetch_new(keycloak_url, realm)
                    .await?
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
                    Err(e) => error!("Could not public key: {:?}", e),
                }
            }
        });
        (authenticator, Some(worker))
    }

    /// Creates a new authenticator which uses a static key.
    pub fn with_static_key(static_key: String) -> Self {
        let static_key = DecodingKey::from_rsa_pem(static_key.as_bytes())
            .expect("The static key is not a valid pem key.");
        Authentication::Static {
            public_key: static_key,
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims, UE> {
        let key = match &self {
            Authentication::Keycloak { public_key, .. } => (*public_key.lock().await).clone(),
            Authentication::Static { public_key, .. } => public_key.clone(),
        };

        // TODO: require correct audience, subject, and issuer.
        // the jwt library checks exp and
        let validated = decode::<JwtClaims>(&token, &key, &Validation::new(Algorithm::RS256))
            .map_err(|e| {
                UE::BadToken(format!("validation failed for token '{}' with error {}", token, e))
            })?;

        Ok(Claims::from(validated.claims))
    }
}

type ExternalAccountId = String;

pub mod scopes {
    // Global scopes
    pub const USERS_READ: &'static str = "users:read";
    pub const GUILDS_READ: &'static str = "guilds:read";
    pub const RPGSYSTEMS_ADD: &'static str = "rpgsystems:add";
    pub const TITLES_ADD: &'static str = "titles:add";

    // Personal scopes
    pub const ACCOUNT_READ: &'static str = "account:read";
    pub const ACCOUNT_REGISTER: &'static str = "account:modify";
    //pub const ACCOUNT_DELETE: &'static str = "account:delete";

    pub const COLLECTION_READ: &'static str = "collection:read";
    pub const COLLECTION_MODIFY: &'static str = "collection:modify";

    // pub const INVENTORY_READ: &'static str = "inventory:read";
    // pub const INVENTORY_MODIFY: &'static str = "inventory:modify";

    // Librarian specific scopes
    pub const GUILDS_COLLECTION_MODIFY: &'static str = "guilds:collection:modify";
    // pub const GUILDS_INVENTORY_MODIFY: &'static str = "guilds:inventory:modify";
    pub const LIBRARIAN_RPGSYSTEMS_MODIFY: &'static str = "librarian:rpgsystems:modify";
    pub const LIBRARIAN_TITLES_MODIFY: &'static str = "librarian:titles:modify";

    // Privileged scopes
    pub const ARISTOCRAT_ACCOUNTS_READ: &'static str = "aristocrat:accounts:read";
    pub const ARISTOCRAT_ACCOUNTS_MODIFY: &'static str = "aristocrat:accounts:modify";
    pub const ARISTOCRAT_BOOKS_READ: &'static str = "aristocrat:books:read";
    pub const ARISTOCRAT_BOOKS_MODIFY: &'static str = "aristocrat:books:modify";
    pub const ARISTOCRAT_GUILDS_MODIFY: &'static str = "aristocrat:guilds:modify";
}

/// Content of a JWT used by the library to deserialize the content.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct JwtClaims {
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
    /// space separated list of scopes
    pub scope: String,

    // the following properties are only added when the account:register scope is requested.
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
}

/// Represents a valid authentication.
#[derive(Debug)]
pub enum Claims {
    Authorized {
        scopes: HashSet<String>,
        external_account_id: ExternalAccountId,
        account_info: Option<AccountInfo>,
    },
    Anonymous,
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

impl Claims {
    /// Returns an error when a scope is not available
    pub fn require_scope(&self, required_scope: &str) -> Result<(), UE> {
        match self {
            Claims::Authorized { scopes, .. } if scopes.contains(required_scope) => Ok(()),
            Claims::Authorized { scopes, .. } => {
                warn!(
                    "Authentication failed! Role '{}' was required, but only '{:?}' were given.",
                    required_scope, scopes
                );
                Err(UE::YouShallNotPass)
            }
            Claims::Anonymous => Err(UE::AuthenticationRequired),
        }
    }

    /// Asserts that authentication was successful and returns the subject.
    pub fn external_account_id(&self) -> Result<ExternalAccountId, UE> {
        match self {
            Claims::Authorized {
                external_account_id,
                ..
            } => Ok(external_account_id.to_string()),
            Claims::Anonymous => Err(UE::AuthenticationRequired),
        }
    }

    /// Returns the account information that are provided with the account:register scope.
    pub fn account_info(&self) -> Result<AccountInfo, UE> {
        match self {
            Claims::Authorized {
                account_info: Some(info),
                ..
            } => Ok(info.clone()),
            Claims::Anonymous => Err(UE::AuthenticationRequired),
            _ => {
                warn!("No account info was provided.");
                Err(UE::YouShallNotPass)
            }
        }
    }

    /// Special case that always succeeds.
    /// Good style to avoid confusion ("did the programmer really want no check‽") and reject
    /// malformed tokens even though none might be required.
    pub fn requires_nothing(&self) -> Result<(), UE> {
        Ok(())
    }
}

impl From<JwtClaims> for Claims {
    fn from(claims: JwtClaims) -> Self {
        Claims::Authorized {
            scopes: claims.scope.split(" ").map(String::from).collect(),
            external_account_id: claims.sub,
            account_info: if let (
                Some(full_name),
                Some(given_name),
                Some(family_name),
                Some(email),
            ) = (
                claims.name,
                claims.given_name,
                claims.family_name,
                claims.email,
            ) {
                Some(AccountInfo {
                    full_name,
                    given_name,
                    family_name,
                    email,
                })
            } else {
                None
            },
        }
    }
}

impl FromRequest for Claims {
    type Error = UE;
    type Future = LocalBoxFuture<'static, Result<Claims, UE>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        // Futures are weird. Not sure if there is better way of doing this, but I would like to
        // make use of the early returns.
        // Found a very similar issue: https://github.com/actix/actix-web/discussions/2182
        // It appears as if this a restriction of rust traits which don't support async functions.
        async move {
            if let Some(header) = req.headers().get("Authorization") {
                let raw_token = header
                    .to_str()
                    .map_err(|_| UE::BadToken("not a valid string".to_string()))?;

                let unvalidated = if raw_token.starts_with("Bearer ") {
                    raw_token.replacen("Bearer ", "", 1)
                } else {
                    return Err(UE::BadToken(raw_token.to_string()));
                };

                let authenticator = &req
                    .app_data::<Data<AppState>>()
                    .ok_or(UE::Internal(IE::MissingAppState))?
                    .authenticator;

                authenticator.verify_token(&unvalidated).await
            } else {
                Ok(Claims::Anonymous)
            }
        }
        .boxed_local()
    }
}
