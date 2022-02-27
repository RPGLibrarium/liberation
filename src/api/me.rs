use actix_web::{HttpResponse, web};
use actix_web::web::Json;
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::Claims;
use crate::authentication::scopes::{ACCOUNT_READ, ACCOUNT_REGISTER};
use crate::models::{AccountActive, NewAccount};

/// Get the account information of the currently authenticated user.
pub async fn get(app: web::Data<AppState>, claims: Claims) -> MyResponder {
    claims.require_scope(ACCOUNT_READ)?;
    let external_id = claims.external_account_id()?;
    let conn = app.open_database_connection()?;
    let account = actions::find_account_by_external_id(&conn, external_id)?;
    Ok(HttpResponse::Ok().json(account))
}

/// Register an account for the currently authenticated user.
pub async fn post(app: web::Data<AppState>, claims: Claims) -> MyResponder {
    claims.require_scope(ACCOUNT_REGISTER)?;
    let account_info = claims.account_info()?;
    let external_id = claims.external_account_id()?;
    let conn = app.open_database_connection()?;

    let new_account = NewAccount {
        active: true,
        external_id,
        full_name: account_info.full_name,
        given_name: account_info.given_name,
        family_name: account_info.family_name,
        email: account_info.email,
    };

    let account = actions::create_account(&conn, new_account)?;
    Ok(HttpResponse::Created().json(account))
}

/// Update an account for the currently authenticated user. Can be used to de-/reactivate an
/// account.
pub async fn patch(app: web::Data<AppState>, claims: Claims, body: Json<AccountActive>) -> MyResponder {
    claims.require_scope(ACCOUNT_REGISTER)?;
    let account_info = claims.account_info()?;
    let external_id = claims.external_account_id()?;
    let conn = app.open_database_connection()?;

    let updated_account = NewAccount {
        active: body.into_inner().is_active,
        external_id: external_id.clone(),
        full_name: account_info.full_name,
        given_name: account_info.given_name,
        family_name: account_info.family_name,
        email: account_info.email,
    };

    let old_account = actions::find_account_by_external_id(&conn, external_id)?;
    let account = actions::update_account(&conn, old_account.account_id, updated_account)?;
    Ok(HttpResponse::Created().json(account))
}

// Users can't delete their account, for security reasons. An an aristocrat can though.

pub mod collection {
    use actix_web::{HttpResponse, web};
    use crate::actions;
    use crate::actions::{delete_book_owned_by_member, find_book_owned_by_member};
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::authentication::Claims;
    use crate::authentication::scopes::{COLLECTION_MODIFY, COLLECTION_READ};
    use crate::error::UserFacingError;
    use crate::models::{PostOwnedBook};

    /// Displays the collection of the authenticated user.
    pub async fn get_all(app: web::Data<AppState>, claims: Claims) -> MyResponder {
        claims.require_scope(COLLECTION_READ)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account = actions::find_account_by_external_id(&conn, external_id)?;
        if !account.active { return Err(UserFacingError::Deactivated); }
        let books = actions::list_books_owned_by_member(&conn, account)?;
        Ok(HttpResponse::Ok().json(books))
    }

    pub async fn post(
        app: web::Data<AppState>,
        claims: Claims,
        posted_book: web::Json<PostOwnedBook>,
    ) -> MyResponder {
        claims.require_scope(COLLECTION_MODIFY)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account = actions::find_account_by_external_id(&conn, external_id)?;
        if !account.active { return Err(UserFacingError::Deactivated); }
        let created_book = actions::create_book_owned_by_member(&conn, account, posted_book.into_inner())?;
        Ok(HttpResponse::Created().json(created_book))
    }

    pub async fn get_one(
        app: web::Data<AppState>,
        claims: Claims,
        search_id: web::Path<i32>,
    ) -> MyResponder {
        claims.require_scope(COLLECTION_READ)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account = actions::find_account_by_external_id(&conn, external_id)?;
        if !account.active { return Err(UserFacingError::Deactivated); }
        let book = find_book_owned_by_member(&conn, account, *search_id)?;
        Ok(HttpResponse::Created().json(book))
    }

    pub async fn delete(
        app: web::Data<AppState>,
        claims: Claims,
        delete_id: web::Path<i32>,
    ) -> MyResponder {
        claims.require_scope(COLLECTION_MODIFY)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account = actions::find_account_by_external_id(&conn, external_id)?;
        if !account.active { return Err(UserFacingError::Deactivated); }
        delete_book_owned_by_member(&conn, &account, *delete_id)?;
        Ok(HttpResponse::Ok().finish())
    }
}
