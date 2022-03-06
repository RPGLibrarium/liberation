use crate::error::{UserFacingError as UE, UserFacingError};
use crate::models::*;
use crate::InternalError as IE;
use diesel::mysql::Mysql;
use diesel::result::DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation};
use diesel::result::{Error as DE, Error};
use diesel::sql_types::Bool;
use diesel::{
    BoolExpressionMethods, BoxableExpression, ExpressionMethods, MysqlConnection,
    OptionalExtension, QueryDsl, RunQueryDsl,
};
use log::debug;

/// Mapping all the errors is anoying.
fn handle_db_errors(e: Error) -> UserFacingError {
    match e {
        DE::DatabaseError(UniqueViolation, cause) => {
            debug!("unique violation: {}", cause.message());
            UE::AlreadyExists
        }
        DE::DatabaseError(ForeignKeyViolation, cause) => {
            debug!("foreign key violation: {}", cause.message());
            UE::InvalidForeignKey
        }
        DE::NotFound => UE::NotFound,
        _ => UE::Internal(IE::DatabaseError(e)),
    }
}

pub fn list_rpg_systems(conn: &MysqlConnection) -> Result<Vec<RpgSystem>, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems
        .load::<RpgSystem>(conn)
        .map_err(handle_db_errors)
}

pub fn create_rpg_system(
    conn: &MysqlConnection,
    new_rpg_system: NewRpgSystem,
) -> Result<RpgSystem, UE> {
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

pub fn find_rpg_system(conn: &MysqlConnection, search_id: Id) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn update_rpg_system(
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

    find_rpg_system(conn, write_to_id)
}

pub fn delete_rpgsystem(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
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

pub fn list_titles(conn: &MysqlConnection) -> Result<Vec<Title>, UE> {
    use crate::schema::titles::dsl::*;
    titles.load::<Title>(conn).map_err(handle_db_errors)
}

pub fn create_title(conn: &MysqlConnection, new_title: NewTitle) -> Result<Title, UE> {
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

pub fn find_title(conn: &MysqlConnection, search_id: Id) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    titles.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn update_title(
    conn: &MysqlConnection,
    write_to_id: Id,
    new_info: NewTitle,
) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;

    let affected = diesel::update(titles.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update title must affect only a single row.");

    find_title(conn, write_to_id)
}

pub fn delete_title(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
    use crate::schema::titles::dsl::*;
    let affected = diesel::delete(titles.find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "delete titles must affect only a single row.");
    Ok(())
}

pub fn list_accounts(conn: &MysqlConnection) -> Result<Vec<Account>, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.load::<Account>(conn).map_err(handle_db_errors)
}

pub fn create_account(conn: &MysqlConnection, new_account: NewAccount) -> Result<Account, UE> {
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

pub fn find_account(conn: &MysqlConnection, search_id: Id) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    accounts
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn update_account(
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

    find_account(conn, write_to_id)
}

pub fn find_current_registered_account(
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

pub fn deactivate_account(conn: &MysqlConnection, account: &Account) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::update(accounts.find(account.id))
        .set(active.eq(false))
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(
        affected, 1,
        "deactivate account must affect only a single row."
    );

    Ok(())
}

pub fn delete_account(conn: &MysqlConnection, account: &Account) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::delete(accounts.find(account.id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(
        affected, 1,
        "delete rpg_system must affect only a single row."
    );
    Ok(())
}

pub fn list_guilds(conn: &MysqlConnection) -> Result<Vec<Guild>, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.load::<Guild>(conn).map_err(handle_db_errors)
}

pub fn create_guild(conn: &MysqlConnection, new_guild: NewGuild) -> Result<Guild, UE> {
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

pub fn find_guild(conn: &MysqlConnection, search_id: Id) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn update_guild(
    conn: &MysqlConnection,
    write_to_id: Id,
    new_info: NewGuild,
) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    let affected = diesel::update(guilds.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update guilds must affect only a single row.");

    find_guild(conn, write_to_id)
}

pub fn list_books(conn: &MysqlConnection) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books.load::<Book>(conn).map_err(handle_db_errors)
}

pub fn create_book(conn: &MysqlConnection, new_book: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::insert_into(books)
        .values(new_book.clone())
        .execute(conn)
        .map_err(handle_db_errors)?;
    assert_eq!(affected, 1, "create books must affect only a single row.");

    // TODO: this would be A FUCKING LOT nicer with postgres
    let (member_id, guild_id) = new_book.owner.into();
    debug!("memberid {:?}, guildid {:?}", member_id, guild_id);

    books
        .filter(with_inventory_key(
            new_book.external_inventory_id,
            member_id,
            guild_id,
        ))
        .first::<Book>(conn)
        .map_err(handle_db_errors)
}

/// Little helper function which creates an sql query searching for an inventory key.
fn with_inventory_key(
    ext_inventory_id: Id,
    member_id: Option<Id>,
    guild_id: Option<Id>,
) -> Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType = Bool>> {
    use crate::schema::books::dsl::*;
    let matches_member_id: Box<
        dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType = Bool>,
    > = if let Some(id) = member_id {
        Box::new(owner_member_by_id.eq(id))
    } else {
        Box::new(owner_member_by_id.is_null())
    };

    let matches_guild_id: Box<
        dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType = Bool>,
    > = if let Some(id) = guild_id {
        Box::new(owner_guild_by_id.eq(id))
    } else {
        Box::new(owner_guild_by_id.is_null())
    };

    Box::new(
        external_inventory_id
            .eq(ext_inventory_id)
            .and(matches_member_id)
            .and(matches_guild_id),
    )
}

pub fn find_book(conn: &MysqlConnection, search_id: Id) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books.find(search_id).first(conn).map_err(handle_db_errors)
}

pub fn update_book(conn: &MysqlConnection, write_to_id: Id, new_info: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::update(books.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "update books must affect only a single row.");

    find_book(conn, write_to_id)
}

pub fn delete_book(conn: &MysqlConnection, delete_id: Id) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::delete(books.find(delete_id))
        .execute(conn)
        .map_err(handle_db_errors)?
        .assert_row_existed()?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

// Member collection
pub fn create_book_owned_by_account(
    conn: &MysqlConnection,
    account: Account,
    partial_book: PostOwnedBook,
) -> Result<Book, UE> {
    let new_book = partial_book.owned_by(Owner::Member {
        id: account.id,
    });
    create_book(&conn, new_book)
}

pub fn find_book_owned_by_account(
    conn: &MysqlConnection,
    account: Account,
    search_id: Id,
) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(owner_member_by_id.eq(account.id))
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn list_books_owned_by_account(
    conn: &MysqlConnection,
    account: Account,
) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(owner_member_by_id.eq(account.id))
        .load(conn)
        .map_err(handle_db_errors)
}

pub fn delete_book_owned_by_account(
    conn: &MysqlConnection,
    account: &Account,
    delete_id: Id,
) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::delete(
        books
            .filter(owner_member_by_id.eq(account.id))
            .find(delete_id),
    )
    .execute(conn)
    .map_err(handle_db_errors)?
    .assert_row_existed()?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn delete_all_books_owned_by_account(
    conn: &MysqlConnection,
    account: &Account,
) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    diesel::delete(books.filter(owner_member_by_id.eq(account.id)))
        .execute(conn)
        .map_err(handle_db_errors)?;
    Ok(())
}

