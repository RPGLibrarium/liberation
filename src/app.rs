use crate::authentication::Authentication;
use crate::error::{InternalError, UserFacingError};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct AppState {
    pub database: DbPool,
    pub authenticator: Authentication,
    // pub live_users: LiveUsers,
}

impl AppState {
    pub fn new(
        database: DbPool,
        authenticator: Authentication, /*, live_users: LiveUsers*/
    ) -> Self {
        AppState {
            database,
            authenticator,
            // live_users,
        }
    }

    pub fn open_database_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, UserFacingError> {
        self.database
            .get()
            .map_err(|e| UserFacingError::Internal(InternalError::DatabasePoolingError(e)))
    }
}
