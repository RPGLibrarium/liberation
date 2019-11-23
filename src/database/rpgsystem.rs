use super::*;
use std::string::String;
use serde::{Serialize, Deserialize};
use mysql::params;

/// Id type for RpgSystem
pub type RpgSystemId = Id;

/// An RPG System
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RpgSystem {
    /// Id
    pub id: Option<RpgSystemId>,
    /// Name of RPG System
    pub name: String,
    /// Common abbreviation of the system name, e.g. D&D
    pub shortname: Option<String>,
}

impl RpgSystem {
    /// Construct a new RpgSystem object with given parameters
    pub fn new(id: Option<RpgSystemId>, name: String, shortname: Option<String>) -> RpgSystem {
        RpgSystem {
            id: id,
            name: name,
            shortname: shortname,
        }
    }
}

impl DMO for RpgSystem {
    type Id = RpgSystemId;
    fn insert(db: &Database, inp: &RpgSystem) -> Result<RpgSystemId, Error> {
        check_varchar_length!(inp.name);
        Ok(db
            .pool
            .prep_exec(
                "insert into rpg_systems (name, shortname) values (:name, :shortname)",
                params! {
                    "name" => inp.name.clone(),
                    "shortname" => inp.shortname.clone()
                },
            )
            .map(|result| result.last_insert_id())?)
    }

    fn get_all(db: &Database) -> Result<Vec<RpgSystem>, Error> {
        Ok(db
            .pool
            .prep_exec(
                "select rpg_system_id, name, shortname from rpg_systems;",
                (),
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name, short): (Option<RpgSystemId>, String, Option<String>) =
                            mysql::from_row(row);
                        RpgSystem {
                            id: id,
                            name: name,
                            shortname: short,
                        }
                    })
                    .collect()
            })?)
    }

    //TODO: Test
    fn get(db: &Database, rpg_system_id: Id) -> Result<Option<RpgSystem>, Error> {
        let mut results = db.pool
            .prep_exec(
                "select rpg_system_id, name, shortname from rpg_systems where rpg_system_id=:rpg_system_id;",
                params!{
                    "rpg_system_id" => rpg_system_id,
                },
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name, short) : (Option<RpgSystemId>, String, Option<String>) = mysql::from_row(row);
                        RpgSystem { id: id, name: name, shortname: short }
                    })
                    .collect::<Vec<RpgSystem>>()
            })?;
        return Ok(results.pop());
    }

    fn update(db: &Database, rpgsystem: &RpgSystem) -> Result<(), Error> {
        check_varchar_length!(rpgsystem.name);
        /*match rpgsystem.shortname {
            None => (),
            Some(short) => check_varchar_length!(short)
        }*/

        Ok(db
            .pool
            .prep_exec(
                "update rpg_systems set name=:name, shortname=:short where rpg_system_id=:id;",
                params! {
                    "name" => rpgsystem.name.clone(),
                    "short" => rpgsystem.shortname.clone(),
                    "id" => rpgsystem.id,
                },
            )
            .map(|_| ())?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from rpg_systems where rpg_system_id=:id",
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
    use database::RpgSystem;
    use database::{Database, Error};

    #[test]
    fn insert_rpg_system_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let mut system_in = RpgSystem::new(None, _s("Shadowrun 5"), Some(_s("SR5👿")));

        let result = db
            .insert(&system_in)
            .and_then(|id| Ok((id, db.get::<RpgSystem>(id)?)));

        teardown(settings);
        let (new_id, system_out) = result.unwrap();
        system_in.id = Some(new_id);
        assert_eq!(system_in, system_out.unwrap());
    }

    #[test]
    fn insert_rpg_system_no_shortname_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let mut system_in = RpgSystem::new(None, _s("Shadowrun 5"), None);

        let result = db
            .insert(&system_in)
            .and_then(|id| Ok((id, db.get::<RpgSystem>(id)?)));

        teardown(settings);
        let (new_id, system_out) = result.unwrap();
        system_in.id = Some(new_id);
        assert_eq!(system_in, system_out.unwrap());
    }

    #[test]
    fn insert_rpg_system_name_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db.insert(&mut RpgSystem::new(
            None,
            String::from(TOO_LONG_STRING),
            None,
        ));
        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    /*#[test]
    fn insert_rpg_system_shortname_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db.insert(&mut RpgSystem::new(None, String::from("Kobolde"), Some(String::from(TOO_LONG_STRING))));
        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"shortname\")"),
        }
    }*/

    #[test]
    fn update_rpg_system_name_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let mut system_in = RpgSystem::new(None, _s("Shadowrunn 5"), None);
        let result = db.insert(&mut system_in).and_then(|id| {
            system_in.id = Some(id);
            system_in.name = _s("Shadowrun 5");
            system_in.shortname = Some(_s("SR5"));
            db.update(&system_in).and_then(|_| {
                db.get::<RpgSystem>(id).and_then(|recovered| {
                    Ok(recovered.map_or(false, |fetched_system| system_in == fetched_system))
                })
            })
        });

        teardown(settings);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated rpgsystem to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_rpg_system_shortname_null() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let mut system_in = RpgSystem::new(None, _s("Shadowrun 5"), None);
        let result = db.insert(&mut system_in).and_then(|id| {
            system_in.name = _s("SR5");
            db.update(&system_in).and_then(|_| {
                db.get::<RpgSystem>(id).and_then(|recovered| {
                    Ok(recovered.map_or(false, |fetched_system| fetched_system.shortname == None))
                })
            })
        });

        teardown(settings);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated rpgsystem shortname to be None after retrieval"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_rpg_system_name_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let mut system_in = RpgSystem::new(None, _s("Shadowrun 5"), Some(_s("SR5👿")));
        let result = db.insert(&mut system_in).and_then(|_| {
            system_in.name = String::from(TOO_LONG_STRING);
            db.update(&system_in)
        });

        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"rpgsystem.name\")"
            ),
        }
    }

    //TODO
    #[test]
    fn get_rpg_system_by_id_correct() {}
}