// Guild collection
pub fn create_book_owned_by_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    partial_book: PostOwnedBook,
) -> Result<Book, UE> {
    let new_book = partial_book.owned_by(Owner::Guild { id: guild.id });
    create_book(&conn, new_book)
}

pub fn find_book_owned_by_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    search_id: Id,
) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(owner_guild_by_id.eq(guild.id))
        .find(search_id)
        .first(conn)
        .map_err(handle_db_errors)
}

pub fn list_books_owned_by_guild(conn: &MysqlConnection, guild: &Guild) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books
        .filter(owner_guild_by_id.eq(guild.id))
        .load(conn)
        .map_err(handle_db_errors)
}

pub fn delete_book_owned_by_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    delete_id: i32,
) -> Result<(), UE> {
    use crate::schema::books::dsl::*;

    let affected = diesel::delete(
        books
            .filter(owner_guild_by_id.eq(guild.id))
            .find(delete_id),
    )
    .execute(conn)
    .map_err(handle_db_errors)?
    .assert_row_existed()?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn delete_all_books_owned_by_guild(conn: &MysqlConnection, guild: &Guild) -> Result<(), UE> {
    use crate::schema::books::dsl::*;
    diesel::delete(books.filter(owner_guild_by_id.eq(guild.id)))
        .execute(conn)
        .map_err(handle_db_errors)?;
    Ok(())
}

// Guilds Access control
pub fn assert_librarian_for_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    account: &Account,
) -> Result<(), UE> {
    use crate::schema::librarians::dsl::*;
    let permission = librarians
        .filter(
            guild_id
                .eq(guild.id)
                .and(account_id.eq(account.id)),
        )
        .first::<Librarian>(conn)
        .map_err(|e| match e {
            // In this case no finding a result means the permission is missing. We can't use
            // the error handler.
            DE::NotFound => UE::YouShallNotPass,
            _ => handle_db_errors(e),
        })?;
    assert_eq!(permission.account_id, account.id);
    assert_eq!(permission.guild_id, guild.id);
    Ok(())
}

pub trait AccountAssertions {
    fn assert_active(self) -> Result<Account, UE>;
    fn assert_exists(self) -> Result<Account, UE>;
    fn assert_registered(self) -> Result<Account, UE>;
}

impl AccountAssertions for Option<Account> {
    fn assert_active(self) -> Result<Account, UE> {
        let account = self.ok_or(UserFacingError::NotRegistered)?;
        match account.active {
            true => Ok(account),
            false => Err(UE::Deactivated),
        }
    }
    fn assert_exists(self) -> Result<Account, UE> {
        self.ok_or(UserFacingError::NotFound)
    }
    fn assert_registered(self) -> Result<Account, UE> {
        self.ok_or(UserFacingError::NotRegistered)
    }
}

pub trait RowsAffectedAssertions {
    fn assert_row_existed(self) -> Result<usize, UE>;
}

impl RowsAffectedAssertions for usize {
    fn assert_row_existed(self) -> Result<usize, UE> {
        if self == 0 {
            Err(UE::NotFound)
        } else {
            Ok(self)
        }
    }
}
