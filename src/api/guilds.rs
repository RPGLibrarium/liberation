use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::{ARISTOCRAT_GUILDS_MODIFY, GUILDS_READ};
use crate::authentication::Claims;
use crate::models::{Id, NewGuild};
use actix_web::{web, HttpResponse};

pub async fn get_all(app: web::Data<AppState>, claims: Claims) -> MyResponder {
    claims.require_scope(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guilds = actions::guild::list(&conn)?;
    Ok(HttpResponse::Ok().json(guilds))
}

pub async fn post(
    app: web::Data<AppState>,
    claims: Claims,
    new_guild: web::Json<NewGuild>,
) -> MyResponder {
    claims.require_scope(ARISTOCRAT_GUILDS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let created = actions::guild::create(&conn, new_guild.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    claims: Claims,
    search_id: web::Path<Id>,
) -> MyResponder {
    claims.require_scope(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guild = actions::guild::find(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(guild))
}

pub async fn put(
    app: web::Data<AppState>,
    claims: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewGuild>,
) -> MyResponder {
    claims.require_scope(ARISTOCRAT_GUILDS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::guild::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub mod collection {
    use crate::actions;
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::authentication::scopes::{GUILDS_COLLECTION_MODIFY, GUILDS_READ};
    use crate::authentication::Claims;
    use crate::models::{Id, Owner, PostOwnedBook, QueryOptions};
    use actix_web::{web, HttpResponse};

    pub async fn get_all(
        app: web::Data<AppState>,
        claims: Claims,
        guild_id: web::Path<Id>,
        query: web::Query<QueryOptions>,
    ) -> MyResponder {
        claims.require_scope(GUILDS_READ)?;
        let conn = app.open_database_connection()?;
        // Using the guild id directly would work as well, but this way we can distinguish what
        // doesn't exist and we are more consistent with the other endpoints.
        let guild = Owner::from(actions::guild::find(&conn, *guild_id)?);
        actions::authorization::can_read_book_of_owner(&conn, &claims, guild)?;
        if query.recursive {
            let books = actions::book::recursive_list_owned_by(&conn, guild.into())?;
            Ok(HttpResponse::Ok().json(books))
        } else {
            let books = actions::book::list_owned_by(&conn, guild.into())?;
            Ok(HttpResponse::Ok().json(books))
        }
    }

    pub async fn post(
        app: web::Data<AppState>,
        claims: Claims,
        guild_id: web::Path<Id>,
        posted_book: web::Json<PostOwnedBook>,
    ) -> MyResponder {
        claims.require_scope(GUILDS_COLLECTION_MODIFY)?;
        let conn = app.open_database_connection()?;
        let guild = Owner::from(actions::guild::find(&conn, *guild_id)?);
        actions::authorization::can_modify_book_of_owner(&conn, &claims, guild)?;

        let created_book = actions::book::create_owned_by(&conn, guild, posted_book.into_inner())?;
        Ok(HttpResponse::Created().json(created_book))
    }
}
