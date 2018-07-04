use super::*;

pub type BookId = Id;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Book {
    pub id: BookId,
    pub title: TitleId,
    pub owner_type: EntityType,
    pub owner: EntityId,
    pub quality: String,
}

impl Book {
    pub fn new(
        id: BookId,
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

        Ok(Book::new(id, title, owner, owner_type, quality))
    }
}

impl DMO for Book {
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

    //TODO: Test
    fn get(&db: &Database, book_id: BookId) -> Result<Option<Book>, Error> {
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
                //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
                Book::from_db(id, title, owner_member, owner_guild, owner_type, quality).unwrap()
            }).collect::<Vec<Book>>()
        })?;
        return Ok(results.pop());
    }

    fn insert(
        &db: &Database,
        title: TitleId,
        owner: EntityId,
        owner_type: EntityType,
        quality: String,
    ) -> Result<Book, Error> {
        check_varchar_length!(quality);
        Ok(db.pool.prep_exec("insert into books (title_by_id, owner_member_by_id, owner_guild_by_id, quality) values (:title, :owner_member, :owner_guild, :quality)",
        params!{
            "title" => title,
            "owner_member" => match owner_type {
                EntityType::Member => Some(owner),
                EntityType::Guild => None,
            },
            "owner_guild" => match owner_type {
                EntityType::Member => None,
                EntityType::Guild => Some(owner),
            },
            "quality" => quality.clone(),
        }).map(|result| {
            Book::new(result.last_insert_id(), title, owner, owner_type, quality)
        })?)
    }

    fn update(&db: &Database, book: &Book) -> Result<(), Error> {
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
}
#[cfg(test)]
mod tests {
    /*
    ██████   ██████   ██████  ██   ██ ███████
    ██   ██ ██    ██ ██    ██ ██  ██  ██
    ██████  ██    ██ ██    ██ █████   ███████
    ██   ██ ██    ██ ██    ██ ██  ██       ██
    ██████   ██████   ██████  ██   ██ ███████
    */

    fn insert_book_default(db: &Database) -> Result<Book, Error> {
        return db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            })
            .and_then(|title| {
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            })
            .and_then(|(title, member)| {
                db.insert_book(title.id, member.id, EntityType::Member, _s("vähri guhd!"))
            });
    }

    #[test]
    fn insert_book_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = insert_book_default(&db)
            .and_then(|orig_book| db.get_books().and_then(|books| Ok((orig_book, books))))
            .and_then(|(orig_book, mut books)| {
                Ok(books
                    .pop()
                    .map_or(false, |fetched_book| orig_book == fetched_book))
            });
        teardown(dbname);
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
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            })
            .and_then(|title| {
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            })
            .and_then(|(title, member)| {
                db.insert_book(title.id, member.id, EntityType::Member, _s(TOO_LONG_STRING))
            })
            .and_then(|orig_book| db.get_books().and_then(|books| Ok((orig_book, books))))
            .and_then(|(orig_book, mut books)| {
                Ok(books
                    .pop()
                    .map_or(false, |fetched_book| orig_book == fetched_book))
            });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!(
                "Expected DatabaseError::FieldError(FieldError::DataTooLong(\"book.quality\")"
            ),
        }
    }

    #[test]
    fn insert_book_invalid_title() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
            .and_then(|member| {
                db.insert_book(01248163264, member.id, EntityType::Member, _s("quite good"))
            })
            .and_then(|orig_book| db.get_books().and_then(|books| Ok((orig_book, books))))
            .and_then(|(orig_book, mut books)| {
                Ok(books
                    .pop()
                    .map_or(false, |fetched_book| orig_book == fetched_book))
            });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_invalid_owner_id() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            })
            .and_then(|title| {
                db.insert_book(title.id, 012481632, EntityType::Member, _s("quite good"))
            })
            .and_then(|orig_book| db.get_books().and_then(|books| Ok((orig_book, books))))
            .and_then(|(orig_book, mut books)| {
                Ok(books
                    .pop()
                    .map_or(false, |fetched_book| orig_book == fetched_book))
            });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_wrong_owner_type() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system| {
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            })
            .and_then(|title| {
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            })
            .and_then(|(title, member)| {
                db.insert_book(title.id, member.id, EntityType::Guild, _s("quite good"))
            })
            .and_then(|orig_book| db.get_books().and_then(|books| Ok((orig_book, books))))
            .and_then(|(orig_book, mut books)| {
                Ok(books
                    .pop()
                    .map_or(false, |fetched_book| orig_book == fetched_book))
            });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Cthulhu"))
            .and_then(|system| {
                db.insert_title(
                    _s("Cthulhu 666th Edition"),
                    system.id,
                    _s("en"),
                    _s("Pegasus"),
                    2066,
                    None,
                )
            })
            .and_then(|title| {
                db.insert_member(_s("annother-uuuuuiiii-iiiiddd-123443214"))
                    .and_then(|member| Ok((title, member)))
            })
            .and_then(|(title, member)| {
                db.insert_guild(_s("Ravenclaw"), _s("Sesame Street 123"), member.id)
                    .and_then(|guild| Ok((title, guild)))
            })
            .and_then(|(title, guild)| {
                insert_book_default(&db).and_then(|orig_book| Ok((orig_book, title, guild)))
            })
            .and_then(|(mut orig_book, title, guild)| {
                orig_book.title = title.id;
                orig_book.owner = guild.id;
                orig_book.owner_type = EntityType::Guild;
                orig_book.quality = _s("bad");
                db.update_book(&orig_book).and_then(|_| Ok(orig_book))
            })
            .and_then(|book| {
                db.get_books().and_then(|mut books| {
                    Ok(books
                        .pop()
                        .map_or(false, |fetched_book| book == fetched_book))
                })
            });
        teardown(dbname);
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
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = insert_book_default(&db).and_then(|mut orig_book| {
            orig_book.title = 0123481642;
            db.update_book(&orig_book)
        });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_invalid_owner_id() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = insert_book_default(&db).and_then(|mut orig_book| {
            orig_book.owner = 0123481642;
            db.update_book(&orig_book)
        });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_wrong_owner_type() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = insert_book_default(&db).and_then(|mut orig_book| {
            orig_book.owner_type = EntityType::Guild;
            db.update_book(&orig_book)
        });
        teardown(dbname);
        match result {
            Err(Error::ConstraintError(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_quality_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        let result = insert_book_default(&db).and_then(|mut orig_book| {
            orig_book.quality = _s(TOO_LONG_STRING);
            db.update_book(&orig_book)
        });
        teardown(dbname);
        match result {
            Err(Error::DataTooLong(_)) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong)"),
        }
    }
}
