use api::*;
use auth::KeycloakCache;
use auth::Token;
use database::*;
use std::collections::HashMap;

pub fn get_rpgsystems(
    db: &Database,
    kc: &KeycloakCache,
    token: Token,
) -> Result<GetRpgSystems, Error> {
    //TODO Error mapping
    let rpgsystems = db.get_all::<RpgSystem>()?;
    Ok(GetRpgSystems { rpgsystems })
}

pub fn get_rpgsystem(
    db: &Database,
    token: Token,
    system_id: RpgSystemId,
) -> Result<GetRpgSystem, Error> {
    //TODO: Handle None()
    let system = db.get::<RpgSystem>(system_id)?.unwrap();
    let titles = db.get_titles_by_rpg_system(system_id)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetRpgSystem::new(system, titles))
}

pub fn post_rpgsystem(
    db: &Database,
    token: Token,
    system: &mut PutPostRpgSystem,
) -> Result<RpgSystemId, Error> {
    //TODO: Error handling
    //TODO: Assert Id is unset
    Ok(db.insert::<RpgSystem>(&mut system.rpgsystem)?)
}

pub fn put_rpgsystem(db: &Database, token: Token, system: &PutPostRpgSystem) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<RpgSystem>(&system.rpgsystem)?)
}

pub fn delete_rpgsystem(db: &Database, token: Token, systemid: RpgSystemId) -> Result<(), Error> {
    //TODO: Errorhandling, 404
    db.delete::<RpgSystem>(systemid)?;
    Ok(())
}

pub fn get_titles(db: &Database, token: Token) -> Result<GetTitles, Error> {
    //TODO: authentication

    //TODO Error mapping
    let tuples = db.get_titles_with_details()?;

    Ok(GetTitles {
        titles: tuples
            .into_iter()
            .map(|(title, system, stock, available)| {
                TitleWithSystem::new(title, system, stock, available)
            })
            .collect(),
    })
}

pub fn get_title(db: &Database, token: Token, title_id: TitleId) -> Result<GetTitle, Error> {
    let (title, system, stock, available) = db.get_title_with_details(title_id)?.unwrap();
    let books = get_books_by_title_id(db, title_id)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetTitle::new(title, system, stock, available, books))
}

pub fn post_title(db: &Database, token: Token, title: &mut PutPostTitle) -> Result<TitleId, Error> {
    //TODO: Error handling
    Ok(db.insert::<Title>(&mut title.title)?)
}

pub fn put_title(db: &Database, token: Token, title: &PutPostTitle) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<Title>(&title.title)?)
}

pub fn delete_title(db: &Database, titleid: TitleId) -> Result<(), Error> {
    //TODO: Errorhandling
    db.delete::<Title>(titleid)?;
    Ok(())
}

//TODO: Stub
fn get_books_by_title_id(
    db: &Database,
    titleid: TitleId,
) -> Result<Vec<BookWithOwnerWithRental>, Error> {
    return Ok(vec![]);
}

pub fn get_books(db: &Database, token: Token) -> Result<GetBooks, Error> {
    //TODO: authentication

    //TODO Error mapping
    let books = db.get_books_with_details()?;
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

    Ok(GetBooks {
        books: books
            .into_iter()
            .map(move |(book, rental, available)| {
                BookWithTitleWithOwnerWithRental {
                    id: book.id.expect("book id shall not be empty"),
                    quality: book.quality,
                    available: available,
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
