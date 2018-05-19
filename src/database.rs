use mysql;
use dmos;
use error::DatabaseError as Error;
use error::FieldError;

pub static INIT_DB_STRUCTURE: &str = include_str!("../res/init-db-structure.sql");

const MAX_VARCHAR_LENGTH: usize = 255;

pub struct Database {
    pool: mysql::Pool
}

macro_rules! check_varchar_length{
    ($( $x:expr ),+) => {
        $(if $x.chars().count() > MAX_VARCHAR_LENGTH {
            return Err(Error::from(FieldError::DataTooLong(String::from(stringify!($x)))))
        };)*
    }
}
impl Database {

    pub fn new(url:String) -> Result<Database, Error> {
        let pool = mysql::Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        conn.query(INIT_DB_STRUCTURE)?;

        return Ok(Database{
            pool: pool,
        })
    }

    pub fn insert_rpg_system(&self, name: String) -> Result<dmos::RpgSystem, Error> {
        check_varchar_length!(name);
        Ok(self.pool.prep_exec("insert into rpg_systems (name) values (:name)",
            params!{
                "name" => name.clone(),
            }).map(|result| {
                dmos::RpgSystem {
                    id: result.last_insert_id(),
                    name: name,
                }
            })?)
        }

    pub fn get_rpg_systems(&self) -> Result<Vec<dmos::RpgSystem>, Error> {
        Ok(self.pool.prep_exec("select * from rpg_systems;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name) = mysql::from_row(row);
                dmos::RpgSystem {
                    id: id,
                    name: name,
                }
            }).collect()
        })?)
    }

    pub fn update_rpg_system(&self, rpgsystem: &dmos::RpgSystem) ->  Result<(), Error> {
        check_varchar_length!(rpgsystem.name);
        Ok(self.pool.prep_exec("update rpg_systems set name=:name where id=:id;",
            params!{
                "name" => rpgsystem.name.clone(),
                "id" => rpgsystem.id,
            }).and(Ok(()))?)
    }

    pub fn get_titles(&self) -> Result<Vec<dmos::Title>, Error> {
        Ok(self.pool.prep_exec("select id, name, system, language, publisher, year, coverimage from titles;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, system, language, publisher, year, coverimage) = mysql::from_row(row);
                dmos::Title {
                    id: id,
                    name: name,
                    system: system,
                    language: language,
                    publisher: publisher,
                    year: year,
                    coverimage: coverimage,
                }
            }).collect()
        })?)
    }

    pub fn insert_title(&self, name: String, system: dmos::RpgSystemId, language: String, publisher: String, year: dmos::Year, coverimage: Option<String>) -> Result<dmos::Title, Error>{
        check_varchar_length!(name, language, publisher);
        Ok(self.pool.prep_exec("insert into titles (name, system, language, publisher, year, coverimage) values (:name, :system, :language, :publisher, :year, :coverimage)",
            params!{
                "name" => name.clone(),
                "system" => system,
                "language" => language.clone(),
                "publisher" => publisher.clone(),
                "year" => year,
                "coverimage" => coverimage.clone(),
            }).map(|result| {
                dmos::Title {
                    id: result.last_insert_id(),
                    name: name,
                    system: system,
                    language: language,
                    publisher: publisher,
                    year: year,
                    coverimage: coverimage,
                }
            })?)
    }

    pub fn update_title(&self, title: &dmos::Title) -> Result<(), Error> {
        check_varchar_length!(title.name, title.language, title.publisher);
        Ok(self.pool.prep_exec("update titles set name=:name, system=:system, language=:language, publisher=:publisher, year=:year, coverimage=:coverimage where id=:id;",
            params!{
                "name" => title.name.clone(),
                "system" => title.system,
                "language" => title.language.clone(),
                "publisher" => title.publisher.clone(),
                "year" => title.year,
                "coverimage" => title.coverimage.clone(),
                "id" => title.id,
            }).and(Ok(()))?)
    }

    pub fn get_books(&self) -> Result<Vec<dmos::Book>, Error> {
        Ok(self.pool.prep_exec("select id, title, owner_member, owner_guild, owner_type, quality from books;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, title, owner_member, owner_guild, owner_type, quality) = mysql::from_row(row);
                //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
                dmos::Book::from_db(id, title, owner_member, owner_guild, owner_type, quality).unwrap()
            }).collect()
        })?)
    }

    pub fn insert_book(&self, title: dmos::TitleId, owner: dmos::EntityId, owner_type: dmos::EntityType, quality: String) -> Result<dmos::Book, Error>{
        check_varchar_length!(quality);
        Ok(self.pool.prep_exec("insert into books (title, owner_member, owner_guild, quality) values (:title, :owner_member, :owner_guild, :quality)",
            params!{
                "title" => title,
                "owner_member" => match owner_type {
                    dmos::EntityType::Member => Some(owner),
                    dmos::EntityType::Guild => None,
                },
                "owner_guild" => match owner_type {
                    dmos::EntityType::Member => None,
                    dmos::EntityType::Guild => Some(owner),
                },
                "quality" => quality.clone(),
            }).map(|result| {
                dmos::Book::new(result.last_insert_id(), title, owner, owner_type, quality)
            })?)
    }

    pub fn update_book(&self, book: &dmos::Book) -> Result<(), Error> {
        check_varchar_length!(book.quality);
        Ok(self.pool.prep_exec("update books set title=:title, owner_member=:owner_member, owner_guild=:owner_guild, quality=:quality where id=:id;",
            params!{
                "title" => book.title,
                "owner_member" => match book.owner_type {
                    dmos::EntityType::Member => Some(book.owner),
                    dmos::EntityType::Guild => None,
                },
                "owner_guild" => match book.owner_type {
                    dmos::EntityType::Member => None,
                    dmos::EntityType::Guild => Some(book.owner),
                },
                "quality" => book.quality.clone(),
                "id" => book.id,
            }).and(Ok(()))?)
    }

    pub fn insert_member(&self, external_id: String) -> Result<dmos::Member, Error> {
        check_varchar_length!(external_id);
        Ok(self.pool.prep_exec("insert into members (external_id) values (:external_id)",
            params!{
                "external_id" => external_id.clone(),
            }).map(|result| {
                dmos::Member {
                    id: result.last_insert_id(),
                    external_id: external_id,
                }
            })?)
    }

    pub fn get_members(&self) -> Result<Vec<dmos::Member>, Error> {
        Ok(self.pool.prep_exec("select id, external_id from members;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, external_id) = mysql::from_row(row);
                dmos::Member {
                    id: id,
                    external_id: external_id,
                }
            }).collect()
        })?)
    }

    pub fn update_member(&self, member: &dmos::Member) ->  Result<(), Error> {
        check_varchar_length!(member.external_id);
        Ok(self.pool.prep_exec("update members set external_id=:external_id where id=:id",
            params!{
                "external_id" => member.external_id.clone(),
                "id" => member.id,
            }).and(Ok(()))?)
    }

    pub fn insert_guild(&self, name: String, address: String, contact: dmos::MemberId) -> Result<dmos::Guild, Error> {
        check_varchar_length!(name, address);
        Ok(self.pool.prep_exec("insert into guilds (name, address, contact) values (:name, :address, :contact)",
            params!{
                "name" => name.clone(),
                "address" => address.clone(),
                "contact" => contact,
            }).map(|result| {
                dmos::Guild {
                    id: result.last_insert_id(),
                    name: name,
                    address: address,
                    contact: contact,
                }
            })?)
    }

    pub fn get_guilds(&self) -> Result<Vec<dmos::Guild>, Error> {
        Ok(self.pool.prep_exec("select id, name, address, contact from guilds;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, address, contact) = mysql::from_row(row);
                dmos::Guild {
                    id: id,
                    name: name,
                    address: address,
                    contact: contact,
                }
            }).collect()
        })?)
    }

    pub fn update_guild(&self, guild: &dmos::Guild) ->  Result<(), Error> {
        check_varchar_length!(guild.name, guild.address);
        Ok(self.pool.prep_exec("update guilds set name=:name, address=:address, contact=:contact where id=:id",
            params!{
                "name" => guild.name.clone(),
                "address" => guild.address.clone(),
                "contact" => guild.contact,
                "id" => guild.id,
            }).and(Ok(()))?)
    }

    pub fn get_rentals(&self) -> Result<Vec<dmos::Rental>, Error> {
        Ok(self.pool.prep_exec("select id, from, to, book, rentee_member, rentee_guild, rentee_type from rentals;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, from, to, book, rentee_member, rentee_guild, rentee_type) = mysql::from_row(row);
                //FIXME: @FutureMe: You should have handled the error directly!!!! You stupid prick.
                dmos::Rental::from_db(id, from, to, book, rentee_member, rentee_guild, rentee_type).unwrap()
            }).collect()
        })?)
    }

    pub fn insert_rental(&self, id: dmos::RentalId, from: dmos::Date, to: dmos::Date, book: dmos::BookId, rentee: dmos::EntityId, rentee_type: dmos::EntityType) -> Result<dmos::Rental, Error>{
        Ok(self.pool.prep_exec("insert into titles ( from, to, book, rentee_member, rentee_guild) values (:from, :to, :book, :rentee_member, :rentee_guild)",
            params!{
                "from" => from.clone(),
                "to" => to.clone(),
                "book" => book,
                "rentee_member" => match rentee_type {
                    dmos::EntityType::Member => Some(rentee),
                    dmos::EntityType::Guild => None,
                },
                "rentee_guild" => match rentee_type {
                    dmos::EntityType::Member => None,
                    dmos::EntityType::Guild => Some(rentee),
                },
            }).map(|result| {
                dmos::Rental::new(result.last_insert_id(), from, to, book, rentee, rentee_type)
            })?)
    }

    pub fn update_rental(&self, rental: &dmos::Rental) -> Result<(), Error> {
        Ok(self.pool.prep_exec("update rentals set from=:from, to=:to, book=:book, rentee_member=:rentee_member, rentee_guild=:rentee_guild where id=:id;",
            params!{
                "from" => rental.from.clone(),
                "to" => rental.to.clone(),
                "book" => rental.book,
                "rentee_member" => match rental.rentee_type {
                    dmos::EntityType::Member => Some(rental.rentee),
                    dmos::EntityType::Guild => None,
                },
                "rentee_guild" => match rental.rentee_type {
                    dmos::EntityType::Member => None,
                    dmos::EntityType::Guild => Some(rental.rentee),
                },
                "id" => rental.id,
            }).and(Ok(()))?)
    }
}

