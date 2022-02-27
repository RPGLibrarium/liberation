use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::Authentication;
use crate::authentication::roles::{BOOKS_CREATE, BOOKS_DELETE, BOOKS_EDIT, BOOKS_READ};
use crate::models::NewBook;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    let books = actions::list_books(&conn)?;
    Ok(HttpResponse::Ok().json(books))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_book: web::Json<NewBook>,
) -> MyResponder {
    authentication.requires_role(BOOKS_CREATE)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_book(&conn, new_book.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    search_id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_role(BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    let title = actions::find_book(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(title))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    write_to_id: web::Path<i32>,
    new_info: web::Json<NewBook>,
) -> MyResponder {
    authentication.requires_role(BOOKS_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_book(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Authentication,
    delete_id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_role(BOOKS_DELETE)?;
    let conn = app.open_database_connection()?;
    let updated = actions::delete_book(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().json(updated))
}
