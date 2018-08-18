pub use super::error::Error;
use super::settings;
use chrono::prelude::*;
pub static INIT_DB_STRUCTURE: &str = include_str!("../../res/init-db-structure.sql");

macro_rules! check_varchar_length {
    ($( $x:expr ),+) => {
        $(if $x.chars().count() > 255 {
            return Err(Error::DataTooLong(String::from(stringify!($x))))
        };)*
    }
}
macro_rules! check_date {
    ($( $x:expr ),+) => {
        $(if 1000 > $x.year() || $x.year() > 9999 {
            return Err(Error::IllegalValueForType(String::from(stringify!($x))))
        };)*
    }
}

mod book;
mod entity;
mod guild;
mod member;
mod rental;
mod rpgsystem;
mod title;

pub use self::book::Book;
pub use self::entity::EntityType;
pub use self::guild::Guild;
pub use self::member::Member;
pub use self::rental::Rental;
pub use self::rpgsystem::RpgSystem;
pub use self::title::Title;

pub use self::book::BookId;
pub use self::entity::EntityId;
pub use self::guild::GuildId;
pub use self::member::ExternalId;
pub use self::member::MemberId;
pub use self::rental::RentalId;
pub use self::rpgsystem::RpgSystemId;
pub use self::title::TitleId;

use mysql;

pub type Id = u64;

pub type Year = i16;
pub type Date = NaiveDate;

pub mod type_aliases {
    pub use super::BookId;
    pub use super::EntityId;
    pub use super::ExternalId;
    pub use super::GuildId;
    pub use super::MemberId;
    pub use super::RentalId;
    pub use super::RpgSystemId;
    pub use super::TitleId;

    pub use super::Id;

    pub use super::Date;
    pub use super::Year;
}
#[derive(Clone)]
pub struct Database {
    pool: mysql::Pool,
}

//static SQL_DATEFORMAT: &str = "%Y-%m-%d";

impl Database {
    pub fn from_settings(settings: &settings::Database) -> Result<Database, Error> {
        let mut opts = mysql::OptsBuilder::default();
        opts.ip_or_hostname(settings.hostname.clone())
            .user(settings.username.clone())
            .pass(settings.password.clone())
            .db_name(Some(settings.database.clone()))
            .prefer_socket(false);

        match settings.port {
            Some(port) => {
                opts.tcp_port(port);
            }
            None => {}
        }

        let pool = mysql::Pool::new(opts)?;

        let mut conn = pool.get_conn()?;
        conn.query(INIT_DB_STRUCTURE)?;

        return Ok(Database { pool: pool });
    }

    pub fn get_all<T: DMO>(&self) -> Result<Vec<T>, Error> {
        T::get_all(self)
    }

    pub fn get<T: DMO>(&self, id: T::Id) -> Result<Option<T>, Error> {
        T::get(self, id)
    }

    pub fn insert<T: DMO>(&self, inp: &mut T) -> Result<Id, Error> {
        T::insert(self, inp)
    }

    pub fn update<T: DMO>(&self, up: &T) -> Result<(), Error> {
        T::update(self, up)
    }

    pub fn delete<T: DMO>(&self, id: T::Id) -> Result<bool, Error> {
        T::delete(self, id)
    }

