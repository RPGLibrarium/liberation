use super::*;
use std::string::String;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use mysql::{Value, FromRowError, Row};

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

impl DMO for RpgSystem {
    type Id = RpgSystemId;

    fn select_columns() -> Vec<&'static str> {
        vec!["name", "shortname"]
    }

    fn id_column() -> &'static str {
        "rpg_system_id"
    }

    fn table_name() -> &'static str {
        "rpg_systems"
    }

    fn insert_params(&self) -> Vec<(String, Value)> {
        params! {
            "rpg_system_id" => self.id,
            "name" => self.name,
            "shortname" => self.shortname
        }
    }

    fn from_row(row: Row) -> Result<Self, Error> {
        let (id, name, shortname) = mysql::from_row(row.clone());
        Ok(
            RpgSystem {
                id,
                name,
                shortname,
            })
    }
}

/*
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
*/
