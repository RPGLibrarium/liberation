use crate::database::Database;
use crate::api::GetRpgSystems;
use crate::error::Error;
use crate::model::RpgSystem;

/// Get all RPG systems from model
pub fn get_rpgsystems(db: &Database) -> Result<GetRpgSystems, Error> {
    Ok(GetRpgSystems{ rpgsystems: db.get_all::<RpgSystem>()? })
}
