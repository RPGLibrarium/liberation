use actix_web::{HttpResponse, web};
use crate::actions;
use crate::api::MyResponder;
use crate::app::AppState;
use crate::auth::Authentication;
use crate::auth::roles::{GUILDS_EDIT, GUILDS_READ};
use crate::models::{Guild, NewGuild};

pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
    authentication.requires_role(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guilds = actions::list_guilds(&conn)?;
    Ok(HttpResponse::Ok().json(guilds))
}

pub async fn post(
    app: web::Data<AppState>,
    authentication: Authentication,
    new_guild: web::Json<NewGuild>,
) -> MyResponder {
    authentication.requires_role(GUILDS_EDIT)?;
    let conn = app.open_database_connection()?;
    let created = actions::create_guild(&conn, new_guild.into_inner())?;
    Ok(HttpResponse::Created().json(created))
}

pub async fn get_one(
    app: web::Data<AppState>,
    authentication: Authentication,
    search_id: web::Path<i32>,
) -> MyResponder {
    authentication.requires_role(GUILDS_READ)?;
    let conn = app.open_database_connection()?;
    let guild = actions::find_guild(&conn, *search_id)?;
    Ok(HttpResponse::Ok().json(guild))
}

pub async fn put(
    app: web::Data<AppState>,
    authentication: Authentication,
    write_to_id: web::Path<i32>,
    new_info: web::Json<NewGuild>,
) -> MyResponder {
    authentication.requires_role(GUILDS_EDIT)?;
    let conn = app.open_database_connection()?;
    let updated = actions::update_guild(&conn, *write_to_id, new_info.into_inner())?;
    Ok(HttpResponse::Ok().json(updated))
}