    pub fn get_titles_by_rpg_system(&self, system_id: RpgSystemId) -> Result<Vec<Title>, Error> {
        let results = self.pool
        .prep_exec(
            "select title_id, name, rpg_system_by_id, language, publisher, year, coverimage from titles where rpg_system_by_id=:system_id;",
            params!{
                "system_id" => system_id,
            },
        )
        .map_err(|err| Error::DatabaseError(err))
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, system, language, publisher, year, coverimage) = mysql::from_row(row);
                Title {
                    id: id,
                    name: name,
                    system: system,
                    language: language,
                    publisher: publisher,
                    year: year,
                    coverimage: coverimage,
                }
            }).collect::<Vec<Title>>()
        });
        return results;
    }

    pub fn get_titles_with_details(&self) -> Result<Vec<(Title, RpgSystem, u32, u32)>, Error> {
        let result = self.pool
            .prep_exec(
                "select title_id, titles.name, language, publisher, year, coverimage, rpg_systems.rpg_system_id, rpg_systems.name, rpg_systems.shortname, count(book_id) as stock, exists(select rentals.rental_id from rentals where rentals.book_by_id = books.book_id and rentals.to_date >= now()) available \
                 from titles join rpg_systems on titles.rpg_system_by_id = rpg_systems.rpg_system_id \
                    left outer join books on titles.title_id = books.title_by_id \
                    group by title_id;
                    ", ()
            )
            .map_err(|err| Error::DatabaseError(err))
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, name, language, publisher, year, coverimage, system_id, system_name, system_short, stock, available): (Option<TitleId>, String, String, String, i16, Option<String>, RpgSystemId, String, Option<String>, u32, u32)  = mysql::from_row(row);
                    (
                        Title {
                            id: id,
                            name: name,
                            system: system_id,
                            language: language,
                            publisher: publisher,
                            year: year,
                            coverimage: coverimage,
                        },
                        RpgSystem {
                            id: Some(system_id),
                            name: system_name,
                            shortname: system_short,
                        },
                        stock,
                        available
                    )
                }).collect::<Vec<(Title, RpgSystem, u32, u32)>>()
            });
        return result;
    }

    pub fn get_title_with_details(
        &self,
        title_id: TitleId,
    ) -> Result<Option<(Title, RpgSystem, u32, u32)>, Error> {
        let mut result = self.pool
            .prep_exec(
                "select title_id, titles.name, language, publisher, year, coverimage, rpg_systems.rpg_system_id, rpg_systems.name, rpg_systems.shortname, count(book_id) as stock,     exists(select rentals.rental_id from rentals where rentals.book_by_id = books.book_id and rentals.to_date >= now()) available \
                 from titles join rpg_systems on titles.rpg_system_by_id = rpg_systems.rpg_system_id \
                    left outer join books on titles.title_id = books.title_by_id \
                    where title_id=:titleid \
                    group by title_id;
                    ",
                params!{
                    "titleid" => title_id,
                })
            .map_err(|err| Error::DatabaseError(err))
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, name, language, publisher, year, coverimage, system_id, system_name, system_short, stock, available) : (Option<TitleId>, String, String, String, i16, Option<String>, RpgSystemId, String, Option<String>, u32, u32) = mysql::from_row(row);
                    (
                            Title {
                                id: id,
                                name: name,
                                system: system_id,
                                language: language,
                                publisher: publisher,
                                year: year,
                                coverimage: coverimage,
                            },
                            RpgSystem {
                                id: Some(system_id),
                                name: system_name,
                                shortname: system_short
                            },
                            stock,
                            available
                    )
                }).collect::<Vec<(Title, RpgSystem, u32, u32)>>()
            })?;
        return Ok(result.pop());
    }

    //TODO: Unfinished
    pub fn get_books_by_title(&self, id: TitleId) -> Result<Vec<Book>, Error> {
        return Ok(vec![]);
    }

    // one function to query them all, retrieve their data and store it in stucts
    pub fn get_books_with_details(&self) -> Result<Vec<(Book, Option<Rental>, bool)>, Error> {
        return self.pool
            .prep_exec(
                "select
                    books.book_id, books.owner_type, books.quality, books.title_by_id, \
                    if(books.owner_type = 'member', o_members.member_id, o_guilds.guild_id) as owner_id, \
                    rentals.rental_id, rentals.from_date, rentals.to_date, rentals.rentee_type, \
                    if(rentals.rentee_type = 'member', r_members.member_id, r_guilds.guild_id) as rentee_id, \
                    (rentals.to_date is null or rentals.to_date < CURRENT_DATE) as available \
                from books \
                left outer join members as o_members on books.owner_member_by_id = o_members.member_id and books.owner_type = 'member' \
                left outer join guilds as o_guilds on books.owner_guild_by_id = o_guilds.guild_id and books.owner_type = 'guild' \
                left outer join rentals on books.book_id = rentals.book_by_id and rentals.to_date >= ALL (select to_date from rentals where book_by_id = books.book_id) \
                left outer join members as r_members on rentals.rentee_member_by_id = r_members.member_id and rentals.rentee_type = 'member' \
                left outer join guilds as r_guilds on rentals.rentee_guild_by_id = r_guilds.guild_id and rentals.rentee_type = 'guild' \
                group by book_id;
                ", ())
            .map_err(|err| Error::DatabaseError(err))
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (book_id, owner_type, quality, title_id, owner_id, rental_id, rental_from, rental_to, rentee_type, rentee_id, available)
                    : (BookId, String, String, TitleId, EntityId, Option<RentalId>, Option<NaiveDate>, Option<NaiveDate>, Option<String>, Option<EntityId>, bool) = mysql::from_row(row);
                    (
                        Book {
                            id: Some(book_id),
                            title: title_id,
                            owner_type: EntityType::from_str(owner_type.as_str()).expect("Bad owner type"),
                            owner: owner_id,
                            quality: quality,
                        },
                        rental_id.map_or_else(|| None, |id| Some(Rental {
                            id: Some(id),
                            from: rental_from.expect("rental start date is not set"),
                            to: rental_to.expect("rental end date is not set"),
                            book: book_id,
                            rentee: rentee_id.expect("rentee_id is not set"),
                            rentee_type: EntityType::from_str(rentee_type.expect("rentee type is not set").as_str()).expect("Bad rentee Type"),
                        })),
                        available,
                    )
                }).collect::<Vec<(Book, Option<Rental>, bool)>>()
            });
    }
}

