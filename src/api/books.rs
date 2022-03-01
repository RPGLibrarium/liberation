use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::Claims;
use crate::authentication::scopes::{ARISTOCRAT_BOOKS_MODIFY, ARISTOCRAT_BOOKS_READ};
use crate::models::{Id, NewBook};

pub async fn get_all(app: web::Data<AppState>, authentication: Claims) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    let books = actions::list_books(&conn)?;
    Ok(HttpResponse::Ok().json(books))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_book: web::Json<NewBook>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_book(&conn, new_book.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    let book = actions::find_book(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(book))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewBook>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_book(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Claims,
    delete_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    actions::delete_book(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
