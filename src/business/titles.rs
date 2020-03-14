use crate::database::{Database, Title, RpgSystem};
use crate::api::{GetTitles, TitleWithSystem};
use crate::error::Error;
use crate::database::dmo::DMO;
use crate::business::vec_to_map;

/// Get all titles from database
pub fn get_titles(db: &Database) -> Result<GetTitles, Error> {
    //TODO: authentication

    let titles = Title::get_all(db)?;
    let systems = vec_to_map(RpgSystem::get_all(db)?);


    Ok(GetTitles {
        titles: titles
            .into_iter()
            .map(|title| {
                let system = systems.get(&title.system)
                    .cloned()
                    .expect("title must have a system");

                TitleWithSystem{
                    id: title.id.expect("title must have an id"),
                    name: title.name,
                    system,
                    language: title.language,
                    publisher: title.publisher,
                    year: title.year,
                    coverimage: title.coverimage,
                    stock: None,
                    available: None
                }
            })
            .collect(),
    })
}
