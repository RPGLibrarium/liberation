use std::future::{Ready, ready};
use actix_web::{FromRequest, http, HttpRequest};
use actix_web::dev::Payload;
use jsonwebtoken::{Algorithm, decode, Validation};
use serde::{Serialize, Deserialize};
use crate::error::UserFacingError as Error;
use crate::error::UserFacingError::{AuthenticationRequired, YouShallNotPass};
use crate::{AppState, UserFacingError};
use crate::InternalError::MissingAppState;

type AccountId = u32;

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

    pub fn requires_aristocrat(&self) -> Result<(), Error> {
        match self {
            Authentication::Authorized { is_aristocrat: true, .. } => Ok(()),
            Authentication::Authorized { .. } => Err(YouShallNotPass),
            Authentication::Anonymous => Err(AuthenticationRequired)
        }
    }

    pub fn requires_librarian(&self, required_guild_id: AccountId) -> Result<(), Error> {
        match self {
            Authentication::Authorized { librarian_for, .. } if librarian_for.contains(&required_guild_id) => Ok(()),
            Authentication::Authorized { .. } => Err(YouShallNotPass),
            Authentication::Anonymous => Err(AuthenticationRequired)
        }
    }

    pub fn requires_any_librarian(&self) -> Result<Vec<AccountId>, Error> {
        match self {
            Authentication::Authorized { librarian_for, .. } if !librarian_for.is_empty() => Ok(librarian_for.clone()),
            Authentication::Authorized { .. } => Err(YouShallNotPass),
            Authentication::Anonymous => Err(AuthenticationRequired)
        }
    }

    pub fn requires_member(&self, required_member_id: AccountId) -> Result<(), Error> {
        match self {
            Authentication::Authorized { account_id, .. } if *account_id == required_member_id => Ok(()),
            Authentication::Authorized { .. } => Err(YouShallNotPass),
            Authentication::Anonymous => Err(AuthenticationRequired)
        }
    }

    pub fn requires_any_member(&self) -> Result<AccountId, Error> {
        match self {
            Authentication::Authorized { account_id, .. } => Ok(*account_id),
            Authentication::Anonymous => Err(AuthenticationRequired)
        }
    }

    pub fn requires_nothing(&self) -> Result<(), Error> { Ok(()) }
}

impl FromRequest for Authentication {
    type Error = UserFacingError;
    type Future = Ready<Result<Authentication, UserFacingError>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Futures are weird. Not sure if there is better way of doing this, but I would like to
        // make use of the early returns.
        fn check_token(req: &HttpRequest) -> Result<Authentication, UserFacingError> {
            if let Some(header) = req.headers().get("Authorization") {
                let raw_token = header.to_str().map_err(|_| UserFacingError::BadToken)?;

                let unvalidated = if raw_token.starts_with("Bearer ") {
                    raw_token.replacen("Bearer ", "", 1)
                } else {
                    return Err(UserFacingError::BadToken);
                };

                let decode_key = req.app_data::<AppState>()
                    .ok_or(Error::Internal(MissingAppState))?
                    .jwt_key();

                // TODO: require correct audience, subject, and issuer.
                let token = decode::<Claims>(
                    &unvalidated,
                    &decode_key,
                    &Validation::new(Algorithm::RS256),
                ).map_err(|_e| Error::BadToken)?; // TODO: log the error

                Ok(Authentication::authorized(
                    token.claims.roles.contains(&"aristocrat".to_string()),
                    todo!("librarian mapping not implemented"),
                    todo!("account id mapping not implemented"),
                ))
            } else { Ok(Authentication::Anonymous) }
        }
        ready(check_token(req))
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