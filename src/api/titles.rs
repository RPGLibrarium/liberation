use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::*;
use crate::authentication::Claims;
use crate::models::NewTitle;
use actix_web::{web, HttpResponse};

pub async fn get_all(app: web::Data<AppState>, authentication: Claims) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let titles = actions::list_titles(&conn)?;
    Ok(HttpResponse::Ok().json(titles))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_title: web::Json<NewTitle>,
) -> MyResponder {
    authentication.require_scope(TITLES_MODIFY)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_title(&conn, new_title.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    let title = actions::find_title(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(title))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<i32>,
    new_info: web::Json<NewTitle>,
) -> MyResponder {
    authentication.require_scope(TITLES_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_title(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Claims,
    delete_id: web::Path<i32>,
) -> MyResponder {
    authentication.require_scope(TITLES_MODIFY)?;
    let conn = app.open_database_connection()?;
    actions::delete_title(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
