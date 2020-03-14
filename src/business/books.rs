use crate::database::{Database, RpgSystem, Title, Guild, Book};
use crate::auth::Claims;
use crate::api::GetBooks;
use crate::error::Error;
use std::collections::HashMap;

/// Get all books from database
pub fn get_books(db: &Database, claims: Option<Claims>) -> Result<GetBooks, Error> {
    //TODO: authentication

    //TODO Error mapping
    let books = Book::get_all(db)?;
    let systems_vec = RpgSystem::get_all(db)?;
    let titles_vec = Title::get_all(db)?;
    let guilds_vec = Guild::get_all(db)?;

    let mut systems_map: HashMap<RpgSystemId, RpgSystem> = HashMap::new();
    for system in systems_vec {
        match system.id {
            Some(id) => {
                systems_map.insert(id, system);
                ()
            }
            None => (),
        }
    }
    let mut titles_map: HashMap<TitleId, TitleWithSystem> = HashMap::new();
    for title in titles_vec {
        match title.id {
            Some(id) => {
                match systems_map.get(&title.system).cloned() {
                    Some(system) => {
                        titles_map.insert(id, TitleWithSystem::new(title, system, 0, 0));
                        ()
                    }
                    None => (),
                }
                ()
            }
            None => (),
        }
    }
    let mut guilds_map: HashMap<GuildId, Guild> = HashMap::new();
    for guild in guilds_vec {
        match guild.id {
            Some(id) => {
                guilds_map.insert(id, guild);
                ()
            }
            None => (),
        }
    }
    let titles_map = titles_map;
    let guilds_map = guilds_map;

    let available = false; //TODO calculate from data
    let rental = None; //TODO:

    Ok(GetBooks {
        books: books
            .into_iter()
            .map(move | book | {
                BookWithTitleWithOwnerWithRental {
                    id: book.id.expect("book id shall not be empty"),
                    quality: book.quality,
                    available,
                    external_inventory_id: book.external_inventory_id,
                    rental: match rental {
                        None => None,
                        Some(r) => Some(RentalWithRentee {
                            from: r.from,
                            to: r.to,
                            rentee: Entity {
                                entity_type: r.rentee_type.clone(),
                                id: r.rentee,
                                name: match r.rentee_type {
                                    EntityType::Guild => guilds_map
                                        .get(&r.rentee)
                                        .expect("invalid guild id")
                                        .name
                                        .clone(),
                                    EntityType::Member => String::from("NO DATA"), // TODO use keycloak
                                },
                            },
                        }),
                    },
                    title: titles_map
                        .get(&book.title)
                        .cloned()
                        .expect("invalid book title"),
                    owner: Entity {
                        entity_type: book.owner_type.clone(),
                        id: book.owner,
                        name: match book.owner_type {
                            EntityType::Guild => guilds_map
                                .get(&book.owner)
                                .expect("invalid guild id")
                                .name
                                .clone(),
                            EntityType::Member => String::from("NO DATA"), // TODO use keycloak
                        },
                    },
                }
            })
            .collect(),
    })
}
