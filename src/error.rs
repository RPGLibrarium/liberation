use std::io;
use thiserror::Error;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use actix_web::http::{header, StatusCode};
use config::ConfigError;
use diesel::r2d2::{PoolError};
use diesel::result::Error as DieselError;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum UserFacingError {
    #[error("authentication required")]
    AuthenticationRequired,
    #[error("authentication was successful, but you shall not pass.")]
    YouShallNotPass,
    #[error("you are not registered.")]
    NoRegistered,
    #[error("your token smells bad, go away.")]
    BadToken(String),
    #[error("element was not found")]
    NotFound,
    #[error("deactivated")]
    Deactivated,
    #[error("element already exists")]
    AlreadyExists,
    #[error("invalid foreign key")]
    InvalidForeignKey,
    #[error("an internal server error occurred")]
    Internal(InternalError),
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("config is invalid")]
    ConfigError(#[from] ConfigError),
    #[error("data base connection failed")]
    DatabaseError(#[from] DieselError),
    #[error("getting a connection from the pool failed")]
    DatabasePoolingError(#[from] PoolError),
    #[error("could not find the app state during jwt checking")]
    MissingAppState,
    #[error("io error")]
    IOError(#[from] io::Error),
    #[error("join error")]
    JoinError(#[from] JoinError),
    #[error("keycloak is not reachable")]
    KeycloakNotReachable(#[from] reqwest::Error),
    #[error("keycloak returned a bad key")]
    KeycloakKeyHasBadFormat(#[from] jsonwebtoken::errors::Error),
    #[error("authenticating with keycloak failed")]
    KeycloakAuthenticationFailed(#[from] Box<dyn std::error::Error>),
}

/// actix uses this trait to decide on status codes.
/// see here for more information https://actix.rs/docs/errors/
// My IntelliJ thinks this is wrong, but it's not. Display is provided by thiserror.
impl ResponseError for UserFacingError {
    fn error_response(&self) -> HttpResponse {
        let mut response = HttpResponseBuilder::new(self.status_code());

        if let &UserFacingError::AuthenticationRequired = self {
            response.insert_header((header::WWW_AUTHENTICATE, format!("Bearer realm=\"{}\"", "liberation"))); //TODO: Use config for realm name
        }
        response
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        use UserFacingError as UE;

        match *self {
            UE::AuthenticationRequired => StatusCode::UNAUTHORIZED,
            UE::YouShallNotPass => StatusCode::FORBIDDEN,
            UE::NoRegistered => StatusCode::FORBIDDEN,
            UE::BadToken(_) => StatusCode::BAD_REQUEST,
            UE::NotFound => StatusCode::NOT_FOUND,
            UE::Deactivated => StatusCode::FORBIDDEN,
            UE::AlreadyExists => StatusCode::CONFLICT,
            UE::InvalidForeignKey => StatusCode::BAD_REQUEST,
            UE::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
