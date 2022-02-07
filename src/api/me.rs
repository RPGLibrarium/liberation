use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::error::UserFacingError;
use crate::models::{NewAccount, NewAccountPost};

pub async fn get(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    let member_id = authentication.requires_any_member()?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account_by_external_id(&conn, member_id)?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn post(app: web::Data<AppState>, authentication: Authentication, custom: web::Json<NewAccountPost>) -> MyResponder {
    let member_id = authentication.requires_any_member()?;
    let conn = app.open_database_connection()?;

    let new_account = match authentication {
        Authentication::Authorized {
            full_name,
            given_name,
            family_name,
            email, ..
        } => Ok(NewAccount {
            active: true,
            external_id: member_id,
            full_name,
            given_name,
            family_name,
            email,
            username: custom.username.to_string(),
        }),
        Authentication::Anonymous => Err(UserFacingError::AuthenticationRequired)
    }?;

    let account = actions::create_account(&conn, new_account)?;
    Ok(HttpResponse::Created().json(account))
}

// TODO: I don't see anything useful to patch.
// pub async fn patch(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
//     let member_id = authentication.requires_any_member()?;
//     let conn = app.open_database_connection()?;
//     let account = actions::find_account_by_external_id(&conn, member_id)?;
//     Ok(HttpResponse::Ok().json(account))
// }

pub async fn delete(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    let member_id = authentication.requires_any_member()?;
    let conn = app.open_database_connection()?;
    actions::deactivate_account_by_external_id(&conn, member_id)?;
    Ok(HttpResponse::Ok().finish())
}
