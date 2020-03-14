use crate::database::{Database, RpgSystem, Title, Guild, Book};
use crate::auth::Claims;
use crate::api::{GetBooks, BookWithTitleWithOwnerWithRental, TitleWithSystem};
use crate::error::Error;
use std::collections::HashMap;
use crate::database::dmo::DMO;
use crate::business::vec_to_map;

/// Get all books from database
pub fn get_books(db: &Database, claims: Option<Claims>) -> Result<GetBooks, Error> {
    //TODO: authentication

    //TODO Error mapping
    let books = Book::get_all(db)?;

    let systems = vec_to_map(RpgSystem::get_all(db)?);
    let titles = vec_to_map(Title::get_all(db)?);


    Ok(GetBooks {
        books: books
            .into_iter()
            .map(move |book| {
                let title = titles
                    .get(&book.title)
                    .cloned()
                    .expect("invalid book title");

                let system = systems
                    .get(&title.system)
                    .cloned()
                    .expect("titles should always have a system");

                let title_with_rpgsystem = TitleWithSystem {
                    id: title.id.expect("title must have an id"),
                    name: title.name,
                    system,
                    language: title.language,
                    publisher: title.publisher,
                    year: title.year,
                    coverimage: title.coverimage,
                    stock: None,
                    available: None,
                };

                BookWithTitleWithOwnerWithRental {
                    id: book.id.expect("book id shall not be empty"),
                    quality: book.quality,
                    external_inventory_id: book.external_inventory_id,
                    title: title_with_rpgsystem
                }
            })
            .collect(),
    })
}
