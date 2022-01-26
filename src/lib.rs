#![forbid(unsafe_code)]
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod claims;
pub mod error;
mod keycloak;

use diesel::prelude::*;
use diesel::result::Error::NotFound;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use jsonwebtoken::DecodingKey;
use crate::claims::Authentication;
use crate::error::{InternalError, UserFacingError};
use crate::models::{NewRpgSystem, RpgSystem, Title};
use crate::schema::rpg_systems::dsl::rpg_systems;
use crate::schema::titles::dsl::titles;
use crate::UserFacingError::Internal;

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub database: DbPool,
    pub kc_public_key: DecodingKey,
}

pub fn list_rpg_systems(claims: Authentication, conn: &MysqlConnection) -> Result<Vec<RpgSystem>, UserFacingError> {
    claims.requires_nothing()
        .and_then(|()| rpg_systems
            .load(conn)
            .map_err(InternalError::DatabaseError)
            .map_err(Internal)
        )
}

pub fn get_rpg_systems(claims: Authentication, conn: &MysqlConnection, rpg_system_id: i32) -> Result<RpgSystem, UserFacingError> {
    claims.requires_nothing()
        .and_then(|()| rpg_systems
            .find(rpg_system_id)
            .first(conn)
            .map_err(|e|
                match e {
                    NotFound => UserFacingError::NotFound,
                    e => Internal(InternalError::DatabaseError(e))
                }
            )
        )
}

pub fn create_rpg_system<'a>(claims: Authentication, conn: &MysqlConnection, name: &'a str, shortname: &'a str) -> Result<(), UserFacingError> {
    use schema::rpg_systems;

    claims.requires_any_librarian()
        .and_then(|_| {
            let new_rpg_system = NewRpgSystem {
                name,
                shortname,
            };
            diesel::insert_into(rpg_systems::table)
                .values(&new_rpg_system)
                .execute(conn)
                .map_err(InternalError::DatabaseError)
                .map_err(Internal)
                .map(|_| ())
        })
}

pub fn list_titles(claims: Authentication, conn: &MysqlConnection) -> Result<Vec<Title>, UserFacingError> {
    claims.requires_nothing()
        .and_then(|()| titles
            .load(conn)
            .map_err(InternalError::DatabaseError)
            .map_err(Internal)
        )
}

pub fn create_title<'a>(conn: &MysqlConnection, name: &'a str, shortname: &'a str) {
    use schema::rpg_systems;

    let new_rpg_system = NewRpgSystem {
        name,
        shortname,
    };

    diesel::insert_into(rpg_systems::table)
        .values(&new_rpg_system)
        .execute(conn)
        .expect("Error saving new rpg_system.");
}
