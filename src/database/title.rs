use super::*;
use mysql;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use mysql::Value;

/// Id type for Title
pub type TitleId = Id;

/// Describes an abstract book. Copys of the book are stored in the Book type.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title {
    /// Id
    pub id: Option<TitleId>,
    /// Name/title of the book
    pub name: String,
    /// RPG System to which the book belongs
    pub system: RpgSystemId,
    /// Language
    pub language: String,
    /// Publisher
    pub publisher: String,
    /// Year of publishing
    pub year: Year,
    /// Cover image of book
    pub coverimage: Option<String>,
}

impl Title {
    /// Construct a new Title object with given parameters
    pub fn new(
        id: Option<TitleId>,
        name: String,
        system: RpgSystemId,
        language: String,
        publisher: String,
        year: Year,
        coverimage: Option<String>,
    ) -> Title {
        Title {
            id: id,
            name: name,
            system: system,
            language: language,
            publisher: publisher,
            year: year,
            coverimage: coverimage,
        }
    }
}

impl DMO for Title {
    type Id = TitleId;

    fn select_columns() -> Vec<&'static str> {
        vec!["name", "rpg_system_by_id", "language", "publisher", "year", "coverimage"]
    }

    fn id_column() -> &'static str {
        "title_id"
    }

    fn table_name() -> &'static str {
        "titles"
    }

    fn insert_params(&self) -> HashMap<String, Value, RandomState> {
        params!{
            "title_id" => self.id,
            "name" => self.name,
            "rpg_system_by_id" => self.system,
            "language" => self.language,
            "publisher" => self.publisher,
            "year" => self.year,
            "coverimage" => self.coverimage
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use database::test_util::*;
    use database::*;

    #[test]
    fn insert_title_name_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    String::from(TOO_LONG_STRING),
                    system_id,
                    String::from("de"),
                    String::from("??"),
                    1248,
                    None,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_language_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    String::from("Kobolde"),
                    system_id,
                    String::from(TOO_LONG_STRING),
                    String::from("??"),
                    1248,
                    None,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_publisher_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    String::from("Kobolde"),
                    system_id,
                    String::from("something else"),
                    String::from(TOO_LONG_STRING),
                    1248,
                    None,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                let mut orig_title = Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                );
                db.insert(&mut orig_title).and_then(|title_id| {
                    orig_title.id = Some(title_id);
                    Ok((title_id, orig_title))
                })
            })
            .and_then(|(title_id, orig_title)| {
                db.get(title_id).and_then(|rec_title| {
                    Ok(rec_title.map_or(false, |fetched_title| orig_title == fetched_title))
                })
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted title was not in DB :("),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_title_name_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                let mut orig_title = Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                );
                db.insert(&mut orig_title).and_then(|_| Ok(orig_title))
            })
            .and_then(|mut orig_title| {
                orig_title.name = _s(TOO_LONG_STRING);
                return db.update(&orig_title);
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_language_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                let mut orig_title = Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                );
                db.insert(&mut orig_title).and_then(|_| Ok(orig_title))
            })
            .and_then(|mut orig_title| {
                orig_title.language = _s(TOO_LONG_STRING);
                return db.update(&orig_title);
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_publisher_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, String::from("Kobolde"), None))
            .and_then(|system_id| {
                let mut orig_title = Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                );
                db.insert(&mut orig_title).and_then(|_| Ok(orig_title))
            })
            .and_then(|mut orig_title| {
                orig_title.publisher = _s(TOO_LONG_STRING);
                return db.update(&orig_title);
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, _s("Kobolde"), None))
            .and_then(|system_id| {
                let mut orig_title = Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2142,
                    None,
                );
                db.insert(&mut orig_title)
                    .and_then(|title_id| Ok((title_id, orig_title)))
            })
            .and_then(|(title_id, mut orig_title)| {
                orig_title.id = Some(title_id);
                orig_title.name = _s("new name");
                orig_title.year = 1999;
                orig_title.publisher = _s("new publisher");
                db.update(&orig_title).and_then(|_| {
                    db.get(title_id).and_then(|rec_title| {
                        Ok(rec_title.map_or(false, |fetched_title| orig_title == fetched_title))
                    })
                })
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated title to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    //TODO
    #[test]
    fn get_titles_by_rpg_system_correct() {}
}
*/
