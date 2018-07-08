use super::*;

use mysql;
pub type TitleId = Id;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Title {
    pub id: Option<TitleId>,
    pub name: String,
    pub system: RpgSystemId,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

pub fn get_titles_by_rpg_system(
    db: &Database,
    rpg_system_id: RpgSystemId,
) -> Result<Vec<Title>, Error> {
    Ok(db.pool
            .prep_exec(
                "select title_id, name, rpg_system_by_id, language, publisher, year, coverimage from titles where rpg_system_by_id=:rpg_system_id;",
                params!{
                    "rpg_system_id" => rpg_system_id,
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
                }).collect()
            })?
        )
}

pub fn count_books_by_title() {}

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

    fn insert(db: &Database, inp: &Title) -> Result<Title, Error> {
        check_varchar_length!(inp.name, inp.language, inp.publisher);
        Ok(db.pool.prep_exec("insert into titles (name, rpg_system_by_id, language, publisher, year, coverimage) values (:name, :system, :language, :publisher, :year, :coverimage)",
        params!{
            "name" => inp.name.clone(),
            "system" => inp.system,
            "language" => inp.language.clone(),
            "publisher" => inp.publisher.clone(),
            "year" => inp.year,
            "coverimage" => inp.coverimage.clone(),
        }).map(|result| {
            Title {
                id: Some(result.last_insert_id()),
                ..*inp
            }
        })?)
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
        Ok(db.pool
            .prep_exec(
                "delete from titles where title_id=:id",
                params!{
                    "id" => id,
                },
            )
            .map_err(|err| Error::DatabaseError(err))
            .and_then(|result| match result.affected_rows() {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(Error::IllegalState()),
            })?)
    }
}

#[cfg(test)]
mod tests {
    use database::test_util::*;
    use database::{Database, Error, DMO};

    #[test]
    fn insert_title_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(
                    String::from(TOO_LONG_STRING),
                    system.id,
                    String::from("de"),
                    String::from("??"),
                    1248,
                    None,
                )
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_language_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(
                    String::from("Kobolde"),
                    system.id,
                    String::from(TOO_LONG_STRING),
                    String::from("??"),
                    1248,
                    None,
                )
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_publisher_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(
                    String::from("Kobolde"),
                    system.id,
                    String::from("de"),
                    String::from(TOO_LONG_STRING),
                    1248,
                    None,
                )
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn insert_title_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            })
            .and_then(|title| {
                db.get_titles().and_then(|mut titles| {
                    Ok(titles
                        .pop()
                        .map_or(false, |fetched_title| title == fetched_title))
                })
            });
        teardown(dbname);
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
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None)
            })
            .and_then(|mut title| {
                title.name = _s(TOO_LONG_STRING);
                return db.update_title(&title);
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_language_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None)
            })
            .and_then(|mut title| {
                title.language = _s(TOO_LONG_STRING);
                return db.update_title(&title);
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_publisher_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None)
            })
            .and_then(|mut title| {
                title.publisher = _s(TOO_LONG_STRING);
                return db.update_title(&title);
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }

    #[test]
    fn update_title_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2142, None)
            })
            .and_then(|mut title| {
                title.name = _s("new name");
                title.year = 1999;
                title.publisher = _s("new publisher");
                db.update_title(&title).and_then(|_| {
                    db.get_titles().and_then(|mut titles| {
                        Ok(titles
                            .pop()
                            .map_or(false, |fetched_title| title == fetched_title))
                    })
                })
            });
        teardown(dbname);
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
