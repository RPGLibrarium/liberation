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
    ConstraintError(Option<Field>),
    DataTooLong(Field),
    IllegalValueForType(Field),
}

impl From<FieldError> for DatabaseError {
    fn from(error: FieldError) -> Self {
        DatabaseError::FieldError(error)
    }
}

impl From<MySqlError> for DatabaseError {
    fn from(error: MySqlError) -> Self {

        match error {
            MySqlError::MySqlError(ref e) if e.code == 1452 => DatabaseError::FieldError(FieldError::ConstraintError(None)),
            /*MySqlError::MySqlError(e) => match e.code {
                1452 => DatabaseError::FieldError(FieldError::ConstraintError(None)),
                _ => DatabaseError::GenericError(MySqlError::MySqlError(e)),
            },*/
            _ => DatabaseError::GenericError(error),
        }
    }
}
