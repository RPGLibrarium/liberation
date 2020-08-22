use mysql::prelude::{Queryable, FromValue};
use mysql::{Value, Row};

use crate::settings;
use crate::error::Error;


/// The FromRow trait is bad, because it wants a totally useless error type. This is just better.
pub trait BetterFromRow {
    fn from_row(row: Row) -> Result<Self, Error>
        where Self: Sized;
}

pub trait DAO: Sized {
    const TABLE_NAME: &'static str;
    const IDENTIFIER_COLUMN: &'static str;

    type Data: Into<Vec<(String, Value)>> + BetterFromRow + Clone;
    type Identifier: Into<Value> + FromValue + From<u64> + Clone;

    fn construct(id: Self::Identifier, data: Self::Data, database: &Database) -> Result<Self, Error>;
    fn deconstruct(self) -> (Self::Identifier, Self::Data);
}

#[derive(Clone)]
pub struct Database {
    pool: mysql::Pool,
}

pub static INIT_DB_STRUCTURE: &str = include_str!("../res/init-db-structure.sql");

impl Database {
    pub fn from_settings(settings: &settings::Database) -> Result<Database, Error> {
        let opts = mysql::OptsBuilder::default()
            .ip_or_hostname(settings.hostname.clone())
            .user(settings.username.clone())
            .pass(settings.password.clone())
            .db_name(Some(settings.database.clone()))
            .prefer_socket(false);

        let opts = match settings.port {
            Some(port) => opts.tcp_port(port),
            None => opts
        };

        let pool = mysql::Pool::new(opts)?;

        // TODO: Log more about model init; is model altered?
        info!("Connecting...");
        let mut conn = pool.get_conn()?;
        info!("Writing tables...");
        //conn.query_drop(INIT_DB_STRUCTURE)?;

        return Ok(Database { pool });
    }

    pub fn get<T: DAO>(&self, id: &T::Identifier) -> Result<Option<T>, Error> {
        let query_string = format!("select * from {} where {} = :id;", T::TABLE_NAME, T::IDENTIFIER_COLUMN);

        if let Some(row) = self.pool.get_conn()?
            .exec_first(query_string, params! { "id" => Value::from(id) })?
        {
            Some(T::construct(id.clone(), T::Data::from_row(row)?, self)).transpose()
        } else { Ok(None) }
    }

    pub fn get_all<T: DAO>(&self) -> Result<Vec<T>, Error> {
        let query_string = format!("select * from {};", T::TABLE_NAME);

        self.pool.get_conn()?
            .exec(query_string, ())?
            .into_iter()
            .map(|row: Row| T::construct(
                row.get(<T as DAO>::IDENTIFIER_COLUMN)
                    .ok_or(Error::IllegalState("Failed to convert identifier to u64"))?,
                T::Data::from_row(row)?, self))
            .collect()
    }

    pub fn put<T: DAO>(&mut self, data: T::Data) -> Result<T, Error> {
        let data_vec = data.clone().into();

        let named_params_string = data_vec.iter()
            .map(|(key, _)| format!(":{}", key))
            .collect::<Vec<String>>().join(", ");

        let columns_string = data_vec.iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<String>>().join(", ");

        let query_string = format!("insert into {} ({}) values ({})", T::TABLE_NAME, columns_string, named_params_string);

        let mut conn = self.pool.get_conn()?;

        conn.exec_drop(query_string, data_vec)?;

        T::construct(T::Identifier::from(conn.last_insert_id()), data, self)
    }

    pub fn post<T: DAO>(&self, item: T) -> Result<bool, Error> {
        let (identifier, data) = item.deconstruct();
        let mut data_vec = data.clone().into();
        data_vec.insert(0, (<T as DAO>::IDENTIFIER_COLUMN.to_string(), identifier.into()));

        let assignment_string = data_vec.iter().map(|(key, _)| format!("{}=:{}", key, key))
            .collect::<Vec<String>>()
            .join(", ");

        let query_string = format!("update {} {} where {}=:{}", T::TABLE_NAME, assignment_string, T::IDENTIFIER_COLUMN, T::IDENTIFIER_COLUMN);

        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(query_string, data_vec)?;

        match conn.affected_rows() {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::IllegalState("Delete affected no or more than one rows. This should not happen.")),
        }
    }

    pub fn delete_id<T: DAO>(&self, id: T::Identifier) -> Result<bool, Error> {
        let query_string = format!("delete from {} where {}=:{}", T::TABLE_NAME, T::IDENTIFIER_COLUMN, T::IDENTIFIER_COLUMN);

        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(query_string, params! { T::IDENTIFIER_COLUMN => id.into() })?;

        match conn.affected_rows() {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::IllegalState("Delete affected no or more than one rows. This should not happen.")),
        }
    }

    pub fn delete_obj<T: DAO>(&self, item: T) -> Result<bool, Error> {
        self.delete_id::<T>(item.deconstruct().0)
    }
}

