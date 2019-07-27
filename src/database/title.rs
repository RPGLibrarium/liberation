use super::*;
use mysql;

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
    fn get_all(db: &Database) -> Result<Vec<Title>, Error> {
        Ok(db.pool.prep_exec("select title_id, name, rpg_system_by_id, language, publisher, year, coverimage from titles;",())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, system, language, publisher, year, coverimage) = mysql::from_row(row);
            Title {
                id: id,
                name: name,
                system: system,
                language: language,
                publisher: publisher,
                year: year,
                coverimage: coverimage,
            }
        }).collect()
    })?)
    }

    //TODO: Test
    fn get(db: &Database, title_id: TitleId) -> Result<Option<Title>, Error> {
        let mut results = db.pool
        .prep_exec(
            "select title_id, name, rpg_system_by_id, language, publisher, year, coverimage from titles where title_id=:title_id;",
            params!{
                "title_id" => title_id,
            },
        )
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, system, language, publisher, year, coverimage) = mysql::from_row(row);
                Title {
                    id: id,
                    name: name,
                    system: system,
                    language: language,
                    publisher: publisher,
                    year: year,
                    coverimage: coverimage,
                }
            }).collect::<Vec<Title>>()
        })?;
        return Ok(results.pop());
    }

    //TODO Test

    fn insert(db: &Database, inp: &Title) -> Result<TitleId, Error> {
        check_varchar_length!(inp.name, inp.language, inp.publisher);
        Ok(db.pool.prep_exec("insert into titles (name, rpg_system_by_id, language, publisher, year, coverimage) values (:name, :system, :language, :publisher, :year, :coverimage)",
        params!{
            "name" => inp.name.clone(),
            "system" => inp.system,
            "language" => inp.language.clone(),
            "publisher" => inp.publisher.clone(),
            "year" => inp.year,
            "coverimage" => inp.coverimage.clone(),
        }).map(|result| result.last_insert_id())?)
    }

    fn update(db: &Database, title: &Title) -> Result<(), Error> {
        check_varchar_length!(title.name, title.language, title.publisher);
        Ok(db.pool.prep_exec("update titles set name=:name, rpg_system_by_id=:system, language=:language, publisher=:publisher, year=:year, coverimage=:coverimage where title_id=:id;",
        params!{
            "name" => title.name.clone(),
            "system" => title.system,
            "language" => title.language.clone(),
            "publisher" => title.publisher.clone(),
            "year" => title.year,
            "coverimage" => title.coverimage.clone(),
            "id" => title.id,
        }).and(Ok(()))?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from titles where title_id=:id",
                params! {
                    "id" => id,
                },
            )
            .map_err(|err| Error::DatabaseError(err))
            .and_then(|result| match result.affected_rows() {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(Error::IllegalState),
            })?)
    }
}

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
