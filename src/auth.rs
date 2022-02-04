use jsonwebtoken::DecodingKey;
use log::{debug, info};
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures::future::{LocalBoxFuture};
use futures::FutureExt;
use jsonwebtoken::{Algorithm, decode, Validation};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use crate::AppState;
use crate::error::UserFacingError as UE;
use crate::InternalError::MissingAppState;

type AccountId = u32;

pub enum Authenticator {
    KeycloakLive {
        endpoint: String,
        public_key: Mutex<DecodingKey>,
    },
    OauthStatic { public_key: DecodingKey },
}

impl Authenticator {
    pub async fn update(&self) {
        match &self {
            Authenticator::KeycloakLive { endpoint, public_key } => {
                info!("Using rotating keys from keycloak.");
                let key = todo!("not implemented");
                let mut lock = public_key.lock().await;
                debug!("Setting new key in shared mutable state.");
                *lock = key;
            }
            Authenticator::OauthStatic { .. } => debug!("Avoiding regeneration of public key.")
        }
    }

    pub async fn verify_token(&self, token: &str) -> Result<Authentication, UE> {
        let key = match &self {
            Authenticator::KeycloakLive { public_key, .. } => (*public_key.lock().await).clone(),
            Authenticator::OauthStatic { public_key } => public_key.clone(),
        };

        // TODO: require correct audience, subject, and issuer.
        let validated = decode::<Claims>(&token, &key, &Validation::new(Algorithm::RS256))
            .map_err(|_e| UE::BadToken)?; // TODO: log the error

        Ok(Authentication::authorized(
            validated.claims.roles.contains(&"aristocrat".to_string()),
            todo!("librarian mapping not implemented"),
            todo!("account id mapping not implemented"),
        ))
    }
}

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
        return Authentication::Authorized {
            is_aristocrat,
            librarian_for,
            account_id,
        };
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
            Authentication::Authorized { account_id, .. } => Ok(*account_id),
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
                let raw_token = header.to_str().map_err(|_| UE::BadToken)?;

                let unvalidated = if raw_token.starts_with("Bearer ") {
                    raw_token.replacen("Bearer ", "", 1)
                } else {
                    return Err(UE::BadToken);
                };

                let authenticator = &req.app_data::<AppState>()
                    .ok_or(UE::Internal(MissingAppState))?
                    .authenticator;

                authenticator.verify_token(&unvalidated).await
            } else { Ok(Authentication::Anonymous) }
        }.boxed_local()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub aud: String,
    // Optional. Audience
    pub exp: usize,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,
    // Optional. Issued at (as UTC timestamp)
    pub iss: String,
    // Optional. Issuer
    pub nbf: usize,
    // Optional. Not Before (as UTC timestamp)
    pub sub: String,
    // Optional. Subject (whom token refers to)
    pub roles: Vec<String>,
    pub name: String,
    pub email: String,
    // ... whatever!
}
