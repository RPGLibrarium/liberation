use std::fmt::Display;
use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::{header, StatusCode};
use diesel::r2d2::{PoolError};
use diesel::result::Error as DieselError;

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
    #[error("an internal server error occured")]
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
    #[error("data base connection failed")]
    DatabaseError(#[from] DieselError),
    #[error("getting a connection from the pool failed")]
    DatabasePoolingError(#[from] PoolError),
    #[error("could not find the app state during jwt checking")]
    MissingAppState,
}

/// actix uses this trait to decide on status codes.
/// see here for more information https://actix.rs/docs/errors/
impl ResponseError for UserFacingError {
    fn error_response(&self) -> HttpResponse {
        use actix_web::dev::HttpResponseBuilder;

        let mut response = HttpResponseBuilder::new(self.status_code());

        if let &UserFacingError::AuthenticationRequired = self {
            response.set_header(header::WWW_AUTHENTICATE, format!("Bearer realm=\"{}\"", "liberation")); //TODO: Use config for realm name
        }
        response
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        use UserFacingError as UE;

        match *self {
            UE::AuthenticationRequired => StatusCode::UNAUTHORIZED,
            UE::YouShallNotPass => StatusCode::FORBIDDEN,
            UE::BadToken => StatusCode::BAD_REQUEST,
            UE::NotFound => StatusCode::NOT_FOUND,
            UE::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
