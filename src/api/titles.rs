use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::*;
use crate::authentication::Claims;
use crate::models::{Id, NewTitle, QueryOptions};
use actix_web::{web, HttpResponse};

pub async fn get_all(
    app: web::Data<AppState>,
    authentication: Claims,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;

    if (*query).recursive {
        let titles = actions::title::recursive_list(&conn)?;
        Ok(HttpResponse::Ok().json(titles))
    } else {
        let titles = actions::title::list(&conn)?;
        Ok(HttpResponse::Ok().json(titles))
    }
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Claims,
    new_title: web::Json<NewTitle>,
) -> MyResponder {
    authentication.require_scope(TITLES_ADD)?;
    let conn = app.open_database_connection()?;
    let created = actions::title::create(&conn, new_title.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Claims,
    search_id: web::Path<Id>,
    query: web::Query<QueryOptions>,
) -> MyResponder {
    authentication.requires_nothing()?;
    let conn = app.open_database_connection()?;
    if (*query).recursive {
        let title = actions::title::recursive_find(&conn, *search_id)?;
        Ok(HttpResponse::Ok().json(title))
    } else {
        let title = actions::title::find(&conn, *search_id)?;
        Ok(HttpResponse::Ok().json(title))
    }
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Claims,
    write_to_id: web::Path<Id>,
    new_info: web::Json<NewTitle>,
) -> MyResponder {
    authentication.require_scope(LIBRARIAN_TITLES_MODIFY)?;
    let conn = app.open_database_connection()?;
    let updated = actions::title::update(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete(
    app: web::Data<AppState>,
    authentication: Claims,
    delete_id: web::Path<Id>,
) -> MyResponder {
    authentication.require_scope(LIBRARIAN_TITLES_MODIFY)?;
    let conn = app.open_database_connection()?;
    actions::title::delete(&conn, *delete_id)?;
    Ok(HttpResponse::Ok().finish())
}
