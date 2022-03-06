use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::{ARISTOCRAT_BOOKS_MODIFY, ARISTOCRAT_BOOKS_READ};
use crate::authentication::Claims;
use crate::models::{Id, NewBook, QueryOptions};
use actix_web::{web, HttpResponse};

pub async fn get_all(
    app: web::Data<AppState>,
    authentication: Claims,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_READ)?;
    let conn = app.open_database_connection()?;

    if query.recursive {
        let books = actions::book::recursive_list(&conn)?;
        Ok(HttpResponse::Ok().json(books))
    } else {
        let books = actions::book::list(&conn)?;
        Ok(HttpResponse::Ok().json(books))
    }
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_book: web::Json<NewBook>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let created = actions::book::create(&conn, new_book.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<Id>,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    if query.recursive {
        let books = actions::book::recursive_find(&conn, *search_id)?;
        Ok(HttpResponse::Ok().json(books))
    } else {
        let book = actions::book::find(&conn, *search_id)?;
        Ok(HttpResponse::Ok().json(book))
    }
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewBook>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::book::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Claims,
    delete_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    actions::book::delete(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
