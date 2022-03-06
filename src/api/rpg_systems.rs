use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::*;
use crate::authentication::Claims;
use crate::models::{Id, NewRpgSystem};
use actix_web::{web, HttpResponse};

// Don't ask to many questions about the arguments. With typing magic actix allows us to get the
// state or arguments from the request. We can use up to 12 arguments to get data auto-
// magically out of the request.
// https://github.com/actix/actix-web/blob/2a12b41456f40b28c1efe0ec6947e8f50ba22006/src/handler.rs
// https://actix.rs/docs/extractors/
pub async fn get_all(app: web::Data<AppState>, authentication: Claims) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let rpg_systems = actions::rpg_system::list(&conn)?;
    Ok(HttpResponse::Ok().json(rpg_systems))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_rpg_system: web::Json<NewRpgSystem>,
) -> MyResponder {
    authentication.require_scope(RPGSYSTEMS_ADD)?;
    let conn = app.open_database_connection()?;
    let created = actions::rpg_system::create(&conn, new_rpg_system.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<Id>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let rpg_system = actions::rpg_system::find(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(rpg_system))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewRpgSystem>,
) -> MyResponder {
    authentication.require_scope(LIBRARIAN_RPGSYSTEMS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::rpg_system::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Claims,
    delete_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(LIBRARIAN_RPGSYSTEMS_MODIFY)?;
    let conn = app.open_database_connection()?;
    actions::rpg_system::delete(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
