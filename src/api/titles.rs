use actix_web::{HttpResponse, web};
use liberation::{actions, AppState, open_database_connection};
use liberation::auth::Authentication;
use liberation::models::{NewTitle, Title};
use crate::api::MyResponder;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = open_database_connection(&app)?;
    let titles = actions::list_titles(&conn)?;
    Ok(HttpResponse::Ok().json(titles))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_title: web::Json<NewTitle>,
) -> MyResponder {
    authentication.requires_any_librarian()?;
    let conn = open_database_connection(&app)?;
    let created = actions::create_title(&conn, new_title.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = open_database_connection(&app)?;
    let title = actions::find_title(&conn, *id)?;
    Ok(HttpResponse::Ok().json(title))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    updated_title: web::Json<Title>,
) -> MyResponder {
    authentication.requires_any_librarian()?;
    let conn = open_database_connection(&app)?;
    let updated = actions::update_title(&conn, updated_title.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

