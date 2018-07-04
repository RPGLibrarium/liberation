use super::*;

pub type MemberId = EntityId;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Member {
    pub id: MemberId,
    pub external_id: String,
}

impl DMO for Member {
    fn insert(&self, external_id: String) -> Result<Member, Error> {
        check_varchar_length!(external_id);
        Ok(self.pool
            .prep_exec(
                "insert into members (external_id) values (:external_id)",
                params!{
                    "external_id" => external_id.clone(),
                },
            )
            .map(|result| Member {
                id: result.last_insert_id(),
                external_id: external_id,
            })?)
    }

    fn get(&db: &Database) -> Result<Vec<Member>, Error> {
        Ok(db.pool
            .prep_exec("select member_id, external_id from members;", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, external_id) = mysql::from_row(row);
                        Member {
                            id: id,
                            external_id: external_id,
                        }
                    })
                    .collect()
            })?)
    }

    fn update(&db: &Database, member: &Member) -> Result<(), Error> {
        check_varchar_length!(member.external_id);
        Ok(db.pool
            .prep_exec(
                "update members set external_id=:external_id where member_id=:id",
                params!{
                    "external_id" => member.external_id.clone(),
                    "id" => member.id,
                },
            )
            .and(Ok(()))?)
    }
}
#[cfg(test)]
mod tests {
    /*
    ███    ███ ███████ ███    ███ ██████  ███████ ██████  ███████
    ████  ████ ██      ████  ████ ██   ██ ██      ██   ██ ██
    ██ ████ ██ █████   ██ ████ ██ ██████  █████   ██████  ███████
    ██  ██  ██ ██      ██  ██  ██ ██   ██ ██      ██   ██      ██
    ██      ██ ███████ ██      ██ ██████  ███████ ██   ██ ███████
    */

    #[test]
    fn insert_member_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let member_in = db.insert_member(String::from("someexternalId")).unwrap();
        let member_out = db.get_members().unwrap().pop().unwrap();
        assert_eq!(member_in, member_out);
        teardown(dbname);
    }

    #[test]
    fn insert_member_external_id_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(String::from(TOO_LONG_STRING));
        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"external_id\")"
            ),
        }
    }

    #[test]
    fn update_member_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(_s("somememberId")).and_then(|mut member| {
            member.external_id = _s("someotherId");
            db.update_member(&member).and_then(|_| {
                db.get_members().and_then(|mut members| {
                    Ok(members
                        .pop()
                        .map_or(false, |fetched_member| member == fetched_member))
                })
            })
        });

        teardown(dbname);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated member to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_member_external_id_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert_member(String::from("somememberId"))
            .and_then(|mut member| {
                member.external_id = String::from(TOO_LONG_STRING);
                return db.update_member(&member);
            });

        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"member.external_id\")"),
        }
    }
}
