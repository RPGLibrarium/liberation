use crate::actions;
use crate::actions::authorization;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::{
    ARISTOCRAT_BOOKS_MODIFY, ARISTOCRAT_BOOKS_READ, COLLECTION_MODIFY, COLLECTION_READ,
    GUILDS_COLLECTION_MODIFY, GUILDS_READ,
};
use crate::authentication::Claims;
use crate::models::{Id, NewBook, QueryOptions};
use actix_web::{web, HttpResponse};

pub async fn get_all(
    app: web::Data<AppState>,
    claims: Claims,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    claims.require_scope(ARISTOCRAT_BOOKS_READ)?;
    let conn = app.open_database_connection()?;
    if query.recursive {
        let books = actions::book::double_recursive_list(&conn)?;
        Ok(HttpResponse::Ok().json(books))
    } else {
        let books = actions::book::list(&conn)?;
        Ok(HttpResponse::Ok().json(books))
    }
}

pub async fn post(
    app: web::Data<AppState>,
    claims: Claims,
    new_book: web::Json<NewBook>,
) -> MyResponder {
    claims.require_scope_in(vec![
        ARISTOCRAT_BOOKS_MODIFY,
        COLLECTION_MODIFY,
        GUILDS_COLLECTION_MODIFY,
    ])?;
    let conn = app.open_database_connection()?;
    let created = actions::book::create(&conn, new_book.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    claims: Claims,
    search_id: web::Path<Id>,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    claims.require_scope_in(vec![ARISTOCRAT_BOOKS_READ, COLLECTION_READ, GUILDS_READ])?;
    let conn = app.open_database_connection()?;
    if query.recursive {
        let book = actions::book::recursive_find(&conn, *search_id)?;
        authorization::can_read_book_of_owner(&conn, &claims, book.owner)?;
        Ok(HttpResponse::Ok().json(book))
    } else {
        let book = actions::book::find(&conn, *search_id)?;
        authorization::can_read_book_of_owner(&conn, &claims, book.owner)?;
        Ok(HttpResponse::Ok().json(book))
    }
}

pub async fn put(
    app: web::Data<AppState>,
    claims: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewBook>,
) -> MyResponder {
    claims.require_scope(ARISTOCRAT_BOOKS_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::book::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    claims: Claims,
    delete_id: web::Path<Id>,
) -> MyResponder {
    claims.require_scope_in(vec![
        ARISTOCRAT_BOOKS_MODIFY,
        COLLECTION_MODIFY,
        GUILDS_COLLECTION_MODIFY,
    ])?;
    let conn = app.open_database_connection()?;
    let book = actions::book::find(&conn, *delete_id)?;
    authorization::can_modify_book_of_owner(&conn, &claims, book.owner)?;
    actions::book::delete(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
