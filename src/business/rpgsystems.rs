use crate::database::{Database, RpgSystem};
use crate::api::GetRpgSystems;
use crate::error::Error;

/// Get all RPG systems from database
pub fn get_rpgsystems(db: &Database) -> Result<GetRpgSystems, Error> {
    match db.get_all::<RpgSystem>() {
        Ok(rpgsystems) => Ok(GetRpgSystems { rpgsystems }),
        Err(e) => Err(e),
    }
}
