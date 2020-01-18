use mysql::prelude::FromRow;
use super::Database;
use super::Id;
use mysql::{FromRowError, Row, Params, Value};
use crate::database::Error;
use std::collections::HashMap;


/// Implementing the DMO trait guarantees the provision of basic database functions
pub trait DMO<T = Self>: FromRow {
    /// Id
    type Id;
    /// Gets all objects of self type from the underlying database
    fn get_all(db: &Database) -> Result<Vec<T>, Error> {
        let all_columns = [[Self::id_column()], Self::columns()].concat();

        let query_string = format!("select {} from {};", all_columns.join(", "), Self::table_name());

        let results = db.pool.prep_exec(query_string, ())?;

        results.map(|x| x.unwrap())
            .map(|row| Self::from_row(row))
            .collect()
    }

    /// Gets an object of self type with given id from the underlying database
    fn get(db: &Database, id: Self::Id) -> Result<Option<T>, Error> {
        let all_columns = [[Self::id_column()], Self::columns()].concat();

        let query_string = format!("select {} from {} where {} = ?;", all_columns.join(", "), Self::table_name(), Self::id_column());

        let results = db.pool.prep_exec(query_string, params![id])?;

        results.map(|x| x.unwrap())
            .map(|row| Self::from_row(row))
            .collect()
            .pop()
    }

    /// Inserts an object of self type into the underlying database
    fn insert(db: &Database, dmo: &T) -> Result<Id, Error> {
        let named_parameters = Self::columns().iter().map(|name| format!(":{}", name));

        let query_string = format!("insert into {} ({}) values ({})", Self::table_name(), Self::columns().join(", "), named_parameters.join(", "));

        let results = db.pool.prep_exec(query_string, dmo.to_row())?;

        Ok(results.last_insert_id())
    }

    /// Updates an object of self type in the underlying database
    fn update(db: &Database, dmo: &T) -> Result<(), Error> {
        let assignments = Self::columns().iter().map(|name| format!("{}=:{}", name, name));

        let query_string = format!("update {} {} where {}=:{}", Self::table_name(), assignmets.join(", "), Self::id_column(), Self::id_column());

        let values = dmo.to_db_params();

        let params = values.clone();
        params.insert(Self::id_column(), dmo.to_db_id());

        db.pool.prep_exec(query_string, params)?;
        Ok(())
    }

    /// Deletes an object of self type from the underlying database
    fn delete(db: &Database, id: Self::Id) -> Result<bool, Error> {
        let query_string = format!("delete from {} where {}=:{}", Self::table_name(), Self::id_column(), Self::id_column());

        let results = db.pool.prep_exec(query_string, params! { Self::id_column() => id })?;

        match results.affected_rows() {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::IllegalState),
        }
    }

    /// columns in database representation, without primary key/id.
    fn columns() -> &'static Vec<String>;
    /// column that contains the primary key/id.
    fn id_column() -> &'static String;
    /// name of the table
    fn table_name() -> &'static String;
    /// construct params from the dmo
    fn to_db_params(&self) -> HashMap<String, Value>;
    fn to_db_id(&self) -> Id;
}
