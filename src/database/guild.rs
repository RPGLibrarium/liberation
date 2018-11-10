use super::*;

/// Id type for guild
pub type GuildId = EntityId;

/// Any organisation involved in book renting
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guild {
    /// Unique id
    pub id: Option<GuildId>,
    /// Name of Guild
    pub name: String,
    /// Address of Guild
    pub address: String,
    /// Id of Member to contact
    pub contact: MemberId,
}

impl Guild {
    /// Construct a new Guild object with given parameters
    pub fn new(id: Option<GuildId>, name: String, address: String, contact: MemberId) -> Guild {
        Guild {
            id: id,
            name: name,
            address: address,
            contact: contact,
        }
    }
}

impl DMO for Guild {
    type Id = GuildId;

    fn insert(db: &Database, inp: &mut Guild) -> Result<GuildId, Error> {
        check_varchar_length!(inp.name, inp.address);
        Ok(db.pool.prep_exec("insert into guilds (name, address, contact_by_member_id) values (:name, :address, :contact)",
        params!{
            "name" => inp.name.clone(),
            "address" => inp.address.clone(),
            "contact" => inp.contact,
        }).map(|result| {
            inp.id = Some(result.last_insert_id());
            result.last_insert_id()
        })?)
    }

    fn get(db: &Database, guild_id: GuildId) -> Result<Option<Guild>, Error> {
        let mut results = db.pool
        .prep_exec(
            "select guild_id, name, address, contact_by_member_id from guilds where guild_id=:guild_id;",
            params!{
                "guild_id" => guild_id,
            },
        )
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, name, address, contact) = mysql::from_row(row);
            Guild {
                id: id,
                name: name,
                address: address,
                contact: contact
            }
        }).collect::<Vec<Guild>>()
    })?;
        return Ok(results.pop());
    }

    fn get_all(db: &Database) -> Result<Vec<Guild>, Error> {
        Ok(db
            .pool
            .prep_exec(
                "select guild_id, name, address, contact_by_member_id from guilds;",
                (),
            ).map(|result| {
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
                    }).collect()
            })?)
    }

    fn update(db: &Database, guild: &Guild) -> Result<(), Error> {
        check_varchar_length!(guild.name, guild.address);
        Ok(db.pool.prep_exec("update guilds set name=:name, address=:address, contact_by_member_id=:contact where guild_id=:id",
        params!{
            "name" => guild.name.clone(),
            "address" => guild.address.clone(),
            "contact" => guild.contact,
            "id" => guild.id,
        }).and(Ok(()))?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from guilds where GuildId=:id",
                params!{
                    "id" => id,
                },
            ).map_err(|err| Error::DatabaseError(err))
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
    fn insert_guild_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id")))
            .and_then(|member_id| {
                let mut orig_guild = Guild::new(
                    None,
                    _s("LibrariumAachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                );
                db.insert(&mut orig_guild)
                    .and_then(|guild_id| Ok((guild_id, orig_guild)))
            }).and_then(|(guild_id, orig_guild)| {
                db.get(guild_id).and_then(|rec_guild| {
                    Ok(rec_guild.map_or(false, |fetched_guild| orig_guild == fetched_guild))
                })
            });
        teardown(settings);
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
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id")))
            .and_then(|member_id| {
                db.insert(&mut Guild::new(
                    None,
                    _s(TOO_LONG_STRING),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    #[test]
    fn update_guild_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id1")))
            .and_then(|member_id| {
                let mut orig_guild = Guild::new(
                    None,
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                );
                db.insert(&mut orig_guild)
                    .and_then(|guild_id| Ok((guild_id, orig_guild)))
            }).and_then(|(guild_id, orig_guild)| {
                db.insert(&mut Member::new(None, _s("other_id")))
                    .and_then(|other_member_id| Ok((guild_id, orig_guild, other_member_id)))
            }).and_then(|(guild_id, mut orig_guild, other_member_id)| {
                orig_guild.name = _s("RPG Librarium Aaachen");
                orig_guild.address = _s("postsfadfeddfasdfasdff");
                orig_guild.contact = other_member_id;
                db.update(&orig_guild)
                    .and_then(|_| Ok((guild_id, orig_guild)))
            }).and_then(|(guild_id, orig_guild)| {
                db.get(guild_id).and_then(|rec_guild| {
                    Ok(rec_guild.map_or(false, |fetched_guild| orig_guild == fetched_guild))
                })
            });
        teardown(settings);

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
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id1")))
            .and_then(|member_id| {
                let mut orig_guild = Guild::new(
                    None,
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                );
                db.insert(&mut orig_guild)
                    .and_then(|guild_id| Ok(orig_guild))
            }).and_then(|mut orig_guild| {
                orig_guild.name = _s(TOO_LONG_STRING);
                db.update(&orig_guild)
            });

        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => {
                panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.name\")")
            }
        }
    }

    #[test]
    fn update_guild_address_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id1")))
            .and_then(|member_id| {
                let mut orig_guild = Guild::new(
                    None,
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                );
                db.insert(&mut orig_guild).and_then(|_| Ok(orig_guild))
            }).and_then(|mut orig_guild| {
                orig_guild.address = _s(TOO_LONG_STRING);
                db.update(&orig_guild)
            });

        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.address\")"
            ),
        }
    }

    #[test]
    fn insert_guild_invalid_contact() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db.insert(&mut Guild::new(
            None,
            _s("RPG Librarium Aachen"),
            _s("Postfach 1231238581412 1238414812 Aachen"),
            12345,
        ));
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_guild_invalid_contact() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db
            .insert(&mut Member::new(None, _s("external_id1")))
            .and_then(|member_id| {
                let mut orig_guild = Guild::new(
                    None,
                    _s("Librarium Aachen"),
                    _s("Postfach 1231238581412 1238414812 Aachen"),
                    member_id,
                );
                db.insert(&mut orig_guild)
                    .and_then(|guild_id| Ok(orig_guild))
            }).and_then(|mut orig_guild| {
                orig_guild.contact = 12345;
                db.update(&orig_guild)
            });

        teardown(settings);

        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }
}
