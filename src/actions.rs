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
