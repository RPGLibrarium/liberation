use crate::actions::{handle_db_errors, RowsAffectedAssertions};
use crate::error::UserFacingError as UE;
use crate::models::{Account, Id, NewAccount};
use diesel::{ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl};

pub fn list(conn: &MysqlConnection) -> Result<Vec<Account>, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.load::<Account>(conn).map_err(handle_db_errors)
}

pub fn create(conn: &MysqlConnection, new_account: NewAccount) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::insert_into(accounts)
        .values(new_account.clone())
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "crate account must affect only a single row.");

    let matching = accounts
        .filter(external_id.eq(new_account.external_id))
        .first::<Account>(conn)
        .map_err(handle_db_errors)?;

    Ok(matching)
}

pub fn find(conn: &MysqlConnection, search_id: Id) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    accounts
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn try_find_by_external_id(
    conn: &MysqlConnection,
    search_external_id: String,
) -> Result<Option<Account>, UE> {
    use crate::schema::accounts::dsl::*;
    accounts
        .filter(external_id.eq(search_external_id))
        .first(conn)
        .optional()
        .map_err(handle_db_errors)
}

pub fn update(
    conn: &MysqlConnection,
    write_to_id: Id,
    new_info: NewAccount,
) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::update(accounts.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update account must affect only a single row.");

    find(conn, write_to_id)
}

pub fn deactivate(conn: &MysqlConnection, account: &Account) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::update(accounts.find(account.account_id))
        .set(active.eq(false))
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(
        affected, 1,
        "deactivate account must affect only a single row."
    );

    Ok(())
}

pub fn delete(conn: &MysqlConnection, account: &Account) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::delete(accounts.find(account.account_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(
        affected, 1,
        "delete rpg_system must affect only a single row."
    );
    Ok(())
}