pub trait DMO<T = Self> {
    type Id;
    fn get_all(&Database) -> Result<Vec<T>, Error>;
    fn get(&Database, Self::Id) -> Result<Option<T>, Error>;
    fn insert(&Database, &mut T) -> Result<Id, Error>;
    fn update(&Database, &T) -> Result<(), Error>;
    fn delete(&Database, Self::Id) -> Result<bool, Error>;
}

#[deprecated(since = "0.0.0", note = "this is a stub for later oauth roles")]
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Role {
    pub identifier: String,
}

#[cfg(test)]
mod test_util {
    use super::super::settings::Database as Db;
    use super::super::settings::TestSettings;
    use super::*;
    use mysql;
    use rand::{thread_rng, Rng};

    pub fn _s(s: &str) -> String {
        String::from(s)
    }
    pub fn _d(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd(y, m, d)
    }

    pub const TOO_LONG_STRING: &str = "Das beste 👿System der Welt welches lä😀nger als 255 zeich👿en lang ist, damit wir 😀einen Varchar sprechen!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! Du willst noch mehr=!=! Hier hast du mehr doofe Zeichen !!!!!!!!!! Bist du jetzt glücklich==";

    pub fn setup() -> Db {
        let mut settings = TestSettings::new().unwrap().database;

        let mut opts = mysql::OptsBuilder::default();
        opts.ip_or_hostname(settings.hostname.clone())
            .user(settings.username.clone())
            .pass(settings.password.clone())
            .prefer_socket(false);

        match settings.port {
            Some(port) => {
                opts.tcp_port(port);
            }
            None => {}
        }

        let setup_pool = mysql::Pool::new_manual(1, 2, opts).unwrap();
        let mut conn = setup_pool.get_conn().unwrap();

        let mut rng = thread_rng();
        settings.database = String::from(format!("test_{}", rng.gen::<u32>()));
        conn.query(format!("create database {}", settings.database))
            .unwrap();

        return settings;
    }

    pub fn teardown(settings: Db) {
        let mut opts = mysql::OptsBuilder::default();
        opts.ip_or_hostname(settings.hostname.clone())
            .user(settings.username.clone())
            .pass(settings.password.clone())
            .prefer_socket(false);

        match settings.port {
            Some(port) => {
                opts.tcp_port(port);
            }
            None => {}
        }

        let pool = mysql::Pool::new_manual(1, 2, opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.query(format!("drop database {}", settings.database))
            .unwrap();
    }

    pub fn insert_book_default(db: &Database) -> Result<(BookId, Book), Error> {
        return db
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
                )).and_then(|member_id| Ok((title_id, member_id)))
            })
            .and_then(|(title_id, member_id)| {
                let mut book = Book::new(
                    None,
                    title_id,
                    member_id,
                    EntityType::Member,
                    _s("vähri guhd!"),
                );
                db.insert(&mut book).and_then(|id| Ok((id, book)))
            });
    }
}

#[cfg(test)]
mod tests {
    use database::test_util::*;
    use database::Database;
    /*
    ████████ ███████ ███████ ████████ ███████
       ██    ██      ██         ██    ██
       ██    █████   ███████    ██    ███████
       ██    ██           ██    ██         ██
       ██    ███████ ███████    ██    ███████
    */

    #[test]
    fn connect() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();
        //teardown(settings);
    }

    #[test]
    fn db_init() {
        let settings = setup();
        let db = Database::from_settings(&settings).unwrap();

        teardown(settings);
    }
}
