use jsonwebtoken::DecodingKey;
use log::{debug, error, info};
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures::future::{LocalBoxFuture};
use futures::FutureExt;
use jsonwebtoken::{Algorithm, decode, Validation};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use crate::app::AppState;
use crate::Authenticator::KeycloakLive;
use crate::error::UserFacingError as UE;
use crate::error::InternalError as IE;
use crate::keycloak::RealmMetadata;
use crate::settings::RoleMapping;

type AccountId = String;

pub enum Authenticator {
    KeycloakLive {
        keycloak_url: String,
        realm: String,
        public_key: Mutex<DecodingKey>,
        role_mapping: RoleMapping,
    },
    OauthStatic { public_key: DecodingKey, role_mapping: RoleMapping },
}

impl Authenticator {
    pub async fn with_rotating_keys(keycloak_url: &str, realm: &str, role_mapping: RoleMapping) -> Self {
        let key = RealmMetadata::fetch_new(keycloak_url, realm).await
            .expect("Fetching fist realm metadata failed.")
            .decoding_key()
            .expect("Decoding first public key from keycloak faile.");
        KeycloakLive {
            keycloak_url: keycloak_url.to_string(),
            realm: realm.to_string(),
            public_key: Mutex::new(key),
            role_mapping,
        }
    }

    pub fn with_static_key(static_key: String, role_mapping: RoleMapping) -> Self {
        let static_key = DecodingKey::from_rsa_pem(static_key.as_bytes())
            .expect("The static key is not a valid pem key.");
        Authenticator::OauthStatic { public_key: static_key, role_mapping }
    }

    pub async fn update(&self) -> Result<(), IE> {
        match &self {
            Authenticator::KeycloakLive { keycloak_url, realm, public_key, .. } => {
                info!("Updating rotating keys from keycloak.");
                let key = RealmMetadata::fetch_new(keycloak_url, realm).await?
                    .decoding_key()?;
                let mut lock = public_key.lock().await;
                debug!("Setting new key in shared mutable state.");
                *lock = key;
            }
            Authenticator::OauthStatic { .. } => debug!("Key is static, not updating.")
        }
        Ok(())
    }

    fn role_mapping(&self) -> &RoleMapping {
        match self {
            Authenticator::KeycloakLive { role_mapping, .. } => role_mapping,
            Authenticator::OauthStatic { role_mapping, .. } => role_mapping,
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<Authentication, UE> {
        let key = match &self {
            Authenticator::KeycloakLive { public_key, .. } => (*public_key.lock().await).clone(),
            Authenticator::OauthStatic { public_key, .. } => public_key.clone(),
        };

        let role_mapping = self.role_mapping();

        // TODO: require correct audience, subject, and issuer.
        // the jwt library checks exp and
        let validated = decode::<Claims>(&token, &key, &Validation::new(Algorithm::RS256))
            .map_err(|e| {
                UE::BadToken(format!("validation failed for token '{}' with error {}", token, e))
            })?;

        let validated_roles = validated.claims.realm_access.roles;
        let is_aristocrat = validated_roles.contains(&role_mapping.aristocrat_role);
        let account_id = if validated_roles.contains(&role_mapping.member_role) {
            validated.claims.sub
        } else {
            return Err(UE::BadToken(format!("missing claim '{}' in given '{:?}'", role_mapping.member_role, validated_roles)));
        };

        let librarian_for = validated_roles.into_iter()
            .filter_map(|token| token.strip_prefix(&role_mapping.librarian_role_prefix).map(str::to_string))
            .collect();

        Ok(Authentication::authorized(is_aristocrat, librarian_for, account_id))
    }
}

#[derive(Debug)]
pub enum Authentication {
    Authorized {
        is_aristocrat: bool,
        librarian_for: Vec<AccountId>,
        account_id: AccountId,
    },
    Anonymous,
}

impl Authentication {
    pub fn authorized(is_aristocrat: bool, librarian_for: Vec<AccountId>, account_id: AccountId) -> Self {
        let authentication = Authentication::Authorized {
            is_aristocrat,
            librarian_for,
            account_id,
        };
        debug!("Authenticated: {:?}", authentication);
        authentication
    }

    pub fn requires_aristocrat(&self) -> Result<(), UE> {
        match self {
            Authentication::Authorized { is_aristocrat: true, .. } => Ok(()),
            Authentication::Authorized { .. } => Err(UE::YouShallNotPass),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_librarian(&self, required_guild_id: AccountId) -> Result<(), UE> {
        match self {
            Authentication::Authorized { librarian_for, .. } if librarian_for.contains(&required_guild_id) => Ok(()),
            Authentication::Authorized { .. } => Err(UE::YouShallNotPass),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_any_librarian(&self) -> Result<Vec<AccountId>, UE> {
        match self {
            Authentication::Authorized { librarian_for, .. } if !librarian_for.is_empty() => Ok(librarian_for.clone()),
            Authentication::Authorized { .. } => Err(UE::YouShallNotPass),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_member(&self, required_member_id: AccountId) -> Result<(), UE> {
        match self {
            Authentication::Authorized { account_id, .. } if *account_id == required_member_id => Ok(()),
            Authentication::Authorized { .. } => Err(UE::YouShallNotPass),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_any_member(&self) -> Result<AccountId, UE> {
        match self {
            Authentication::Authorized { account_id, .. } => Ok(account_id.clone()),
            Authentication::Anonymous => Err(UE::AuthenticationRequired)
        }
    }

    pub fn requires_nothing(&self) -> Result<(), UE> { Ok(()) }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    /// Audience
    // pub aud: String,
    /// Expires at (UTC timestamp)
    pub exp: usize,
    /// Issued at (UTC timestamp)
    pub iat: usize,
    /// Issuer
    pub iss: String,
    /// Not Before (UTC timestamp)
    pub nbf: Option<usize>,
    /// Subject (the userid from keycloak)
    pub sub: String,
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
