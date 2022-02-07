use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::DatabaseErrorKind::{UniqueViolation, ForeignKeyViolation};
use diesel::result::Error as DE;
use crate::models::*;
use crate::error::UserFacingError as UE;
use crate::InternalError as IE;
use crate::schema::rpg_systems::rpg_system_id;

pub fn list_rpg_systems(conn: &MysqlConnection) -> Result<Vec<RpgSystem>, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems.load::<RpgSystem>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_rpg_system(conn: &MysqlConnection, new_rpg_system: NewRpgSystem) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    diesel::insert_into(rpg_systems)
        .values(new_rpg_system.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

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

    diesel::update(rpg_systems.find(write_to_id))
        .set(new_info.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_rpg_system(conn, write_to_id)
}

pub fn list_titles(conn: &MysqlConnection) -> Result<Vec<Title>, UE> {
    use crate::schema::titles::dsl::*;
    titles.load::<Title>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_title(conn: &MysqlConnection, new_title: NewTitle) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    diesel::insert_into(titles)
        .values(new_title.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

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

    diesel::update(titles.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::DatabaseError(ForeignKeyViolation, _) => UE::InvalidForeignKey,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_title(conn, write_to_id)
}

pub fn list_accounts(conn: &MysqlConnection) -> Result<Vec<Account>, UE> {
    use crate::schema::accounts::dsl::*;
    accounts.load::<Account>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_account(conn: &MysqlConnection, new_account: NewAccount) -> Result<Account, UE> {
    use crate::schema::accounts::dsl::*;
    diesel::insert_into(accounts)
        .values(accounts.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

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
    diesel::update(accounts.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

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
    diesel::update(accounts.find(delete_account_id))
        .set(active.eq(false))
        .execute(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    Ok(())
}

pub fn deactivate_account_by_external_id(conn: &MysqlConnection, delete_external_id: String) -> Result<(), UE> {
    use crate::schema::accounts::dsl::*;
    // TODO: there is probably a way to do this in a single query.
    let account = find_account_by_external_id(conn, delete_external_id)?;
    diesel::update(&account)
        .set(active.eq(false))
        .execute(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;
    Ok(())
}

pub fn list_guilds(conn: &MysqlConnection) -> Result<Vec<Guild>, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.load::<Guild>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_guild(conn: &MysqlConnection, new_guild: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    diesel::insert_into(guilds)
        .values(new_guild.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    // TODO: this would be nicer with postgres
    let matching = guilds.filter(external_id.eq(new_guild.external_id))
        .first::<Guild>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_guild(conn: &MysqlConnection, id: i32) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;
    guilds.find(id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_guild(conn: &MysqlConnection, write_to_id: i32, new_info: NewGuild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;

    diesel::update(guilds.find(write_to_id))
        .set(new_info)
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_guild(conn, write_to_id)
}
