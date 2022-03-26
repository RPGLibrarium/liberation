use crate::actions::{book, handle_db_errors, RowsAffectedAssertions};
use crate::error::UserFacingError as UE;
use crate::models::{Id, NewRpgSystem, Owner, RpgSystem};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

pub fn list(conn: &MysqlConnection) -> Result<Vec<RpgSystem>, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems
        .load::<RpgSystem>(conn)
        .map_err(handle_db_errors)
}

pub fn create(conn: &MysqlConnection, new_rpg_system: NewRpgSystem) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    let affected = diesel::insert_into(rpg_systems)
        .values(new_rpg_system.clone())
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(
        affected, 1,
        "create rpg system must affect only a single row."
    );

    // There is no good way of doing this with maria db since there is no `RETURNING` statement.
    let matching = rpg_systems
        .filter(name.eq(new_rpg_system.name))
        .first::<RpgSystem>(conn)
        .map_err(handle_db_errors)?;

    Ok(matching)
}

pub fn find(conn: &MysqlConnection, search_id: Id) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn update(
    conn: &MysqlConnection,
    write_to_id: Id,
    new_info: NewRpgSystem,
) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;

    let affected = diesel::update(rpg_systems.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(
        affected, 1,
        "update rpg system must affect only a single row."
    );

    find(conn, write_to_id)
}

pub fn delete(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
    use crate::schema::rpg_systems::dsl::*;
    let affected = diesel::delete(rpg_systems.find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(
        affected, 1,
        "delete rpg_system must affect only a single row."
    );
    Ok(())
}

pub fn list_owned_by(conn: &MysqlConnection, owner: Owner) -> Result<Vec<RpgSystem>, UE> {
    let mut rpgsystems: Vec<RpgSystem> = book::double_recursive_list_owned_by(conn, owner)?
        .into_iter()
        .map(|book| book.title.rpg_system)
        .collect();
    rpgsystems.sort_by_key(|rpg_system| rpg_system.id);
    rpgsystems.dedup();
    Ok(rpgsystems)
}
