pub use super::error::Error;
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
pub use self::member::MemberId;
pub use self::rental::RentalId;
pub use self::rpgsystem::RpgSystemId;
pub use self::title::TitleId;

use mysql;
pub const MAX_VARCHAR_LENGTH: usize = 255;

pub type Id = u64;

pub type Year = i16;
pub type Date = NaiveDate;

#[derive(Clone)]
pub struct Database {
    pool: mysql::Pool,
}

//static SQL_DATEFORMAT: &str = "%Y-%m-%d";

impl Database {
    pub fn new(url: String) -> Result<Database, Error> {
        let mut opts = mysql::OptsBuilder::from_opts(url);
        opts.prefer_socket(false);
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

    pub fn insert<T: DMO>(&self, inp: &T) -> Result<T, Error> {
        T::insert(self, inp)
    }

    pub fn update<T: DMO>(&self, up: &T) -> Result<(), Error> {
        T::update(self, up)
    }

    pub fn delete<T: DMO>(&self, id: T::Id) -> Result<bool, Error> {
        T::delete(self, id)
    }
}

trait DMO<T = Self> {
    type Id;
    fn get_all(&Database) -> Result<Vec<T>, Error>;
    fn get(&Database, Self::Id) -> Result<Option<T>, Error>;
    fn insert(&Database, &T) -> Result<T, Error>;
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
    use chrono::prelude::*;
    use mysql;
    use rand::{thread_rng, Rng};
    use std::env;

    pub fn _s(s: &str) -> String {
        String::from(s)
    }
    pub fn _d(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd(y, m, d)
    }
    pub fn _serv() -> String {
        let server = env::var("SQL_SERVER").expect("SQL_SERVER not set in env");
        let username = env::var("SQL_USER").expect("SQL_SERVER not set in env");
        let password = match env::var("SQL_PASSWORD") {
            Ok(password) => format!(":{}", password),
            Err(_) => _s(""),
        };
        _s(&format!("mysql://{}{}@{}", username, password, server))
    }
    pub const TOO_LONG_STRING: &str = "Das beste 👿System der Welt welches lä😀nger als 255 zeich👿en lang ist, damit wir 😀einen Varchar sprechen!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! Du willst noch mehr=!=! Hier hast du mehr doofe Zeichen !!!!!!!!!! Bist du jetzt glücklich==";

    pub fn setup() -> String {
        let setup_pool = mysql::Pool::new_manual(1, 2, _serv()).unwrap();
        let mut conn = setup_pool.get_conn().unwrap();

        let mut rng = thread_rng();
        let dbname: String = String::from(format!("test_{}", rng.gen::<u32>()));
        conn.query(format!("create database {}", dbname)).unwrap();
        return dbname;
    }

    pub fn teardown(dbname: String) {
        let pool = mysql::Pool::new_manual(1, 2, _serv()).unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.query(format!("drop database {}", dbname)).unwrap();
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
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();
        //teardown(dbname);
    }

    #[test]
    fn db_init() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", _serv(), dbname))).unwrap();

        teardown(dbname);
    }
}
