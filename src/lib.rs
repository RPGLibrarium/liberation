#![forbid(unsafe_code)]
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod error;
pub mod auth;
pub mod actions;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::auth::Authenticator;
use crate::error::{InternalError, UserFacingError};

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    pub database: DbPool,
    pub authenticator: Authenticator,
}

impl AppState {
    pub fn new(database: DbPool, authenticator: Authenticator) -> Self {
        AppState {
            database,
            authenticator,
        }
    }

    pub async fn update(&self) -> Result<(), InternalError> {
        Ok(())
    }
}

pub fn open_database_connection(app: &AppState) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, UserFacingError> {
    app.database.get()
        .map_err(|e| UserFacingError::Internal(InternalError::DatabasePoolingError(e)))
}

