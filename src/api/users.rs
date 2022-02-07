use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::auth::roles::USERS_READ;
use crate::models::User;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(USERS_READ)?;
    let conn = app.open_database_connection()?;
    let accounts = actions::list_accounts(&conn)?;
    let users = accounts.into_iter().map(User::from).collect::<Vec<User>>();
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_role(USERS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account(&conn, *id)?;
    let user = User::from(account);
    Ok(HttpResponse::Ok().json(user))
}
