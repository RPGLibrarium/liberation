pub(crate) mod rpgsystems;
pub(crate) mod titles;
pub(crate) mod books;

use crate::api::*;
use crate::auth::{Claims, KeycloakCache};
use crate::database::*;
use crate::database::dmo::DMO;
use crate::error::Error;
use std::collections::HashMap;

pub fn vec_to_map<T : DMO>(vec: Vec<T>) -> HashMap<T::Id, T> {
    let mut map: HashMap<T::Id, T> = HashMap::new();
    for item in vec {
        match item.get_id() {
            Some(id) => {
                map.insert(id, item);
                ()
            }
            None => (),
        }
    }
    map
}

/*
/// Get an RPG system with given id from database
/// Fills the stock and availability infos when user is logged in.
pub fn get_rpgsystem(
    db: &Database,
    claims: Option<Claims>,
    system_id: RpgSystemId,
) -> Result<GetRpgSystem, Error> {
    let titles = db.get_titles_by_rpg_system(system_id)?;

    let include_stock = match (claims) {
        Some(_) => true,
        _ => false,
    };

    match db.get::<RpgSystem>(system_id) {
        Ok(Some(rpgsystem)) => Ok(GetRpgSystem::new(rpgsystem, titles, include_stock)),
        _ => Err(Error::ItemNotFound),
    }
}

/// Insert a RPG system into database
pub fn post_rpgsystem(
    db: &Database,
    claims: Option<Claims>,
    system: PutPostRpgSystem,
) -> Result<RpgSystemId, Error> {
    //TODO: Error handling
    //TODO: Assert Id is unset
    Ok(db.insert::<RpgSystem>(&system.rpgsystem)?)
}

/// Update a specific system in database
pub fn put_rpgsystem(
    db: &Database,
    claims: Option<Claims>,
    system: &PutPostRpgSystem,
) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<RpgSystem>(&system.rpgsystem)?)
}

/// Delete the RPG system with given id from database
pub fn delete_rpgsystem(
    db: &Database,
    claims: Option<Claims>,
    systemid: RpgSystemId,
) -> Result<(), Error> {
    match db.delete::<RpgSystem>(systemid) {
        Ok(true) => Ok(()),
        Ok(false) => Err(Error::ItemNotFound),
        Err(e) => Err(e),
    }
}
*/


/// Get a title with given id from database
pub fn get_title(
    db: &Database,
    claims: Option<Claims>,
    title_id: TitleId,
) -> Result<GetTitle, Error> {
    let (title, system, stock, available) = db.get_title_with_details(title_id)?.unwrap();
    let books = get_books_by_title_id(db, title_id, claims)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetTitle::new(title, system, stock, available, books))
}

/// Insert a title into database
pub fn post_title(
    db: &Database,
    claims: Option<Claims>,
    title: PutPostTitle,
) -> Result<TitleId, Error> {
    //TODO: Error handling
    Ok(db.insert::<Title>(&title.title)?)
}

/// Update a specific title in database
pub fn put_title(db: &Database, claims: Option<Claims>, title: PutPostTitle) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<Title>(&title.title)?)
}

/// Delete the title with given id from database
pub fn delete_title(db: &Database, claims: Option<Claims>, id: TitleId) -> Result<(), Error> {
    //TODO: Errorhandling
    db.delete::<Title>(id)?;
    Ok(())
}

/// Get all books of a title including rental information
fn get_books_by_title_id(
    db: &Database,
    id: TitleId,
    claims: Option<Claims>,
) -> Result<Vec<BookWithOwnerWithRental>, Error> {
    //TODO: Stub
    return Ok(vec![]);
}


pub fn get_book(db: &Database, claims: Option<Claims>, id: BookId) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(())
}

/*
pub fn post_book(
    db: &Database,
    claims: Option<Claims>,
    book: PutPostBook,
) -> Result<BookId, Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(1234)
}

pub fn put_book(db: &Database, claims: Option<Claims>, book: PutPostBook) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: authentication
    Ok(())
}

pub fn delete_book(db: &Database, claims: Option<Claims>, id: BookId) -> Result<(), Error> {
    //TODO:: Stub
    //TODO: Errorhandling
    db.delete::<Book>(id)?;
    Ok(())
}

pub fn get_members(db: &Database, claims: Option<Claims>) -> Result<GetMembers, Error> {
    //TODO: Stub
    //TODO: Get Members from Database
    //TODO: Complete Infos from Keycloak
    Ok(GetMembers { members: vec![] })
}

pub fn get_member(db: &Database, claims: Option<Claims>, id: MemberId) -> Result<(), Error> {
    //TODO: Stub
    //TODO: Get Members from Database
    //TODO: Complete Infos from Keycloak
    Ok(())
}

pub fn get_guilds(db: &Database, claims: Option<Claims>) -> Result<GetGuilds, Error> {
    //TODO: Stub
    Ok(GetGuilds { guilds: vec![] })
}

pub fn get_guild(db: &Database, claims: Option<Claims>, id: GuildId) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}

pub fn post_guild(
    db: &Database,
    claims: Option<Claims>,
    guild: PutPostGuild,
) -> Result<GuildId, Error> {
    //TODO: Stub
    Ok(1234)
}

pub fn put_guild(db: &Database, claims: Option<Claims>, guild: PutPostGuild) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}

pub fn delete_guild(db: &Database, claims: Option<Claims>, guild: GuildId) -> Result<(), Error> {
    //TODO: Stub
    Ok(())
}
*/