#[cfg(test)]
mod tests {

    /*
    ████████ ███████ ███████ ████████ ███████
       ██    ██      ██         ██    ██
       ██    █████   ███████    ██    ███████
       ██    ██           ██    ██         ██
       ██    ███████ ███████    ██    ███████
    */


    use database::Database;
    use mysql;
    use dmos;
    use error::FieldError;
    use error::DatabaseError;
    use rand::{Rng, thread_rng};

    fn _s(s: &str) -> String { String::from(s) }

    const TOO_LONG_STRING: &str = "Das beste 👿System der Welt welches lä😀nger als 255 zeich👿en lang ist, damit wir 😀einen Varchar sprechen!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! Du willst noch mehr=!=! Hier hast du mehr doofe Zeichen !!!!!!!!!! Bist du jetzt glücklich==";
    const EXPECTED_TOO_LONG: &str = "Expected DatabaseError::FieldError(FieldError::DataTooLong)";
    const SERVER: &str = "mysql://root:thereIsNoPassword!@172.18.0.3";
    fn setup() -> String {
        let setup_pool = mysql::Pool::new_manual(1, 2, SERVER).unwrap();
        let mut conn = setup_pool.get_conn().unwrap();

        let mut rng = thread_rng();
        let dbname: String = String::from(format!("test_{}", rng.next_u32()));
        conn.query(format!("create database {}", dbname)).unwrap();
        return dbname;
    }

