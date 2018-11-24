use super::*;

/// Id type for Book
pub type BookId = Id;

/// Book describes a specific (physical) book
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Book {
    /// Unique id
    pub id: Option<BookId>,
    /// Title of (physical) book
    pub title: TitleId,
    /// Type of current possessor
    pub owner_type: EntityType,
    /// Id of current possessor
    pub owner: EntityId,
    /// Condition of book
    pub quality: String,
}

impl Book {
    /// Construct a new Book object with given parameters
    pub fn new(
        id: Option<BookId>,
        title: TitleId,
        owner: EntityId,
        owner_type: EntityType,
        quality: String,
    ) -> Book {
        return Book {
            id: id,
            title: title,
            owner_type: owner_type,
            owner: owner,
            quality: quality,
        };
    }

    /// Construct a new Book object with given parameters with manual input of owner type
    pub fn from_db(
        id: BookId,
        title: TitleId,
        owner_member: Option<MemberId>,
        owner_guild: Option<GuildId>,
        owner_type: String,
        quality: String,
    ) -> Result<Book, String> {
        let owner_type = match EntityType::from_str(owner_type.as_str()) {
            Ok(x) => x,
            Err(_) => return Err(String::from("Bad owner_type")),
        };

        let owner: EntityId =
            match match owner_type {
                EntityType::Member => owner_member,
                EntityType::Guild => owner_guild,
            } {
                Some(x) => x,
                None => return Err(String::from(
                    "Field 'owner_member' or 'owner_guild' is not set according to 'owner_type'.",
                )),
            };
        Ok(Book::new(Some(id), title, owner, owner_type, quality))
    }
}

impl DMO for Book {
    type Id = BookId;

    fn get_all(db: &Database) -> Result<Vec<Book>, Error> {
        Ok(db.pool.prep_exec("select book_id, title_by_id, owner_member_by_id, owner_guild_by_id, owner_type, quality from books;",())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (id, title, owner_member, owner_guild, owner_type, quality) = mysql::from_row(row);
            //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
            Book::from_db(id, title, owner_member, owner_guild, owner_type, quality).unwrap()
        }).collect()
    })?)
    }

    fn get(db: &Database, book_id: BookId) -> Result<Option<Book>, Error> {
        let mut results = db.pool
        .prep_exec(
            "select book_id, title_by_id, owner_member_by_id, owner_guild_by_id, owner_type, quality from books where book_id=:book_id;",
            params!{
                "book_id" => book_id,
            },
        )
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, title, owner_member, owner_guild, owner_type, quality) = mysql::from_row(row);
                //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.possessor
                Book::from_db(id, title, owner_member, owner_guild, owner_type, quality).unwrap()
            }).collect::<Vec<Book>>()
        })?;
        return Ok(results.pop());
    }

    fn insert(db: &Database, inp: &Book) -> Result<BookId, Error> {
        check_varchar_length!(inp.quality);
        Ok(db.pool.prep_exec("insert into books (title_by_id, owner_member_by_id, owner_guild_by_id, quality) values (:title, :owner_member, :owner_guild, :quality)",
        params!{
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
        }).map(|result| {
                result.last_insert_id()
        })?)
    }

    fn update(db: &Database, book: &Book) -> Result<(), Error> {
        check_varchar_length!(book.quality);
        Ok(db.pool.prep_exec("update books set title_by_id=:title, owner_member_by_id=:owner_member, owner_guild_by_id=:owner_guild, quality=:quality where book_id=:id;",
        params!{
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
            "id" => book.id,
        }).and(Ok(()))?)
    }

    fn delete(db: &Database, id: Id) -> Result<bool, Error> {
        Ok(db
            .pool
            .prep_exec(
                "delete from books where book_id=:id",
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
    fn insert_and_get_book_correct() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db)
            .and_then(|(book_id, orig_book)| {
                db.get(book_id)
                    .and_then(|rec_book| Ok((orig_book, rec_book)))
            }).and_then(|(orig_book, rec_book)| {
                Ok(rec_book.map_or(false, |fetched_book| orig_book == fetched_book))
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
            }).and_then(|title_id| {
                db.insert(&mut Member::new(
                    None,
                    _s("uiii-a-uuid-or-sth-similar-2481632"),
                )).and_then(|member_id| Ok((title_id, member_id)))
            }).and_then(|(title_id, member_id)| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    member_id,
                    EntityType::Member,
                    _s(TOO_LONG_STRING),
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
            )).and_then(|member_id| {
                db.insert(&mut Book::new(
                    None,
                    01248163264,
                    member_id,
                    EntityType::Member,
                    _s("quite good"),
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
            }).and_then(|title_id| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    012481632,
                    EntityType::Member,
                    _s("quite good"),
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
            }).and_then(|title_id| {
                db.insert(&mut Book::new(
                    None,
                    title_id,
                    012481632,
                    EntityType::Guild,
                    _s("quite good"),
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
            }).and_then(|title_id| {
                db.insert(&mut Member::new(
                    None,
                    _s("annother-uuuuuiiii-iiiiddd-123443214"),
                )).and_then(|member_id| Ok((title_id, member_id)))
            }).and_then(|(title_id, member_id)| {
                db.insert(&mut Guild::new(
                    None,
                    _s("Ravenclaw"),
                    _s("Sesame Street 123"),
                    member_id,
                )).and_then(|guild_id| Ok((title_id, guild_id)))
            }).and_then(|(title_id, guild_id)| {
                insert_book_default(&db)
                    .and_then(|(book_id, orig_book)| Ok((orig_book, book_id, title_id, guild_id)))
            }).and_then(|(mut orig_book, book_id, title_id, guild_id)| {
                orig_book.title = title_id;
                orig_book.owner = guild_id;
                orig_book.owner_type = EntityType::Guild;
                orig_book.quality = _s("bad");
                db.update(&orig_book).and_then(|_| Ok((orig_book, book_id)))
            }).and_then(|(orig_book, book_id)| {
                db.get(book_id).and_then(|rec_book| {
                    Ok(rec_book.map_or(false, |fetched_book| orig_book == fetched_book))
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
        let result = insert_book_default(&db).and_then(|(book_id, mut orig_book)| {
            orig_book.title = 0123481642;
            db.update(&orig_book)
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
        let result = insert_book_default(&db).and_then(|(book_id, mut orig_book)| {
            orig_book.owner = 0123481642;
            db.update(&orig_book)
        });
        teardown(settings);
        println!(" >> res: {:?}", result);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_wrong_owner_type() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        let result = insert_book_default(&db).and_then(|(book_id, mut orig_book)| {
            orig_book.owner_type = EntityType::Guild;
            db.update(&orig_book)
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
