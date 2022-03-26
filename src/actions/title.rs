use crate::actions;
use crate::actions::{book, handle_db_errors, RowsAffectedAssertions};
use crate::error::UserFacingError as UE;
use crate::models::{Id, NewTitle, Owner, Title, TitleWithRpgSystem};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

pub fn list(conn: &MysqlConnection) -> Result<Vec<Title>, UE> {
    use crate::schema::titles::dsl::*;
    titles.load::<Title>(conn).map_err(handle_db_errors)
}

pub use actions::recursive::titles::recursive_list;

pub fn create(conn: &MysqlConnection, new_title: NewTitle) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    let affected = diesel::insert_into(titles)
        .values(new_title.clone())
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(affected, 1, "create title must affect only a single row.");

    let matching = titles
        .filter(name.eq(new_title.name))
        .first::<Title>(conn)
        .map_err(handle_db_errors)?;

    Ok(matching)
}

pub fn find(conn: &MysqlConnection, search_id: Id) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    titles.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn recursive_find(conn: &MysqlConnection, search_id: Id) -> Result<TitleWithRpgSystem, UE> {
    use crate::schema::titles::dsl::*;
    let title: Title = titles
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)?;
    let rpg_system = actions::rpg_system::find(conn, title.rpg_system_id)?;
    Ok((title, rpg_system).into())
}

pub fn update(conn: &MysqlConnection, write_to_id: Id, new_info: NewTitle) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;

    let affected = diesel::update(titles.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update title must affect only a single row.");

    find(conn, write_to_id)
}

pub fn delete(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
    use crate::schema::titles::dsl::*;
    let affected = diesel::delete(titles.find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "delete titles must affect only a single row.");
    Ok(())
}

pub fn list_owned_by(conn: &MysqlConnection, owner: Owner) -> Result<Vec<Title>, UE> {
    let mut titles: Vec<Title> = book::recursive_list_owned_by(conn, owner)?
        .into_iter()
        .map(|book| book.title)
        .collect();
    titles.sort_by_key(|title| title.id);
    titles.dedup();
    Ok(titles)
}

pub fn recursive_list_owned_by(
    conn: &MysqlConnection,
    owner: Owner,
) -> Result<Vec<TitleWithRpgSystem>, UE> {
    let mut titles: Vec<TitleWithRpgSystem> = book::double_recursive_list_owned_by(conn, owner)?
        .into_iter()
        .map(|book| book.title)
        .collect();
    titles.sort_by_key(|title| title.id);
    titles.dedup();
    Ok(titles)
}
