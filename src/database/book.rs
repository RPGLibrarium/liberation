use super::*;
use mysql::{Row, QueryResult, MySqlError};

/// Id type for Book
pub type BookId = Id;
pub type ExternalInventoryId = u64;

/// EntityType describes whether an entity is a person or an organisation
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum BookState {
    /// Free for rental by everybody,
    Free,
    /// Rented
    Rented,
    /// Reserved, can only be rented by next person in queue
    Reserved,
    /// Lost, might respawn some day but not available for rental at the moment
    Lost,
    /// Destroyed in all eternity
    Destroyed,
}

impl BookState {
    /// Converts a string describing a BookState to a BookState
    /// possible values: "free", "rented", "reserved", "lost", "destroyed"
    pub fn from_str(s: &str) -> Result<BookState, String> {
        match s {
            "free" => Ok(BookState::Free),
            "rented" => Ok(BookState::Rented),
            "reserved" => Ok(BookState::Reserved),
            "lost" => Ok(BookState::Lost),
            "destroyed" => Ok(BookState::Destroyed),
            _ => Err(String::from("Expected 'free' or 'rented', 'reserved', 'lost', 'destroyed'.")),
        }
    }

    /// Converts an EntityType to a corresponding string
    pub fn to_string(&self) -> String {
        match self {
            BookState::Free => String::from("free"),
            BookState::Rented => String::from("rented"),
            BookState::Reserved => String::from("reserved"),
            BookState::Lost => String::from("lost"),
            BookState::Destroyed => String::from("destroyed"),
        }
    }
}

/// Book describes a specific (physical) book
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Book {
    /// Unique id
    pub id: Option<BookId>,
    /// External id written onto a book and in guild inventory lists
    pub external_inventory_id: ExternalInventoryId,
    /// Title of (physical) book
    pub title: TitleId,
    /// Type of current possessor
    pub owner_type: EntityType,
    /// Id of current possessor
    pub owner: EntityId,
    /// Condition of book
    pub quality: String,
    /// 'Rental' state of the book
    pub state: BookState,
    /// Since when is the book in it's state
    pub state_since: Date,
    /// Type of current Rentee
    pub rentee_type: EntityType,
    /// Id of current Rentee
    pub rentee: EntityId,
}

impl Book {
    /// Construct a new Book object with given parameters with manual input of owner type
    pub fn from_db(
        row: Row
        /*
        id: BookId,
        title: TitleId,
        owner_member: Option<MemberId>,
        owner_guild: Option<GuildId>,
        owner_type: String,
        book_state: String,
        book_state_since: Date,
        quality: String,
        external_inventory_id: ExternalInventoryId,
        */
    ) -> Result<Book, String> {
        let (id, title, owner_member, owner_guild, owner_type, quality, external_inventory_id, book_state, state_since, rentee_type, rentee_member, rentee_guild) = mysql::from_row(row);

        let owner_type = EntityType::from_str(owner_type.as_str())
            .map_err("Bad owner_type in database")?;

        let rentee_type = EntityType::from_str(rentee_type.as_str())
            .map_err("Bad rentee_type in database")?;

        let owner: EntityId = owner_type.select_entity_id(owner_member, owner_guild)?;
        let rentee: EntityId = rentee_type.select_entity_id(rentee_member, rentee_guild)?;

        let state: BookState = BookState::from_str(book_state.as_str())
            .map_err("Bad state in database")?;


        Ok(Book {
            id: Some(id),
            external_inventory_id,
            title,
            owner,
            owner_type,
            quality,
            state,
            state_since,
            rentee,
            rentee_type,
        })
    }
}

//TODO: Transform E1 and E2 to our Database Error.
fn to_vec<F, R, E1, E2>(query: MyResult<QueryResult>, f: F) -> Result<Vec<R>, E1> where F: Fn(Row) -> Result<R, E2> {
    query.map(|result| {
        result.map(|x| x.unwrap()).map(f.unwrap()).collect()
    })
}

impl DMO for Book {
    type Id = BookId;

    fn get_all(db: &Database) -> Result<Vec<Book>, Error> {
        let query = db.pool.prep_exec("select book_id, title_by_id, owner_member_by_id, owner_guild_by_id, owner_type, quality, external_inventory_id, state, state_since, rentee_type, rentee_member_by_id, rentee_guild_by_id from books;", ());
        to_vec(query, |row| Book::from_db(row));
    }

