use crate::database::Database;
use crate::error::Error;
use crate::api::GetBooks;
use crate::model::Book;


/// Get all books from model
pub fn get_books(db: &Database) -> Result<GetBooks, Error> {
    Ok( GetBooks { books: db.get_all::<Book>()? })
}