    fn teardown(dbname: String) {
        let pool = mysql::Pool::new_manual(1, 2, SERVER).unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.query(format!("drop database {}", dbname)).unwrap();
    }

    #[test]
    fn connect() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        teardown(dbname);
    }

    #[test]
    fn insert_rpg_system_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let system_in = db.insert_rpg_system(String::from("SR5👿")).unwrap();
        let system_out = db.get_rpg_systems().unwrap().pop().unwrap();
        assert_eq!(system_in, system_out);
        teardown(dbname);
    }

    #[test]
    fn insert_rpg_system_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_rpg_system(String::from(TOO_LONG_STRING));
        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    #[test]
    fn update_rpg_system_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_rpg_system(_s("SR5👿"))
            .and_then(|mut system| {
                system.name = _s("SR5");
                db.update_rpg_system(&system)
                .and_then(|_| {
                    db.get_rpg_systems()
                    .and_then(|mut systems| Ok(systems.pop().map_or(false, |fetched_system| system == fetched_system)))
                })
            });

        teardown(dbname);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated rpgsystem to be corretly stored in DB"),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn update_rpg_system_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_rpg_system(String::from("SR5👿"))
        .and_then(|mut rpgsystem| {
            rpgsystem.name = String::from(TOO_LONG_STRING);
            return db.update_rpg_system(&rpgsystem);
        });

        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"rpgsystem.name\")"),
        }
    }

    #[test]
    fn insert_title_name_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(String::from(TOO_LONG_STRING), system.id, String::from("de"), String::from("??"), 1248, None));
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn insert_title_language_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(String::from("Kobolde"), system.id, String::from(TOO_LONG_STRING), String::from("??"), 1248, None));
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn insert_title_publisher_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(String::from("Kobolde"), system.id, String::from("de"), String::from(TOO_LONG_STRING), 1248, None));
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn insert_title_correct(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None))
            .and_then(|title| {
                db.get_titles()
                    .and_then(|mut titles| Ok(titles.pop().map_or(false, |fetched_title| title == fetched_title)))
            });
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted title was not in DB :("),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn update_title_name_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None))
            .and_then(|mut title| {
                title.name = _s(TOO_LONG_STRING);
                return db.update_title(&title)
            });
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn update_title_language_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None))
            .and_then(|mut title| {
                title.language = _s(TOO_LONG_STRING);
                return db.update_title(&title)
            });
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn update_title_publisher_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2022, None))
            .and_then(|mut title| {
                title.publisher = _s(TOO_LONG_STRING);
                return db.update_title(&title)
            });
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!(EXPECTED_TOO_LONG),
        }
    }

    #[test]
    fn update_title_correct(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(String::from("Kobolde"))
            .and_then(|system| db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2142, None))
            .and_then(|mut title| {
                title.name = _s("new name");
                title.year = 1999;
                title.publisher = _s("new publisher");
                db.update_title(&title)
                    .and_then(|_| {
                        db.get_titles()
                            .and_then(|mut titles| Ok(titles.pop().map_or(false, |fetched_title| title == fetched_title)))
                    })
            });
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated title to be corretly stored in DB"),
            _ => { result.unwrap(); () },
        }
    }

    /*
    ██████   ██████   ██████  ██   ██ ███████
    ██   ██ ██    ██ ██    ██ ██  ██  ██
    ██████  ██    ██ ██    ██ █████   ███████
    ██   ██ ██    ██ ██    ██ ██  ██       ██
    ██████   ██████   ██████  ██   ██ ███████
    */

    fn insert_book_default(db: &Database) -> Result<dmos::Book, DatabaseError> {
        return db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system|
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            )
            .and_then(|title|
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            )
            .and_then(|(title, member)|
                db.insert_book(title.id, member.id, dmos::EntityType::Member, _s("vähri guhd!"))
            )
    }

    #[test]
    fn insert_book_correct(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        /*let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system|
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
                    .and_then(|title| Ok(title))
            )
            .and_then(|title|
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            )
            .and_then(|(title, member)|
                db.insert_book(title.id, member.id, dmos::EntityType::Member, _s("vähri guhd!"))
            )
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );*/
        let result = insert_book_default(&db)
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted book is not in DB :("),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn insert_book_quality_too_long(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system|
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            )
            .and_then(|title|
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            )
            .and_then(|(title, member)|
                db.insert_book(title.id, member.id, dmos::EntityType::Member, _s(TOO_LONG_STRING))
            )
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"book.quality\")"),
        }
    }

    #[test]
    fn insert_book_invalid_title(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
            .and_then(|member|
                db.insert_book(01248163264, member.id, dmos::EntityType::Member, _s("quite good"))
            )
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::ConstraintError(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_invalid_owner_id(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system|
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            )
            .and_then(|title|
                db.insert_book(title.id, 012481632, dmos::EntityType::Member, _s("quite good"))
            )
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::ConstraintError(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn insert_book_invalid_owner_type(){
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();
        let result = db.insert_rpg_system(_s("Kobolde"))
            .and_then(|system|
                db.insert_title(_s("Kobolde"), system.id, _s("de"), _s("??"), 2031, None)
            )
            .and_then(|title|
                db.insert_member(_s("uiii-a-uuid-or-sth-similar-2481632"))
                    .and_then(|member| Ok((title, member)))
            )
            .and_then(|(title, member)|
                db.insert_book(title.id, member.id, dmos::EntityType::Guild, _s("quite good"))
            )
            .and_then(|orig_book|
                db.get_books().and_then(|books| Ok((orig_book, books)))
            )
            .and_then(|(orig_book, mut books)|
                Ok(books.pop().map_or(false, |fetched_book| orig_book == fetched_book))
            );
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::ConstraintError(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::ConstraintError)"),
        }
    }

    #[test]
    fn update_book_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_rpg_system(_s("Cthulhu"))
            .and_then(|system|
                db.insert_title(_s("Cthulhu 666th Edition"), system.id, _s("en"), _s("Pegasus"), 2066, None)
            )
            .and_then(|title|
                db.insert_member(_s("annother-uuuuuiiii-iiiiddd-123443214"))
                    .and_then(|member| Ok((title, member)))
            ).and_then(|(title, member)|
                db.insert_guild(_s("Ravenclaw"), _s("Sesame Street 123"), member.id)
                    .and_then(|guild| Ok((title, guild)))
            ).and_then(|(title, guild)|
                insert_book_default(&db)
                    .and_then(|mut orig_book| Ok((orig_book, title, guild)))
            )
            .and_then(|(mut orig_book, title, guild)| {
                orig_book.title = title.id;
                orig_book.owner = guild.id;
                orig_book.owner_type = dmos::EntityType::Guild;
                orig_book.quality = _s("bad");
                db.update_book(&orig_book)
                    .and_then(|_| Ok(orig_book))
            })
            .and_then(|book| {
                db.get_books()
                    .and_then(|mut books| Ok(books.pop().map_or(false, |fetched_book| book == fetched_book)))
            });
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated book to be corretly stored in DB"),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn update_book_quality_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let book = dmos::Book{
            id: 123,
            title: 123,
            owner: 456,
            owner_type: dmos::EntityType::Member,
            quality: String::from(TOO_LONG_STRING)
        };
        let result = db.update_book(&book);
        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"book.quality\")"),
        }
    }

    #[test]
    fn insert_member_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let member_in = db.insert_member(String::from("someexternalId")).unwrap();
        let member_out = db.get_members().unwrap().pop().unwrap();
        assert_eq!(member_in, member_out);
        teardown(dbname);
    }

    #[test]
    fn insert_member_external_id_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(String::from(TOO_LONG_STRING));
        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"external_id\")"),
        }
    }

    #[test]
    fn update_member_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("somememberId"))
            .and_then(|mut member| {
                member.external_id = _s("someotherId");
                db.update_member(&member)
                .and_then(|_| {
                    db.get_members()
                    .and_then(|mut members| Ok(members.pop().map_or(false, |fetched_member| member == fetched_member)))
                })
            });

        teardown(dbname);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated member to be corretly stored in DB"),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn update_member_external_id_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(String::from("somememberId"))
        .and_then(|mut member| {
            member.external_id = String::from(TOO_LONG_STRING);
            return db.update_member(&member);
        });

        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"member.external_id\")"),
        }
    }

    #[test]
    fn insert_guild_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("external_id"))
        .and_then(|member| {
            db.insert_guild(_s("LibrariumAachen"), _s("Postfach 1231238581412 1238414812 Aachen"), member.id)
        })
        .and_then(|orig_guild| {
            db.get_guilds().and_then(|guilds| Ok((orig_guild, guilds)))
        })
        .and_then(|(orig_guild, mut guilds)| {
            Ok(guilds.pop().map_or(false, |fetched_guild| orig_guild == fetched_guild))
        });
        teardown(dbname);
        match result {
            Ok(true) => (),
            Ok(false) => panic!("Inserted Guild is not in DB :("),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn insert_guild_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("external_id"))
        .and_then(|member|
            db.insert_guild(_s(TOO_LONG_STRING), _s("Postfach 1231238581412 1238414812 Aachen"), member.id)
        );
        teardown(dbname);
        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"name\")"),
        }
    }

    #[test]
    fn update_guild_correct() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
        .and_then(|member|
            db.insert_guild(_s("Librarium Aachen"), _s("Postfach 1231238581412 1238414812 Aachen"), member.id)
        )
        .and_then(|guild|
            db.insert_member(_s("other_id")).and_then(|other_member| Ok((guild, other_member))))
        .and_then(|(mut guild, other_member)| {
            guild.name = _s("RPG Librarium Aaachen");
            guild.address = _s("postsfadfeddfasdfasdff");
            guild.contact = other_member.id;
            db.update_guild(&guild).and_then(|_| Ok(guild))
        })
        .and_then(|orig_guild|
            db.get_guilds().and_then(|guilds| Ok((orig_guild, guilds)))
        )
        .and_then(|(orig_guild, mut guilds)|
            Ok(guilds.pop().map_or(false, |fetched_guild| orig_guild == fetched_guild))
        );
        teardown(dbname);

        match result {
            Ok(true) => (),
            Ok(false) => panic!("Expected updated guild to be corretly stored in DB"),
            _ => { result.unwrap(); () },
        }
    }

    #[test]
    fn update_guild_name_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
        .and_then(|member|
            db.insert_guild(_s("Librarium Aachen"), _s("Postfach 1231238581412 1238414812 Aachen"), member.id)
        )
        .and_then(|mut guild| {
            guild.name = _s(TOO_LONG_STRING);
            db.update_guild(&guild)
        });

        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.name\")"),
        }
    }

    #[test]
    fn update_guild_address_too_long() {
        let dbname = setup();
        let db = Database::new(String::from(format!("{}/{}", SERVER, dbname))).unwrap();

        let result = db.insert_member(_s("external_id1"))
        .and_then(|member|
            db.insert_guild(_s("Librarium Aachen"), _s("Postfach 1231238581412 1238414812 Aachen"), member.id)
        )
        .and_then(|mut guild| {
            guild.address = _s(TOO_LONG_STRING);
            db.update_guild(&guild)
        });

        teardown(dbname);

        match result {
            Err(DatabaseError::FieldError(FieldError::DataTooLong(_))) => (),
            _ => panic!("Expected DatabaseError::FieldError(FieldError::DataTooLong(\"guild.address\")"),
        }
    }

}
