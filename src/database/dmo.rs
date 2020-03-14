use super::Database;
use super::Id;
use mysql::{FromRowError, Row, Params, Value};
use crate::database::Error;
use std::collections::HashMap;
use std::hash::Hash;
use mysql::prelude::Queryable;
use futures::StreamExt;


/// Implementing the DMO trait guarantees the provision of basic database functions
pub trait DMO: Sized {
    /// Id
    type Id: Into<Value> + Eq + Hash;
    /// Gets all objects of self type from the underlying database
    fn get_all(db: &Database) -> Result<Vec<Self>, Error> {
        let all_columns = [vec![Self::id_column()], Self::select_columns()].concat();

        let query_string = format!("select {} from {};", all_columns.join(", "), Self::table_name());

        db.pool.get_conn()?
            .exec_map(query_string, (), Self::from_row)?
            .into_iter()
            .collect()

    }

    /// Gets an object of self type with given id from the underlying database
    fn get(db: &Database, id: Self::Id) -> Result<Option<Self>, Error> {
        let all_columns = [vec![Self::id_column()], Self::select_columns()].concat();

        let query_string = format!("select {} from {} where {} = :id;", all_columns.join(", "), Self::table_name(), Self::id_column());

        let row: Option<Row> = db.pool.get_conn()?.exec_first(query_string, params! { "id" => id.into() })?;

        Ok(match row {
            Some(x) => Some(Self::from_row(x)?),
            None => None
        })
    }

    /// Inserts an object of self type into the underlying database
    fn insert(db: &Database, dmo: &Self) -> Result<Id, Error> {
        let params = dmo.insert_params();
        // Convert into HashMap for convenience
        let mut params_map: HashMap<String, Value> = params.clone().into_iter().collect();
        // We need to remove the id, because it will be set by the database automatically
        params_map.remove(Self::id_column());
        let keys = params_map.keys();

        // Construct strings for the statement
        let named_params_string = keys.clone().into_iter()
            .map(|name| format!(":{}", name))
            .collect::<Vec<String>>()
            .join(", ");

        let columns_string = keys.into_iter()
            .map(|x| x.clone())
            .collect::<Vec<String>>()
            .join(", ");


        let query_string = format!("insert into {} ({}) values ({})", Self::table_name(), columns_string, named_params_string);

        let mut conn = db.pool.get_conn()?;

        conn.exec_drop(query_string, params)?;

        Ok(conn.last_insert_id())
    }

    /// Updates an object of self type in the underlying database
    fn update(db: &Database, dmo: &Self) -> Result<(), Error> {
        let params = dmo.insert_params();
        // Convert into HashMap for convenience
        let mut params_map: HashMap<String, Value> = params.clone().into_iter().collect();
        // The id is used in the where part and is thus removed from the keys
        let keys_without_id = params_map.keys().filter(|key| key.as_str() != Self::id_column());

        let assignments = keys_without_id.map(|name| format!("{}=:{}", name, name))
            .collect::<Vec<String>>()
            .join(", ");

        let query_string = format!("update {} {} where {}=:{}", Self::table_name(), assignments, Self::id_column(), Self::id_column());

        Ok(db.pool.get_conn()?.exec_drop(query_string, params)?)
    }

    /// Deletes an object of self type from the underlying database
    fn delete(db: &Database, id: Self::Id) -> Result<bool, Error> {
        let query_string = format!("delete from {} where {}=:{}", Self::table_name(), Self::id_column(), Self::id_column());

        let mut conn = db.pool.get_conn()?;

        conn.exec_drop(query_string, params! { Self::id_column() => id.into() })?;

        match conn.affected_rows() {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::IllegalState("Delete affected no or more than one rows. This should not happen.")),
        }
    }

    /// all dmos have a unique id.
    fn get_id(&self) -> Option<Self::Id>;

    /// columns in database representation, without primary key/id.
    fn select_columns() -> Vec<&'static str>;
    /// column that contains the primary key/id.
    fn id_column() -> &'static str;
    /// name of the table
    fn table_name() -> &'static str;
    /// construct params from the dmo
    fn insert_params(&self) -> Vec<(String, Value)>;

    fn from_row(row: Row) -> Result<Self, Error>;
}
