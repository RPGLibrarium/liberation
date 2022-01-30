#![forbid(unsafe_code)]
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod claims;
pub mod error;
mod keycloak;
pub mod actions;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use jsonwebtoken::DecodingKey;
use crate::error::{InternalError, UserFacingError};

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub database: DbPool,
    pub kc_public_key: DecodingKey,
}

pub fn open_database_connection(app: &AppState) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, UserFacingError> {
    app.database.get()
        .map_err(|e| UserFacingError::Internal(InternalError::DatabasePoolingError(e)))
}
