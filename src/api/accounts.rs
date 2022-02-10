use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::auth::roles::{ACCOUNTS_DELETE, ACCOUNTS_EDIT, ACCOUNTS_READ};
use crate::models::NewAccount;

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::list_accounts(&conn)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn get_one(app: web::Data<AppState>, authentication: Authentication, search_id: web::Path<i32>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_READ)?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn put(app: web::Data<AppState>, authentication: Authentication, write_to_id: web::Path<i32>, new_info: web::Json<NewAccount>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated_account = actions::update_account(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated_account))
}

pub async fn delete(app: web::Data<AppState>, authentication: Authentication, delete_id: web::Path<i32>) -> MyResponder {
    authentication.requires_role(ACCOUNTS_DELETE)?;
    let conn = app.open_database_connection()?;
    actions::deactivate_account(&conn, delete_id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}
