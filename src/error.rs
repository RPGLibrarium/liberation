use actix_web::client::SendRequestError;
use actix_web::{error, HttpResponse, ResponseError};
use awc;
use core::num::ParseIntError;
use failure::Fail;
use mysql::{Error as MySqlError, FromRowError, FromValueError};
use oauth2::basic::BasicErrorResponseType;
use oauth2::RequestTokenError;
use std::fmt;
//use std::option::NoneError;

type Field = String;

#[derive(Debug)]
/// An custom error type, that handles convertion to HTTP error codes
pub enum Error {
    // Database related errors
    /// Internal Database Errors -> 500
    DatabaseError(MySqlError),
    /// Database is inconsistent -> 500
    IllegalState(&'static str),
    /// Conversion done by mysql failed
    MySqlRowConversionError(FromRowError),
    MySqlValueConversionError(FromValueError),
    // User input related errors
    /// Database Constraints, usually from invalid User input -> 400 or 500
    ConstraintError(Option<Field>),
    /// User input is too long -> 400
    DataTooLong(Field),
    /// User input has wrong type -> 400
    IllegalValueForType(Field),
    /// Invalid Json from user -> 400
    JsonPayloadError(actix_web::error::JsonPayloadError),
    /// Backend can not authenticate with the Keycloak server-> 500
    //KeycloakAuthenticationError(Box<RequestTokenError<dyn Fail, BasicErrorResponseType>>),
    /// No connection to Keycloak server -> 500
    KeycloakConnectionError(SendRequestError),
    /// Keycload answer wrong -> 500
    KeycloakJsonError(awc::error::JsonPayloadError),
    /// Authentication Token is invalid -> 401
    InvalidAuthenticationError,
    /// Missing a required claim -> 403
    YouShallNotPassError,
    /// Not even logged in -> 401
    SpeakFriendAndEnterError,
    /// Missing parameter in URL -> 400
    BadRequestFormat,
    ///
    ActixError(error::Error),
    /// No item with given id found -> 404
    ItemNotFound,
    /// Conversion from data to Struct failed
    EnumFromStringError(String)
}

impl From<MySqlError> for Error {
    fn from(error: MySqlError) -> Self {
        match error {
            MySqlError::MySqlError(ref e) if e.code == 1452 => Error::ConstraintError(None),
            /*MySqlError::MySqlError(e) => match e.code {
                1452 => DatabaseError::FieldError(FieldError::ConstraintError(None)),
                _ => DatabaseError::GenericError(MySqlError::MySqlError(e)),
            },*/
            _ => Error::DatabaseError(error),
        }
    }
}

//impl From<NoneError> for Error {
//    fn from(error: NoneError) -> Self {
//        Error::BadRequestFormat
//    }
//}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::BadRequestFormat
    }
}

impl From<actix_web::error::JsonPayloadError> for Error {
    fn from(error: actix_web::error::JsonPayloadError) -> Self {
        Error::JsonPayloadError(error)
    }
}

impl From<mysql::FromRowError> for Error{
    fn from(error: mysql::FromRowError) -> Self {
        Error::MySqlRowConversionError(error)
    }
}

impl From<mysql::FromValueError> for Error{
    fn from(error: mysql::FromValueError) -> Self {
        Error::MySqlValueConversionError(error)
    }
}
/*
impl From<RequestTokenError<Fail, BasicErrorResponseType>> for Error {
    fn from(error: RequestTokenError<dyn Fail, BasicErrorResponseType>) -> Self {
        Error::KeycloakAuthenticationError(Box::new(error))
    }
}
*/

// impl From<error::Error> for Error {
//     fn from(error: error::Error) -> Self {
//         Error::ActixError(error)
//     }
// }

/*
impl From<error::InternalError<ParseIntError>> for Error {
    fn from(error: error::InternalError<ParseIntError>) -> Self {
        Error::ActixInternalError(error)
    }
}
*/

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            //TODO: Use Field when available
            Error::ConstraintError(_) => write!(f, "ERROR: unknown constaint error"),
            Error::DataTooLong(ref field) => write!(f, "ERROR: data too long for field: {}", field),
            Error::IllegalValueForType(ref field) => {
                write!(f, "ERROR: illegal value in field: {}", field)
            }
            Error::DatabaseError(ref err) => write!(f, "{{ {} }}", err),
            Error::JsonPayloadError(ref err) => write!(f, "{{ {} }}", err),
            Error::IllegalState(ref msg) => write!(f, "ERROR: Invalid State: {}", msg),
            Error::MySqlRowConversionError(ref rowError) => write!(f, "{{ {} }}", rowError),
            Error::MySqlValueConversionError(ref valueError) => write!(f, "{{ {} }}", valueError),
            Error::EnumFromStringError(ref msg) => write!(f, "ERROR: Creating Enum from String failed with message: {}", msg ),
            //Error::KeycloakAuthenticationError(ref err) => write!(f, "{{ {} }}", err),
            // Error::ActixError(ref err) => write!(f, "{{ {} }}", err),
            _ => write!(f, "ERROR: unknown error"),
        }
    }
}

//impl Fail for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::DataTooLong(ref e) => HttpResponse::BadRequest()
                .header("x-field", e.clone())
                .body(format!("{}", self)),
            Error::InvalidAuthenticationError => HttpResponse::Unauthorized()
                .header(
                    "WWW-Authenticate",
                    format!("Bearer realm=\"{}\"", "liberation"), //TODO: Use config for realm name
                )
                .finish(),
            Error::YouShallNotPassError => HttpResponse::Forbidden().finish(),
            Error::SpeakFriendAndEnterError => HttpResponse::Unauthorized().finish(),
            //_ => HttpResponse::InternalServerError().finish(), TODO: Debugging option
            // Error::ActixError(err) => err.as_response_error().error_response(),
            // Error::ActixInternalError(err) => err.error_response(),
            _ => {
                error!("Internal Server Error: {:?}", self);
                HttpResponse::InternalServerError().body("")
            },
        }
    }
}
