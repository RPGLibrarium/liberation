use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use diesel::result::DatabaseErrorKind::{UniqueViolation, ForeignKeyViolation};
use diesel::result::Error as DE;
use crate::models::*;
use crate::error::UserFacingError as UE;
use crate::InternalError as IE;

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

pub fn find_rpg_system(conn: &MysqlConnection, id: i32) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;
    rpg_systems.find(id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_rpg_system(conn: &MysqlConnection, rpg_system: RpgSystem) -> Result<RpgSystem, UE> {
    use crate::schema::rpg_systems::dsl::*;

    diesel::update(rpg_systems.find(rpg_system.rpg_system_id))
        .set((name.eq(rpg_system.name), shortname.eq(rpg_system.shortname)))
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_rpg_system(conn, rpg_system.rpg_system_id)
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

pub fn find_title(conn: &MysqlConnection, id: i32) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;
    titles.find(id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_title(conn: &MysqlConnection, title: Title) -> Result<Title, UE> {
    use crate::schema::titles::dsl::*;

    diesel::update(titles.find(title.title_id))
        .set((
            name.eq(title.name),
            rpg_system_by_id.eq(title.rpg_system_by_id),
            language.eq(title.language),
            publisher.eq(title.publisher),
            year.eq(title.year),
            coverimage.eq(title.coverimage)
        ))
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::DatabaseError(ForeignKeyViolation, _) => UE::InvalidForeignKey,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_title(conn, title.title_id)
}

pub fn list_members(conn: &MysqlConnection) -> Result<Vec<Member>, UE> {
    use crate::schema::members::dsl::*;
    members.load::<Member>(conn)
        .map_err(|e| UE::Internal(IE::DatabaseError(e)))
}

pub fn create_member(conn: &MysqlConnection, new_member: NewMember) -> Result<Member, UE> {
    use crate::schema::members::dsl::*;
    diesel::insert_into(members)
        .values(new_member.clone())
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    // TODO: this would be nicer with postgres
    let matching = members.filter(external_id.eq(new_member.external_id))
        .first::<Member>(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    Ok(matching)
}

pub fn find_member(conn: &MysqlConnection, id: i32) -> Result<Member, UE> {
    use crate::schema::members::dsl::*;
    members.find(id)
        .first(conn)
        .map_err(|e| match e {
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })
}

pub fn update_members(conn: &MysqlConnection, member: Member) -> Result<Member, UE> {
    use crate::schema::members::dsl::*;

    diesel::update(members.find(member.member_id))
        .set(external_id.eq(member.external_id))
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_member(conn, member.member_id)
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
    let matching = guilds.filter(external_guild_name.eq(new_guild.external_guild_name))
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

pub fn update_guild(conn: &MysqlConnection, guild: Guild) -> Result<Guild, UE> {
    use crate::schema::guilds::dsl::*;

    diesel::update(guilds.find(guild.guild_id))
        .set((
            external_guild_name.eq(guild.external_guild_name),
            name.eq(guild.name),
            address.eq(guild.address),
            contact_by_member_id.eq(guild.contact_by_member_id))
        )
        .execute(conn)
        .map_err(|e| match e {
            DE::DatabaseError(UniqueViolation, _) => UE::AlreadyExists,
            DE::NotFound => UE::NotFound,
            _ => UE::Internal(IE::DatabaseError(e))
        })?;

    find_guild(conn, guild.guild_id)
}
