use super::*;

pub type RpgSystemId = Id;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct RpgSystem {
    pub id: Option<RpgSystemId>,
    pub name: String,
}

impl RpgSystem {
    pub fn new(id: Option<RpgSystemId>, name: String) -> RpgSystem {
        RpgSystem { id: id, name: name }
    }
}

impl DMO for RpgSystem {
    type Id = RpgSystemId;
    fn insert(db: &Database, inp: &RpgSystem) -> Result<RpgSystem, Error> {
        check_varchar_length!(inp.name);
        Ok(db.pool
            .prep_exec(
                "insert into rpg_systems (name) values (:name)",
                params!{
                    "name" => inp.name.clone(),
                },
            )
            .map(|result| RpgSystem {
                id: Some(result.last_insert_id()),
                ..*inp
            })?)
    }

    fn get_all(db: &Database) -> Result<Vec<RpgSystem>, Error> {
        Ok(db.pool
            .prep_exec("select rpg_system_id, name from rpg_systems;", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name) = mysql::from_row(row);
                        RpgSystem { id: id, name: name }
                    })
                    .collect()
            })?)
    }

    //TODO: Test
    fn get(db: &Database, rpg_system_id: Id) -> Result<Option<RpgSystem>, Error> {
        let mut results = db.pool
            .prep_exec(
                "select rpg_system_id, name from rpg_systems where rpg_system_id=:rpg_system_id;",
                params!{
                    "rpg_system_id" => rpg_system_id,
                },
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name) = mysql::from_row(row);
                        RpgSystem { id: id, name: name }
                    })
                    .collect::<Vec<RpgSystem>>()
            })?;
        return Ok(results.pop());
    }

    fn update(db: &Database, rpgsystem: &RpgSystem) -> Result<(), Error> {
        check_varchar_length!(rpgsystem.name);
        Ok(db.pool
            .prep_exec(
                "update rpg_systems set name=:name where rpg_system_id=:id;",
                params!{
                    "name" => rpgsystem.name.clone(),
                    "id" => rpgsystem.id,
                },
            )
            .map(|_| ())?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db.pool
            .prep_exec(
                "delete from rpg_systems where rpg_system_id=:id",
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
    use database::RpgSystem;
    use database::{Database, Error, DMO};

    #[test]
    fn insert_rpg_system_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let system_in = RpgSystem::insert(
            &db,
            RpgSystem {
                id: None,
                name: _s("SR5👿"),
            },
        );
        let system_in = db.insert_rpg_system().unwrap();
        let system_out = db.get_rpg_systems().unwrap().pop().unwrap();
        assert_eq!(system_in, system_out);
        teardown(dbname);
    }

    #[test]
    fn insert_rpg_system_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_rpg_system(String::from(TOO_LONG_STRING));
        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    #[test]
    fn update_rpg_system_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_rpg_system(_s("SR5👿")).and_then(|mut system| {
            system.name = _s("SR5");
            db.update_rpg_system(&system).and_then(|_| {
                db.get_rpg_systems().and_then(|mut systems| {
                    Ok(systems
                        .pop()
                        .map_or(false, |fetched_system| system == fetched_system))
                })
            })
        });

        teardown(dbname);

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
    fn update_rpg_system_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_rpg_system(String::from("SR5👿"))
            .and_then(|mut rpgsystem| {
                rpgsystem.name = String::from(TOO_LONG_STRING);
                return db.update_rpg_system(&rpgsystem);
            });

        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"rpgsystem.name\")"
            ),
        }
    }

    #[test]
    fn get_rpg_system_by_id_correct() {}
}
