use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::models::{Member, NewMember};

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_aristocrat()
        .or(authentication.requires_any_member().map(|_| ()))?;
    let conn = app.open_database_connection()?;
    let members = actions::list_members(&conn)?;
    Ok(HttpResponse::Ok().json(members))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_member: web::Json<NewMember>,
) -> MyResponder {
    authentication.requires_aristocrat()?;
    let conn = app.open_database_connection()?;
    let created = actions::create_member(&conn, new_member.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_aristocrat()?;
    let conn = app.open_database_connection()?;
    let rpg_system = actions::find_rpg_system(&conn, *id)?;
    Ok(HttpResponse::Ok().json(rpg_system))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    updated_member: web::Json<Member>,
) -> MyResponder {
    authentication.requires_aristocrat()?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_members(&conn, updated_member.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}
