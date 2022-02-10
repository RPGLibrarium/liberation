use diesel::{BoolExpressionMethods, BoxableExpression, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::mysql::Mysql;
use diesel::result::DatabaseErrorKind::{UniqueViolation, ForeignKeyViolation};
use diesel::result::Error as DE;
use diesel::sql_types::Bool;
use log::debug;
use crate::error::UserFacingError as UE;
use crate::InternalError as IE;
use crate::models::*;

pub fn list_rpg_systems(conn: &MysqlConnection) -> Result<Vec<RpgSystem>, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems.load::<RpgSystem>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_rpg_system(conn: &MysqlConnection, new_rpg_system: NewRpgSystem) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    let affected = diesel::insert_into(rpg_systems)
        .values(new_rpg_system.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "create rpg system must affect only a single row.");

    // TODO: this would be nicer with postgres
    let matching = rpg_systems.filter(name.eq(new_rpg_system.name))
        .first::<RpgSystem>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_rpg_system(conn: &MysqlConnection, search_id: i32) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems.find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_rpg_system(conn: &MysqlConnection, write_to_id: i32, new_info: NewRpgSystem) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;

    let affected = diesel::update(rpg_systems.find(write_to_id))
        .set(new_info.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "update rpg system must affect only a single row.");

    find_rpg_system(conn, write_to_id)
}

pub fn list_titles(conn: &MysqlConnection) -> Result<Vec<Title>, UE> {
    use crate::schema::titles::dsl::*;
    titles.load::<Title>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_title(conn: &MysqlConnection, new_title: NewTitle) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    let affected = diesel::insert_into(titles)
        .values(new_title.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "create title must affect only a single row.");

    // TODO: this would be nicer with postgres
    let matching = titles.filter(name.eq(new_title.name))
        .first::<Title>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_title(conn: &MysqlConnection, search_id: i32) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    titles.find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_title(conn: &MysqlConnection, write_to_id: i32, new_info: NewTitle) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;

    let affected = diesel::update(titles.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::DatabaseError(ForeignKeyViolation, _) => UE::InvalidForeignKey,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "update title must affect only a single row.");

    find_title(conn, write_to_id)
}

pub fn list_accounts(conn: &MysqlConnection) -> Result<Vec<Account>, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.load::<Account>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_account(conn: &MysqlConnection, new_account: NewAccount) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::insert_into(accounts)
        .values(new_account.clone())
        .execute(conn)
        .map_err(|e| {
            debug!("inserting user caused an error {}", e);
            match e {
                DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
                _ => UE::Internal(IE::DatabaseError(e))
            }
        })?;
    assert_eq!(affected, 1, "crate account must affect only a single row.");

    // TODO: this would be nicer with postgres
    let matching = accounts.filter(external_id.eq(new_account.external_id))
        .first::<Account>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_account(conn: &MysqlConnection, search_id: i32) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_account(conn: &MysqlConnection, write_to_id: i32, new_info: NewAccount) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::update(accounts.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "update account must affect only a single row.");

    find_account(conn, write_to_id)
}

pub fn find_account_by_external_id(conn: &MysqlConnection, search_external_id: String) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.filter(external_id.eq(search_external_id))
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn deactivate_account(conn: &MysqlConnection, delete_account_id: i32) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    let affected = diesel::update(accounts.find(delete_account_id))
        .set(active.eq(false))
        .execute(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "deactivate account must affect only a single row.");
    Ok(())
}

pub fn deactivate_account_by_external_id(conn: &MysqlConnection, delete_external_id: String) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    // TODO: there is probably a way to do this in a single query.
    let account = find_account_by_external_id(conn, delete_external_id)?;
    let affected = diesel::update(&account)
        .set(active.eq(false))
        .execute(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "deactivate accounts by external id must affect only a single row.");
    Ok(())
}

