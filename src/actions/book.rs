use crate::actions;
use crate::actions::{handle_db_errors, RowsAffectedAssertions};
use crate::error::UserFacingError as UE;
use crate::models::{Book, Id, NewBook, Owner, PostOwnedBook, RecursiveBook};
use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, MysqlConnection, QueryDsl,
    RunQueryDsl,
};

/// Little helper function which creates an sql query filtering by owner.
fn with_owner(
    owner: Owner,
) -> Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType = Bool>> {
    use crate::schema::books::dsl::*;
    match owner {
        Owner::Member { id } => {
            Box::new(owner_member_by_id.eq(id).and(owner_guild_by_id.is_null()))
        }
        Owner::Guild { id } => Box::new(owner_guild_by_id.eq(id).and(owner_member_by_id.is_null())),
    }
}

/// Little helper function which creates an sql query searching for an inventory key.
fn with_inventory_key(
    ext_inventory_id: Id,
    owner: Owner,
) -> Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType = Bool>> {
    use crate::schema::books::dsl::*;
    Box::new(
        external_inventory_id
            .eq(ext_inventory_id)
            .and(with_owner(owner)),
    )
}

pub fn list(conn: &MysqlConnection) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books.load::<Book>(conn).map_err(handle_db_errors)
}

pub use actions::recursive::books::recursive_list;

pub fn create(conn: &MysqlConnection, new_book: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::insert_into(books)
        .values(new_book.clone())
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(affected, 1, "create books must affect only a single row.");

    // TODO: this would be A FUCKING LOT nicer with postgres
    books
        .filter(with_inventory_key(
            new_book.external_inventory_id,
            new_book.owner,
        ))
        .first::<Book>(conn)
        .map_err(handle_db_errors)
}

pub fn find(conn: &MysqlConnection, search_id: Id) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn recursive_find(conn: &MysqlConnection, search_id: Id) -> Result<RecursiveBook, UE> {
    use crate::schema::books::dsl::*;
    let book: Book = books
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)?;
    let title = actions::title::recursive_find(conn, book.id)?;
    Ok(RecursiveBook::from((book, title)))
}

pub fn update(conn: &MysqlConnection, write_to_id: Id, new_info: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::update(books.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update books must affect only a single row.");

    find(conn, write_to_id)
}

pub fn delete(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::delete(books.find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn create_owned_by(
    conn: &MysqlConnection,
    owner: Owner,
    partial: PostOwnedBook,
) -> Result<Book, UE> {
    create(conn, partial.owned_by(owner))
}

pub fn find_owned_by(conn: &MysqlConnection, owner: Owner, search_id: Id) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(with_owner(owner))
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn recursive_find_owned_by(conn: &MysqlConnection, owner: Owner, search_id: Id) -> Result<RecursiveBook, UE> {
    // I couldn't find a way to filter inner_joins, hence we filter on application level
    let book = recursive_find(&conn, search_id)?;
    if book.owner == owner {
        Ok(book)
    } else { Err(UE::NotFound) }
}

pub fn list_owned_by(conn: &MysqlConnection, owner: Owner) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(with_owner(owner))
        .load(conn)
        .map_err(handle_db_errors)
}

pub fn recursive_list_owned_by(
    conn: &MysqlConnection,
    owner: Owner,
) -> Result<Vec<RecursiveBook>, UE> {
    // I couldn't find a way to filter inner_joins, hence we filter on application level
    Ok(recursive_list(conn)?
        .into_iter()
        .filter(|book| book.owner == owner)
        .collect())
}

pub fn delete_owned_by(conn: &MysqlConnection, owner: Owner, delete_id: Id) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::delete(books.filter(with_owner(owner)).find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn delete_all_owned_by(conn: &MysqlConnection, owner: Owner) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    diesel::delete(books.filter(with_owner(owner)))
        .execute(conn)
        .map_err(handle_db_errors)?;
    Ok(())
}
