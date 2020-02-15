use super::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use mysql::{Value, FromRowError, Row};

/// Id type for Member
pub type MemberId = EntityId;
/// Id type for external identification
pub type ExternalId = String;

/// Identification information for a person
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Member {
    /// Id
    pub id: Option<MemberId>,
    /// External id for identification with KeyCloak
    pub external_id: ExternalId,
}

impl DMO for Member {
    type Id = MemberId;

    fn select_columns() -> Vec<&'static str> {
        vec!["external_id"]
    }

    fn id_column() -> &'static str {
        "member_id"
    }

    fn table_name() -> &'static str {
        "members"
    }

    fn insert_params(&self) -> Vec<(String, Value)> {
        params! {
            "member_id" => self.id,
            "external_id" => self.external_id
        }
    }

    fn from_row(row: Row) -> Result<Self, Error> {
        let (id, external_id) = mysql::from_row(row.clone());

        Ok(Member {
            id,
            external_id,
        })
    }
}

/*
#[cfg(test)]
mod tests {
    use database::test_util::*;
    use database::*;
    #[test]
    fn insert_member_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let mut member_in = Member::new(None, String::from("someexternalId"));
        let mut member_new_id = None;
        let member_out = db.insert(&mut member_in).and_then(|member_id| {
            member_new_id = Some(member_id);
            db.get(member_id)
        });

        teardown(settings);
        assert_eq!(
            Member {
                id: member_new_id,
                ..member_in
            },
            member_out.unwrap().unwrap()
        );
    }

    #[test]
    fn insert_member_external_id_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let result = db.insert(&mut Member::new(None, String::from(TOO_LONG_STRING)));
        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"external_id\")"
            ),
        }
    }

    #[test]
    fn update_member_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        let mut member_in = Member::new(None, _s("somememberId"));

        let result = db.insert(&mut member_in).and_then(|member_id| {
            let member_update = Member {
                id: Some(member_id),
                external_id: _s("someotherId"),
                ..member_in
            };
            db.update(&member_update).and_then(|_| {
                db.get(member_id).and_then(|rec_member| {
                    Ok(rec_member.map_or(false, |fetched_member| member_update == fetched_member))
                })
            })
        });

        teardown(settings);

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
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let mut member_in = Member::new(None, _s("somememberId"));

        let result = db.insert(&mut member_in).and_then(|member_id| {
            member_in.external_id = String::from(TOO_LONG_STRING);
            return db.update(&member_in);
        });

        teardown(settings);

        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"member.external_id\")"),
        }
    }
}
*/
