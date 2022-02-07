use actix_web::{HttpResponse, web};
use crate::actions;
use crate::auth::Authentication;
use crate::models::{NewTitle, Title};
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::roles::*;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let titles = actions::list_titles(&conn)?;
    Ok(HttpResponse::Ok().json(titles))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_title: web::Json<NewTitle>,
) -> MyResponder {
    authentication.requires_role(TITLES_CREATE)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_title(&conn, new_title.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let title = actions::find_title(&conn, *id)?;
    Ok(HttpResponse::Ok().json(title))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    updated_title: web::Json<Title>,
) -> MyResponder {
    authentication.requires_role(TITLES_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_title(&conn, updated_title.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

