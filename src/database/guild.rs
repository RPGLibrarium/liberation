use super::*;

pub type GuildId = EntityId;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub address: String,
    pub contact: MemberId,
}

impl DMO for Guild {
    fn insert(
        &db: &Database,
        name: String,
        address: String,
        contact: MemberId,
    ) -> Result<Guild, Error> {
        check_varchar_length!(name, address);
        Ok(db.pool.prep_exec("insert into guilds (name, address, contact_by_member_id) values (:name, :address, :contact)",
        params!{
            "name" => name.clone(),
            "address" => address.clone(),
            "contact" => contact,
        }).map(|result| {
            Guild {
                id: result.last_insert_id(),
                name: name,
                address: address,
                contact: contact,
            }
        })?)
    }

    fn get_all(&db: &Database) -> Result<Vec<Guild>, Error> {
        Ok(db.pool
            .prep_exec(
                "select guild_id, name, address, contact_by_member_id from guilds;",
                (),
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name, address, contact) = mysql::from_row(row);
                        Guild {
                            id: id,
                            name: name,
                            address: address,
                            contact: contact,
                        }
                    })
                    .collect()
            })?)
    }

    fn update(&db: &Database, guild: &Guild) -> Result<(), Error> {
        check_varchar_length!(guild.name, guild.address);
        Ok(db.pool.prep_exec("update guilds set name=:name, address=:address, contact_by_member_id=:contact where guild_id=:id",
        params!{
            "name" => guild.name.clone(),
            "address" => guild.address.clone(),
            "contact" => guild.contact,
            "id" => guild.id,
        }).and(Ok(()))?)
    }
}
#[cfg(test)]
mod tests {
    /*
     ██████  ██    ██ ██ ██      ██████  ███████
    ██       ██    ██ ██ ██      ██   ██ ██
    ██   ███ ██    ██ ██ ██      ██   ██ ███████
    ██    ██ ██    ██ ██ ██      ██   ██      ██
     ██████   ██████  ██ ███████ ██████  ███████
    */

    #[test]
    fn insert_guild_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id"))
            .and_then(|member| {
                db.insert_guild(
                    _s("LibrariumAachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member.id,
                )
            })
            .and_then(|orig_guild| db.get_guilds().and_then(|guilds| Ok((orig_guild, guilds))))
            .and_then(|(orig_guild, mut guilds)| {
                Ok(guilds
                    .pop()
                    .map_or(false, |fetched_guild| orig_guild == fetched_guild))
            });
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted Guild is not in DB :("),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn insert_guild_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id")).and_then(|member| {
            db.insert_guild(
                _s(TOO_LONG_STRING),
                _s("Postfach 1231238581412 1238414812 Aachen"),
                member.id,
            )
        });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    #[test]
    fn update_guild_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
            .and_then(|member| {
                db.insert_guild(
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member.id,
                )
            })
            .and_then(|guild| {
                db.insert_member(_s("other_id"))
                    .and_then(|other_member| Ok((guild, other_member)))
            })
            .and_then(|(mut guild, other_member)| {
                guild.name = _s("RPG Librarium Aaachen");
                guild.address = _s("postsfadfeddfasdfasdff");
                guild.contact = other_member.id;
                db.update_guild(&guild).and_then(|_| Ok(guild))
            })
            .and_then(|orig_guild| db.get_guilds().and_then(|guilds| Ok((orig_guild, guilds))))
            .and_then(|(orig_guild, mut guilds)| {
                Ok(guilds
                    .pop()
                    .map_or(false, |fetched_guild| orig_guild == fetched_guild))
            });
        teardown(dbname);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated guild to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_guild_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
            .and_then(|member| {
                db.insert_guild(
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member.id,
                )
            })
            .and_then(|mut guild| {
                guild.name = _s(TOO_LONG_STRING);
                db.update_guild(&guild)
            });

        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => {
                panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.name\")")
            }
        }
    }

    #[test]
    fn update_guild_address_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
            .and_then(|member| {
                db.insert_guild(
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member.id,
                )
            })
            .and_then(|mut guild| {
                guild.address = _s(TOO_LONG_STRING);
                db.update_guild(&guild)
            });

        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.address\")"
            ),
        }
    }

    #[test]
    fn insert_guild_invalid_cotact() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_guild(
            _s("RPG Librarium Aachen"),
            _s("Postfach 1231238581412 1238414812 Aachen"),
            12345,
        );
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_guild_invalid_contact() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
            .and_then(|member| {
                db.insert_guild(
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member.id,
                )
            })
            .and_then(|mut guild| {
                guild.contact = 12345;
                db.update_guild(&guild)
            });

        teardown(dbname);

        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }
}
