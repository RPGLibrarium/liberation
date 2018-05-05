use mysql::Error as MySqlError;

type Field = String;

pub enum Error {
    DatabaseError(DatabaseError),
}

#[derive(Debug)]
pub enum DatabaseError {
    GenericError(MySqlError),
    FieldError(FieldError),
}

#[derive(Debug)]
pub enum FieldError {
    ConstraintError(Field),
    DataTooLong(Field),
    IlleagalValueForType(Field),
}

impl From<FieldError> for DatabaseError {
    fn from(error: FieldError) -> Self {
        DatabaseError::FieldError(error)
    }
}

impl From<MySqlError> for DatabaseError {
    fn from(error: MySqlError) -> Self {
        DatabaseError::GenericError(error)
    }
}