    fn get(db: &Database, book_id: BookId) -> Result<Option<Book>, Error> {
        let mut results = db.pool
            .prep_exec(
                "select book_id, title_by_id, owner_member_by_id, owner_guild_by_id, owner_type, quality, external_inventory_id, state, state_since, rentee_type, rentee_member_by_id, rentee_guild_by_id from books where book_id=:book_id;",
                params! {
                "book_id" => book_id,
            },
            )
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    //let (id, title, owner_member, owner_guild, owner_type, quality, external_inventory_id) = mysql::from_row(row);
                    //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
                    Book::from_db(row).unwrap()
                    //Book::from_db(id, title, owner_member, owner_guild, owner_type, quality, external_inventory_id).unwrap()
                }).collect::<Vec<Book>>()
            })?;
        return Ok(results.pop());
    }

    fn insert(db: &Database, inp: &Book) -> Result<BookId, Error> {
        check_varchar_length!(inp.quality);
        Ok(db.pool.prep_exec("insert into books (title_by_id, owner_member_by_id, owner_guild_by_id, quality, external_inventory_id) values (:title, :owner_member, :owner_guild, :quality, :external_inventory_id)",
                             params! {
            "title" => inp.title,
            "owner_member" => match inp.owner_type {
                EntityType::Member => Some(inp.owner),
                EntityType::Guild => None,
            },
            "owner_guild" => match inp.owner_type {
                EntityType::Member => None,
                EntityType::Guild => Some(inp.owner),
            },
            "quality" => inp.quality.clone(),
            "external_inventory_id" => inp.external_inventory_id,
        }).map(|result| {
            result.last_insert_id()
        })?)
    }

    fn update(db: &Database, book: &Book) -> Result<(), Error> {
        check_varchar_length!(book.quality);
        Ok(db.pool.prep_exec("update books set title_by_id=:title, owner_member_by_id=:owner_member, owner_guild_by_id=:owner_guild, quality=:quality, external_inventory_id=:external_inventory_id where book_id=:id;",
                             params! {
            "title" => book.title,
            "owner_member" => match book.owner_type {
                EntityType::Member => Some(book.owner),
                EntityType::Guild => None,
            },
            "owner_guild" => match book.owner_type {
                EntityType::Member => None,
                EntityType::Guild => Some(book.owner),
            },
            "quality" => book.quality.clone(),
            "external_inventory_id" => book.external_inventory_id,
            "id" => book.id,
        }).and(Ok(()))?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from books where book_id=:id",
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

/*
#[cfg(test)]
mod tests {
    use database::test_util::*;

    use database::*;

    #[test]
    fn insert_and_get_book_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, orig_book)| {
                db.get(book_id).and_then(|rec_book| {
                    Ok((
                        Book {
                            id: Some(book_id),
                            ..orig_book
                        },
                        rec_book,
                    ))
                })
            })
            .and_then(|(book, rec_book)| {
                Ok(rec_book.map_or(false, |fetched_book| book == fetched_book))
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted book is not in DB :("),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn insert_book_quality_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, _s("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                ))
            })
            .and_then(|title_id| {
                db.insert(&mut Member::new(
                    None,
                    _s("uiii-a-uuid-or-sth-similar-2481632"),
                ))
                    .and_then(|member_id| Ok((title_id, member_id)))
            })
            .and_then(|(title_id, member_id)| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    member_id,
                    EntityType::Member,
                    _s(TOO_LONG_STRING),
                    42,
                ))
            });

        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"book.quality\")"
            ),
        }
    }

    #[test]
    fn insert_book_invalid_title() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut Member::new(
                None,
                _s("uiii-a-uuid-or-sth-similar-2481632"),
            ))
            .and_then(|member_id| {
                db.insert(&mut Book::new(
                    None,
                    01248163264,
                    member_id,
                    EntityType::Member,
                    _s("quite good"),
                    42,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_invalid_owner_id() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, _s("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                ))
            })
            .and_then(|title_id| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    012481632,
                    EntityType::Member,
                    _s("quite good"),
                    42,
                ))
            });

        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_wrong_owner_type() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, _s("Kobolde"), None))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    _s("Kobolde"),
                    system_id,
                    _s("de"),
                    _s("??"),
                    2031,
                    None,
                ))
            })
            .and_then(|title_id| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    012481632,
                    EntityType::Guild,
                    _s("quite good"),
                    42,
                ))
            });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = db
            .insert(&mut RpgSystem::new(None, _s("Cthulhu"), Some(_s("CoC"))))
            .and_then(|system_id| {
                db.insert(&mut Title::new(
                    None,
                    _s("Cthulhu 666th Edition"),
                    system_id,
                    _s("en"),
                    _s("Pegasus"),
                    2066,
                    None,
                ))
            })
            .and_then(|title_id| {
                db.insert(&mut Member::new(
                    None,
                    _s("annother-uuuuuiiii-iiiiddd-123443214"),
                ))
                    .and_then(|member_id| Ok((title_id, member_id)))
            })
            .and_then(|(title_id, member_id)| {
                db.insert(&mut Guild::new(
                    None,
                    _s("Ravenclaw"),
                    _s("Sesame Street 123"),
                    member_id,
                ))
                    .and_then(|guild_id| Ok((title_id, guild_id)))
            })
            .and_then(|(title_id, guild_id)| {
                insert_book_default(&db)
                    .and_then(|(book_id, orig_book)| Ok((orig_book, book_id, title_id, guild_id)))
            })
            .and_then(|(orig_book, book_id, title_id, guild_id)| {
                let book_update = Book {
                    id: Some(book_id),
                    title: title_id,
                    owner: guild_id,
                    owner_type: EntityType::Guild,
                    quality: _s("bad"),
                    external_inventory_id: 21,
                    ..orig_book
                };
                db.update(&book_update)
                    .and_then(|_| Ok((book_update, book_id)))
            })
            .and_then(|(book_update, book_id)| {
                db.get(book_id).and_then(|rec_book| {
                    Ok(rec_book.map_or(false, |fetched_book| book_update == fetched_book))
                })
            });
        teardown(settings);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated book to be corretly stored in DB"),
            _ => {
                result.unwrap();
                ()
            }
        }
    }

    #[test]
    fn update_book_invalid_title() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, orig_book)| {
            let book_update = Book {
                id: Some(book_id),
                title: 012481642,
                ..orig_book
            };
            db.update(&book_update)
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_invalid_owner_id() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, orig_book)| {
            let book_update = Book {
                id: Some(book_id),
                owner: 012481642,
                ..orig_book
            };
            db.update(&book_update)
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_wrong_owner_type() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, orig_book)| {
            let book_update = Book {
                id: Some(book_id),
                owner_type: EntityType::Guild,
                ..orig_book
            };
            db.update(&book_update)
        });
        teardown(settings);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_quality_too_long() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, mut orig_book)| {
            orig_book.quality = _s(TOO_LONG_STRING);
            db.update(&orig_book)
        });
        teardown(settings);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }
}
*/
