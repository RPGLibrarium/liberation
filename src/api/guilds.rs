use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::auth::roles::{GUILDS_CREATE, GUILDS_EDIT, GUILDS_READ};
use crate::models::NewGuild;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guilds = actions::list_guilds(&conn)?;
    Ok(HttpResponse::Ok().json(guilds))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_guild: web::Json<NewGuild>,
) -> MyResponder {
    authentication.requires_role(GUILDS_CREATE)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_guild(&conn, new_guild.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    search_id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_role(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guild = actions::find_guild(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(guild))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    write_to_id: web::Path<i32>,
    new_info: web::Json<NewGuild>,
) -> MyResponder {
    authentication.requires_role(GUILDS_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_guild(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub mod collection {
    use actix_web::{HttpResponse, web};
    use crate::actions;
    use crate::actions::find_guild;
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::auth::Authentication;
    use crate::auth::roles::GUILDS_READ;
    use crate::models::PostOwnedBook;

    pub async fn get_all(app: web::Data<AppState>, authentication: Authentication, guild_id: web::Path<i32>) -> MyResponder {
        //TODO: find better roles, ex.
        // liberation:guild:collection:read
        // liberation:guild:{externalGuildId}:collection:read
        authentication.requires_role(GUILDS_READ)?;
        let conn = app.open_database_connection()?;
        // Using the guild id directly would work as well, but this way we can distinguish what
        // doesn't exist and we are more consistent with the other endpoints.
        let guild= actions::find_guild(&conn, *guild_id)?;
        let books = actions::list_books_owned_by_guild(&conn, guild)?;
        Ok(HttpResponse::Ok().json(books))
    }

    pub async fn post(
        app: web::Data<AppState>,
        authentication: Authentication,
        guild_id: web::Path<i32>,
        posted_book: web::Json<PostOwnedBook>,
    ) -> MyResponder {
        //TODO: Guilds are endpoints where a data base connection is required to check the
        // role. I don't like it.
        let conn = app.open_database_connection()?;
        let guild = find_guild(&conn, *guild_id)?;
        authentication.requires_librarian(&guild.external_id)?;
        let created_book = actions::create_book_owned_by_guild(&conn, guild, posted_book.into_inner())?;
        Ok(HttpResponse::Created().json(created_book))
    }

    pub async fn get_one(
        app: web::Data<AppState>,
        authentication: Authentication,
        search_ids: web::Path<(i32, i32)>,
    ) -> MyResponder {
        let (guild_id, search_id) = *search_ids;

        //TODO: find better roles. s.o.
        authentication.requires_role(GUILDS_READ)?;
        let conn = app.open_database_connection()?;
        let guild = find_guild(&conn, guild_id)?;
        let book = actions::find_book_owned_by_guild(&conn, guild, search_id)?;
        Ok(HttpResponse::Ok().json(book))
    }

    pub async fn delete(
        app: web::Data<AppState>,
        authentication: Authentication,
        ids: web::Path<(i32, i32)>,
    ) -> MyResponder {
        let (guild_id, delete_id) = *ids;
        let conn = app.open_database_connection()?;
        let guild = find_guild(&conn, guild_id)?;
        authentication.requires_librarian(&guild.external_id)?;
        actions::delete_book_owned_by_guild(&conn, guild, delete_id)?;
        Ok(HttpResponse::Ok().finish())
    }
}
