use crate::actions;
use crate::actions::AccountAssertions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::authentication::scopes::{ACCOUNT_READ, ACCOUNT_REGISTER};
use crate::authentication::Claims;
use crate::models::{AccountActive, NewAccount};
use actix_web::web::Json;
use actix_web::{web, HttpResponse};

/// Get the account information of the currently authenticated user.
pub async fn get(app: web::Data<AppState>, claims: Claims) -> MyResponder {
    claims.require_scope(ACCOUNT_READ)?;
    let external_id = claims.external_account_id()?;
    let conn = app.open_database_connection()?;
    let account =
        actions::account::try_find_by_external_id(&conn, external_id)?.assert_registered()?;
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

    let account = actions::account::create(&conn, new_account)?;
    Ok(HttpResponse::Created().json(account))
}

/// Update an account for the currently authenticated user. Can be used to de-/reactivate an
/// account.
pub async fn put(
    app: web::Data<AppState>,
    claims: Claims,
    body: Json<AccountActive>,
) -> MyResponder {
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

    match actions::account::try_find_by_external_id(&conn, external_id)? {
        Some(old_account) => {
            let updated = actions::account::update(&conn, old_account.id, updated_account)?;
            Ok(HttpResponse::Ok().json(updated))
        }
        None => {
            let new = actions::account::create(&conn, updated_account)?;
            Ok(HttpResponse::Created().json(new))
        }
    }
}

// Users can't delete their account, for security reasons. An an aristocrat can though.

pub mod collection {
    use crate::actions;
    use crate::actions::AccountAssertions;
    use crate::api::MyResponder;
    use crate::app::AppState;
    use crate::authentication::scopes::{COLLECTION_MODIFY, COLLECTION_READ};
    use crate::authentication::Claims;
    use crate::models::{PostOwnedBook, QueryOptions};
    use actix_web::{web, HttpResponse,};

    /// Displays the collection of the authenticated user.
    pub async fn get_all(
        app: web::Data<AppState>,
        claims: Claims,
        query: web::Query<QueryOptions>,
    ) -> MyResponder {
        claims.require_scope(COLLECTION_READ)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account =
            actions::account::try_find_by_external_id(&conn, external_id)?.assert_active()?;
        if query.recursive {
            let books = actions::book::double_recursive_list_owned_by(&conn, account.into())?;
            Ok(HttpResponse::Ok().json(books))
        } else {
            let books = actions::book::list_owned_by(&conn, account.into())?;
            Ok(HttpResponse::Ok().json(books))
        }
    }

    pub async fn post(
        app: web::Data<AppState>,
        claims: Claims,
        posted_book: web::Json<PostOwnedBook>,
    ) -> MyResponder {
        claims.require_scope(COLLECTION_MODIFY)?;
        let external_id = claims.external_account_id()?;
        let conn = app.open_database_connection()?;
        let account =
            actions::account::try_find_by_external_id(&conn, external_id)?.assert_active()?;
        let conn_argument = &conn;
        let created_book = actions::book::create_owned_by(
            &conn_argument,
            account.into(),
            posted_book.into_inner(),
        )?;
        Ok(HttpResponse::Created().json(created_book))
    }
}
