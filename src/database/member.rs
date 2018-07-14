use super::*;

pub type MemberId = EntityId;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Member {
    pub id: Option<MemberId>,
    pub external_id: String,
}

impl Member {
    pub fn new(id: Option<MemberId>, external_id: String) -> Member {
        Member {
            id: id,
            external_id: external_id,
        }
    }
}

impl DMO for Member {
    type Id = MemberId;
    fn insert(db: &Database, inp: &mut Member) -> Result<MemberId, Error> {
        check_varchar_length!(inp.external_id);
        Ok(db.pool
            .prep_exec(
                "insert into members (external_id) values (:external_id)",
                params!{
                    "external_id" => inp.external_id.clone(),
                },
            )
            .map(|result| {
                inp.id = Some(result.last_insert_id());
                result.last_insert_id()
            })?)
    }

    fn get(db: &Database, member_id: MemberId) -> Result<Option<Member>, Error> {
        let mut results = db.pool
            .prep_exec(
                "select member_id, external_id from members where member_id=:member_id;",
                params!{
                    "member_id" => member_id,
                },
            )
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
                    .collect::<Vec<Member>>()
            })?;
        return Ok(results.pop());
    }

    fn get_all(db: &Database) -> Result<Vec<Member>, Error> {
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

    fn update(db: &Database, member: &Member) -> Result<(), Error> {
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

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db.pool
            .prep_exec(
                "delete from members where member_id=:id",
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
    use database::*;
    #[test]
    fn insert_member_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let mut member_in = Member::new(None, String::from("someexternalId"));
        let member_out = db.insert(&mut member_in)
            .and_then(|member_id| db.get(member_id));

        teardown(dbname);
        assert_eq!(member_in, member_out.unwrap().unwrap());
    }

    #[test]
    fn insert_member_external_id_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        let result = db.insert(&mut Member::new(None, String::from(TOO_LONG_STRING)));
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

        let mut member_in = Member::new(None, _s("somememberId"));

        let result = db.insert(&mut member_in).and_then(|member_id| {
            member_in.external_id = _s("someotherId");
            db.update(&member_in).and_then(|_| {
                db.get(member_id).and_then(|rec_member| {
                    Ok(rec_member.map_or(false, |fetched_member| member_in == fetched_member))
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
        let mut member_in = Member::new(None, _s("somememberId"));

        let result = db.insert(&mut member_in).and_then(|member_id| {
            member_in.external_id = String::from(TOO_LONG_STRING);
            return db.update(&member_in);
        });

        teardown(dbname);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"member.external_id\")"),
        }
    }
}
