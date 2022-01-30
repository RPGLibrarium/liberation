use actix_web::{HttpResponse, web};
use liberation::{actions, AppState, open_database_connection};
use liberation::claims::Authentication;
use liberation::models::{NewRpgSystem, RpgSystem};
use crate::api::MyResponder;


// Don't ask to many questions about the arguments. With typing magic actix allows us to get the
// state or arguments from the request. We can use up to 12 arguments to get data auto-
// magically out of the request.
// https://github.com/actix/actix-web/blob/2a12b41456f40b28c1efe0ec6947e8f50ba22006/src/handler.rs
// https://actix.rs/docs/extractors/
pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = open_database_connection(&app)?;
    let rpg_systems = actions::list_rpg_systems(&conn)?;
    Ok(HttpResponse::Ok().json(rpg_systems))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_rpg_system: web::Json<NewRpgSystem>,
) -> MyResponder {
    authentication.requires_any_librarian()?;
    let conn = open_database_connection(&app)?;
    let created = actions::create_rpg_system(&conn, new_rpg_system.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = open_database_connection(&app)?;
    let rpg_system = actions::find_rpg_system(&conn, *id)?;
    Ok(HttpResponse::Ok().json(rpg_system))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    updated_rpg_system: web::Json<RpgSystem>,
) -> MyResponder {
    authentication.requires_any_librarian()?;
    let conn = open_database_connection(&app)?;
    let created = actions::update_rpg_system(&conn, updated_rpg_system.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}