pub fn list_guilds(conn: &MysqlConnection) -> Result<Vec<Guild>, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.load::<Guild>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_guild(conn: &MysqlConnection, new_guild: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    let affected = diesel::insert_into(guilds)
        .values(new_guild.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "create guilds must affect only a single row.");

    // TODO: this would be nicer with postgres
    let matching = guilds.filter(external_id.eq(new_guild.external_id))
        .first::<Guild>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_guild(conn: &MysqlConnection, search_id: i32) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_guild(conn: &MysqlConnection, write_to_id: i32, new_info: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;

    let affected = diesel::update(guilds.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "update guilds must affect only a single row.");

    find_guild(conn, write_to_id)
}

pub fn list_books(conn: &MysqlConnection) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books.load::<Book>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_book(conn: &MysqlConnection, new_book: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    let affected = diesel::insert_into(books)
        .values(new_book.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "create books must affect only a single row.");

    // TODO: this would be A FUCKING LOT nicer with postgres
    let (member_id, guild_id) = new_book.owner.into();
    debug!("memberid {:?}, guildid {:?}", member_id, guild_id);

    let matching = books.filter(with_inventory_key(
        new_book.external_inventory_id, member_id, guild_id,
    )).first::<Book>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

fn with_inventory_key(ext_inventory_id: i32, member_id: Option<i32>, guild_id: Option<i32>)
                      -> Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType=Bool>> {
    use crate::schema::books::dsl::*;
    let matches_member_id: Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType=Bool>>
        = if let Some(id) = member_id {
        Box::new(owner_member_by_id.eq(id))
    } else {
        Box::new(owner_member_by_id.is_null())
    };

    let matches_guild_id: Box<dyn BoxableExpression<crate::schema::books::table, Mysql, SqlType=Bool>>
        = if let Some(id) = guild_id {
        Box::new(owner_guild_by_id.eq(id))
    } else {
        Box::new(owner_guild_by_id.is_null())
    };

    Box::new(external_inventory_id.eq(ext_inventory_id)
        .and(matches_member_id)
        .and(matches_guild_id))
}

pub fn find_book(conn: &MysqlConnection, search_id: i32) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;
    books.find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_book(conn: &MysqlConnection, write_to_id: i32, new_info: NewBook) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;

    let affected = diesel::update(books.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "update books must affect only a single row.");

    find_book(conn, write_to_id)
}

pub fn delete_book(conn: &MysqlConnection, delete_id: i32) -> Result<(), UE> {
    use crate::schema::books::dsl::*;

    let affected = diesel::delete(books.find(delete_id))
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn list_books_owned_by_member(conn: &MysqlConnection, account: Account) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books.filter(owner_member_by_id.eq(account.account_id))
        .load(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_book_owned_by_member(conn: &MysqlConnection, account: Account, partial_book: PostOwnedBook) -> Result<Book, UE> {
    let new_book = partial_book.owned_by(Owner::Member { id: account.account_id });
    create_book(&conn, new_book)
}


pub fn find_book_owned_by_member(conn: &MysqlConnection, account: Account, search_id: i32) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;

    books.filter(owner_member_by_id.eq(account.account_id))
        .find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn delete_book_owned_by_member(conn: &MysqlConnection, account: Account, delete_id: i32) -> Result<(), UE> {
    use crate::schema::books::dsl::*;

    let affected = diesel::delete(
        books.filter(owner_member_by_id.eq(account.account_id))
            .find(delete_id)
    )
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}

pub fn list_books_owned_by_guild(conn: &MysqlConnection, guild: Guild) -> Result<Vec<Book>, UE> {
    use crate::schema::books::dsl::*;
    books.filter(owner_guild_by_id.eq(guild.guild_id))
        .load(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_book_owned_by_guild(conn: &MysqlConnection, guild: Guild, partial_book: PostOwnedBook) -> Result<Book, UE> {
    let new_book = partial_book.owned_by(Owner::Guild { id: guild.guild_id });
    create_book(&conn, new_book)
}

pub fn find_book_owned_by_guild(conn: &MysqlConnection, guild: Guild, search_id: i32) -> Result<Book, UE> {
    use crate::schema::books::dsl::*;

    books.filter(owner_guild_by_id.eq(guild.guild_id))
        .find(search_id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn delete_book_owned_by_guild(conn: &MysqlConnection, guild: Guild, delete_id: i32) -> Result<(), UE> {
    use crate::schema::books::dsl::*;

    let affected = diesel::delete(
        books.filter(owner_guild_by_id.eq(guild.guild_id))
            .find(delete_id)
    )
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    assert_eq!(affected, 1, "delete books must affect only a single row.");
    Ok(())
}
