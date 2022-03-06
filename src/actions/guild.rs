use crate::actions::{handle_db_errors, RowsAffectedAssertions};
use crate::error::UserFacingError as UE;
use crate::models::{Guild, Id, NewGuild};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

pub fn list(conn: &MysqlConnection) -> Result<Vec<Guild>, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.load::<Guild>(conn).map_err(handle_db_errors)
}

pub fn create(conn: &MysqlConnection, new_guild: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    let affected = diesel::insert_into(guilds)
        .values(new_guild.clone())
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(affected, 1, "create guilds must affect only a single row.");

    let matching = guilds
        .filter(name.eq(new_guild.name))
        .first::<Guild>(conn)
        .map_err(handle_db_errors)?;

    Ok(matching)
}

pub fn find(conn: &MysqlConnection, search_id: Id) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn update(conn: &MysqlConnection, write_to_id: Id, new_info: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    let affected = diesel::update(guilds.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update guilds must affect only a single row.");

    find(conn, write_to_id)
}
