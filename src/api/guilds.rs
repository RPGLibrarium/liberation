use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::{ARISTOCRAT_GUILDS_MODIFY, GUILDS_READ};
use crate::authentication::Claims;
use crate::models::{Id, NewGuild};
use actix_web::{web, HttpResponse};

pub async fn get_all(app: web::Data<AppState>, authentication: Claims) -> MyResponder {
    authentication.require_scope(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guilds = actions::guild::list(&conn)?;
    Ok(HttpResponse::Ok().json(guilds))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_guild: web::Json<NewGuild>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_GUILDS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let created = actions::guild::create(&conn, new_guild.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guild = actions::guild::find(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(guild))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewGuild>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_GUILDS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::guild::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub mod collection {
    use crate::actions;
    use crate::actions::{assert_librarian_for_guild, AccountAssertions};
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::authentication::scopes::{GUILDS_COLLECTION_MODIFY, GUILDS_READ};
    use crate::authentication::Claims;
    use crate::models::{Id, PostOwnedBook, QueryOptions};
    use actix_web::{web, HttpResponse};

    pub async fn get_all(
        app: web::Data<AppState>,
        authentication: Claims,
        guild_id: web::Path<Id>,
        query: web::Query<QueryOptions>,
    ) -> MyResponder {
        authentication.require_scope(GUILDS_READ)?;
        let conn = app.open_database_connection()?;
        // Using the guild id directly would work as well, but this way we can distinguish what
        // doesn't exist and we are more consistent with the other endpoints.
        let guild = actions::guild::find(&conn, *guild_id)?;
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
        authentication: Claims,
        guild_id: web::Path<Id>,
        posted_book: web::Json<PostOwnedBook>,
    ) -> MyResponder {
        authentication.require_scope(GUILDS_COLLECTION_MODIFY)?;
        let member_id = authentication.external_account_id()?;
        let conn = app.open_database_connection()?;
        let guild = actions::guild::find(&conn, *guild_id)?;
        let account =
            actions::account::try_find_by_external_id(&conn, member_id)?.assert_active()?;
        assert_librarian_for_guild(&conn, &guild, &account)?;

        let created_book =
            actions::book::create_owned_by(&conn, guild.into(), posted_book.into_inner())?;
        Ok(HttpResponse::Created().json(created_book))
    }

    pub async fn get_one(
        app: web::Data<AppState>,
        authentication: Claims,
        search_ids: web::Path<(Id, Id)>,
        query: web::Query<QueryOptions>,
    ) -> MyResponder {
        let (guild_id, search_id) = *search_ids;

        authentication.require_scope(GUILDS_READ)?;
        let conn = app.open_database_connection()?;
        let guild = actions::guild::find(&conn, guild_id)?;
        if query.recursive {
            let book = actions::book::recursive_find_owned_by(&conn, guild.into(), search_id)?;
            Ok(HttpResponse::Ok().json(book))
        } else {
            let book = actions::book::find_owned_by(&conn, guild.into(), search_id)?;
            Ok(HttpResponse::Ok().json(book))
        }
    }

    pub async fn delete(
        app: web::Data<AppState>,
        authentication: Claims,
        ids: web::Path<(Id, Id)>,
    ) -> MyResponder {
        let (guild_id, delete_id) = *ids;
        authentication.require_scope(GUILDS_COLLECTION_MODIFY)?;
        let member_id = authentication.external_account_id()?;
        let conn = app.open_database_connection()?;
        let guild = actions::guild::find(&conn, guild_id)?;
        let account =
            actions::account::try_find_by_external_id(&conn, member_id)?.assert_active()?;
        assert_librarian_for_guild(&conn, &guild, &account)?;

        actions::book::delete_owned_by(&conn, guild.into(), delete_id)?;
        Ok(HttpResponse::Ok().finish())
    }
}
