use std::io;
use thiserror::Error;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use actix_web::http::{header, StatusCode};
use config::ConfigError;
use diesel::r2d2::{PoolError};
use diesel::result::Error as DieselError;
use oauth2::RequestTokenError;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum UserFacingError {
    #[error("authentication required")]
    AuthenticationRequired,
    #[error("authentication was successful, but you shall not path.")]
    YouShallNotPass,
    #[error("your token smells bad, go away.")]
    BadToken,
    #[error("element was not found")]
    NotFound,
    #[error("element already exists")]
    AlreadyExists,
    #[error("invalid foreign key")]
    InvalidForeignKey,
    #[error("an internal server error occurred")]
    Internal(InternalError),
}

// impl Display for UserFacingError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         use UserFacingError as UE;
//
//         match *self {
//             UE::AuthenticationRequired => write!(f, "Authentication is needed for this operation."),
//             UE::YouShallNotPass => write!(f, "You shall not pass, you are not important enough for this operation."),
//             UE::NotFound => write!(f, "We couldn't find what you are looking for."),
//             UE::Internal(_) => write!(f, "It's not you. It's us. We probably did something stupid."),
//         }
//     }
// }

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
    KeycloakKeyHasBadFormat(#[from] base64::DecodeError),
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
            UE::BadToken => StatusCode::BAD_REQUEST,
            UE::NotFound => StatusCode::NOT_FOUND,
            UE::AlreadyExists => StatusCode::CONFLICT,
            UE::InvalidForeignKey => StatusCode::BAD_REQUEST,
            UE::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
