use mysql::Error as MySqlError;
use actix_web::{ResponseError, HttpResponse, http, error};
use std::fmt;
use failure::Fail;
type Field = String;

#[derive(Debug)]
pub enum Error {
    DatabaseError(MySqlError),
    ConstraintError(Option<Field>),
    DataTooLong(Field),
    IllegalValueForType(Field),
    JsonPayloadError(error::JsonPayloadError),
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

impl From<error::JsonPayloadError> for Error {
    fn from(error: error::JsonPayloadError) -> Self {
        Error::JsonPayloadError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            //TODO: Use Field when available
            Error::ConstraintError(_) => write!(f, "ERROR: unknown constaint error"),
            Error::DataTooLong(ref field) => write!(f, "ERROR: data too long for field: {}", field),
            Error::IllegalValueForType(ref field) => write!(f, "ERROR: illegal value in field: {}", field),
            Error::DatabaseError(ref err) => write!(f, "{{ {} }}", err),
            Error::JsonPayloadError(ref err) => write!(f, "{{ {} }}", err),
            _ => write!(f, "ERROR: unknown error")
        }
    }
}

impl Fail for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::DataTooLong(ref e) => HttpResponse::BadRequest().header("x-field", e.clone()).body(format!("{}", self)),
            Error::JsonPayloadError(ref e) => HttpResponse::BadRequest().body(format!("{}", self)),
            _ => HttpResponse::InternalServerError().finish()
        }

    }
}
