use crate::database::Database;
use crate::api::GetTitles;
use crate::error::Error;
use crate::model::Title;

/// Get all titles from model
pub fn get_titles(db: &Database) -> Result<GetTitles, Error> {
    Ok( GetTitles { titles: db.get_all::<Title>()? })
}
