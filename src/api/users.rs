use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::USERS_READ;
use crate::authentication::Claims;
use crate::models::{Id, User};
use actix_web::{web, HttpResponse};

pub async fn get_all(app: web::Data<AppState>, claims: Claims) -> MyResponder {
    claims.require_scope(USERS_READ)?;
    let conn = app.open_database_connection()?;
    let accounts = actions::account::list(&conn)?;
    let users = accounts.into_iter().map(User::from).collect::<Vec<User>>();
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_one(
    app: web::Data<AppState>,
    claims: Claims,
    id: web::Path<Id>,
) -> MyResponder {
    claims.require_scope(USERS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::account::find(&conn, *id)?;
    let user = User::from(account);
    Ok(HttpResponse::Ok().json(user))
}
