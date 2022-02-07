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

pub mod books {
    use actix_web::web;
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::auth::Authentication;
    use crate::models::NewBook;

    pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
        let member_id = authentication.requires_any_member()?;
        let conn = app.open_database_connection()?;
        todo!("")
        // let books = actions::list_books(&conn)?;
        // Ok(HttpResponse::Ok().json(books))
    }

    pub async fn post(
        app: web::Data<AppState>,
        authentication: Authentication,
        new_book: web::Json<NewBook>,
    ) -> MyResponder {
        let member_id = authentication.requires_any_member()?;
        let conn = app.open_database_connection()?;
        todo!("")
        // let created = actions::create_book(&conn, new_book.into_inner())?;
        // Ok(HttpResponse::Created().json(created))
    }

    pub async fn get_one(
        app: web::Data<AppState>,
        authentication: Authentication,
        search_id: web::Path<i32>,
    ) -> MyResponder {
        let member_id = authentication.requires_any_member()?;
        let conn = app.open_database_connection()?;
        todo!("")
        // let title = actions::find_book(&conn, *search_id)?;
        // Ok(HttpResponse::Ok().json(title))
    }

    pub async fn put(
        app: web::Data<AppState>,
        authentication: Authentication,
        write_to_id: web::Path<i32>,
        new_info: web::Json<NewBook>,
    ) -> MyResponder {
        let member_id = authentication.requires_any_member()?;
        let conn = app.open_database_connection()?;
        todo!("")
        // let updated = actions::update_book(&conn, *write_to_id, new_info.into_inner())?;
        // Ok(HttpResponse::Ok().json(updated))
    }

    pub async fn delete(
        app: web::Data<AppState>,
        authentication: Authentication,
        delete_id: web::Path<i32>,
    ) -> MyResponder {
        let member_id = authentication.requires_any_member()?;
        let conn = app.open_database_connection()?;
        todo!("")
        // let updated = actions::delete_book(&conn, *delete_id)?;
        // Ok(HttpResponse::Ok().json(updated))
    }


}
