mod dto;

pub use self::dto::*;

use actix_web::error as actix_error;
use actix_web::server::HttpHandlerTask;
use actix_web::{
    fs, http, server, App, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Json, Responder,
    ResponseError, Result,
};
use auth::roles::*;
use auth::{assert_roles, get_claims_for_req, Claims, Keycloak, KeycloakCache};
use business as bus;
use database::*;
use error::Error;
use futures::future::{err, result, Future};
use std::sync::Arc;

/// Handling of external modules
#[derive(Clone)]
pub struct AppState {
    /// Underlaying database
    pub db: Database,
    /// Keycloak for authentification
    pub kc: KeycloakCache,
}

/// Getter for web folder
pub fn get_static() -> Box<dyn server::HttpHandler<Task = Box<HttpHandlerTask>>> {
    App::new()
        .prefix("/web")
        .handler(
            "/",
            fs::StaticFiles::new("./web")
                .unwrap()
                .index_file("index.html"),
        ).boxed()
}

pub fn get_v1(state: AppState) -> Box<dyn server::HttpHandler<Task = Box<HttpHandlerTask>>> {
    App::with_state(state)
        .prefix("/v1")
        .route("/rpgsystems", http::Method::GET, get_rpg_systems)
        .route("/rpgsystems/{systemid}", http::Method::GET, get_rpg_system)
        .route("/rpgsystems", http::Method::POST, post_rpg_system)
        .route("/rpgsystems/{systemid}", http::Method::PUT, put_rpg_system)
        .route(
            "/rpgsystems/{systemid}",
            http::Method::DELETE,
            delete_rpg_system,
        ).route("/titles", http::Method::GET, get_titles)
        .route("/titles/{titleid}", http::Method::GET, get_title)
        .route("/titles", http::Method::POST, post_title)
        .route("/titles/{titleid}", http::Method::PUT, put_title)
        .route("/books", http::Method::GET, get_books)
        .route("/books/{bookid}", http::Method::GET, get_book)
        .route("/books", http::Method::POST, post_book)
        .route("/books/{bookid}", http::Method::PUT, put_book)
        .route("/members", http::Method::GET, get_members)
        .route("/members/{memberid}", http::Method::GET, get_member)
        .route(
            "/members/{memberid}/inventory",
            http::Method::GET,
            get_member_inventory,
        ).route(
            "/members/{memberid}/inventory",
            http::Method::POST,
            post_member_inventory,
        ).route("/guilds", http::Method::GET, get_guilds)
        .route("/guilds/{guildid}", http::Method::GET, get_guild)
        .route("/guilds", http::Method::POST, post_guild)
        .route("/guilds/{guildid}", http::Method::PUT, put_guild)
        .route(
            "/guilds/{guildid}/inventory",
            http::Method::GET,
            get_guild_inventory,
        ).route(
            "/guilds/{guildid}/inventory",
            http::Method::POST,
            post_guild_inventory,
        ).boxed()
}

// Responder<Item = Into<AsyncResult<HttpResponse>>, Error = Into<Error>>
// - HttpResponse
// - Box<Future<Item = Responder, Error = Error>>
// - Json<T>
// https://actix.rs/actix-web/actix_web/trait.Responder.html

/// Get all RpgSystems (if authentification is successful)
fn get_rpg_systems(_req: HttpRequest<AppState>) -> impl Responder {
    assert_roles(&_req, vec![])?;

    return bus::get_rpgsystems(&_req.state().db, &_req.state().kc)
        .and_then(|systems| Ok(Json(systems)));
    // This works because of reasons:
    // Response<Json<T>, Into<Error>> = impl Response
}

/// Get a requested RpgSystem (if authentification is successful)
fn get_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    assert_roles(&_req, vec![])?;

    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::get_rpgsystem(&_req.state().db, id).and_then(|system| Ok(Json(system)))
}

/// Insert a new RpgSystem (if authentification is successful)
fn post_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = _req.state().db.clone();
    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostRpgSystem| {
            bus::post_rpgsystem(&localdb, claims, &mut obj)
        }).and_then(|system_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/rpgsystems/{}", system_id))
                .finish())
        }).responder())
}

/// Update an existing RpgSystem (if authentification is successful)
fn put_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = _req.state().db.clone();
    let id: RpgSystemId = _req.match_info().query("systemid")?;

    Ok(_req
        .json()
        .from_err()
        .and_then(|mut obj: dto::PutPostRpgSystem| {
            obj.rpgsystem.id = Some(id);
            return Ok(obj);
        }).and_then(move |system: PutPostRpgSystem| bus::put_rpgsystem(&localdb, claims, &system))
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder())
}

/// Delete a given RpgSystem (if authentification is successful)
fn delete_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = _req.state().db.clone();
    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::delete_rpgsystem(&_req.state().db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Titles (if authentification is successful)
fn get_titles(_req: HttpRequest<AppState>) -> impl Responder {
    bus::get_titles(&_req.state().db).and_then(|titles| Ok(Json(titles)))
}

/// Get a requested Title (if authentification is successful)
fn get_title(_req: HttpRequest<AppState>) -> impl Responder {
    let id: TitleId = _req.match_info().query("titleid")?;
    let claims = get_claims_for_req(&_req)?;
    bus::get_title(&_req.state().db, id, claims)
        .and_then(|title| Ok(Json(title)))
        .map_err(Error::from)
}

/// Insert a new Title (if authentification is successful)
fn post_title(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let claims = match get_claims_for_req(&_req) {
        Err(_) => return Box::new(err(Error::from(error::Error::InvalidAuthenticationError))),
        Ok(c) => c,
    };
    let is_allowed = check_roles(&claims, vec![ROLE_ADMIN, ROLE_LIBRARIAN]);
    let localdb = _req.state().db.clone();
    _req.json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostTitle| {
            bus::post_title(&localdb, claims, &mut obj).map_err(Error::from)
        }).and_then(|system_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/titles/{}", system_id))
                .finish())
        }).map_err(Error::from)
        .responder()
}

/// Update an existing Title (if authentification is successful)
fn put_title(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let claims = match get_claims_for_req(&_req) {
        Err(_) => return Box::new(err(Error::from(error::Error::InvalidAuthenticationError))),
        Ok(c) => c,
    };
    let is_allowed = check_roles(&claims, vec![ROLE_ADMIN, ROLE_LIBRARIAN]);
    let localdb = _req.state().db.clone();
    let id: Result<TitleId> = _req
        .match_info()
        .query("titleid")
        .map_err(actix_error::ErrorBadRequest);

    _req.json()
        .from_err()
        .and_then(|mut obj: dto::PutPostTitle| {
            obj.title.id = Some(id?);
            return Ok(obj);
        }).and_then(move |title: PutPostTitle| {
            bus::put_title(&localdb, claims, &title).map_err(Error::from)
        }).and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder()
}

/// Get all Books (if authentification is successful)
fn get_books(_req: HttpRequest<AppState>) -> impl Responder {
    let claims = get_claims_for_req(&_req)?;
    // TODO roles
    let is_allowed = check_roles(&claims, vec![ROLE_ADMIN, ROLE_LIBRARIAN]);
    bus::get_books(&_req.state().db, claims).and_then(|books| Ok(Json(books)))
}

/// Get a requested Book (if authentification is successful)
fn get_book(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Book by Id"
}

/// Insert a new Book (if authentification is successful)
fn post_book(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Book"
}

/// Update an existing Book (if authentification is successful)
fn put_book(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT Book"
}

/// Get all Members (if authentification is successful)
fn get_members(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members"
}

/// Get a requested Member (if authentification is successful)
fn get_member(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members/<id>"
}

/// Get the inventory of a Member (if authentification is successful)
fn get_member_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members/<id>/inventory"
}

/// Insert into a member's inventory (if authentification is successful)
fn post_member_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST members/<id>/inventory"
}

/// Get all Guilds (if authentification is successful)
fn get_guilds(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guilds"
}

/// Get a requested Guild (if authentification is successful)
fn get_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guild by Id"
}

/// Get the inventory of a Guild (if authentification is successful)
fn get_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guild inventory by Id"
}

/// Insert a new Guild (if authentification is successful)
fn post_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Guild"
}

/// Update an existing Guild (if authentification is successful)
fn put_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT Guild"
}

/// Insert into a guild's inventory (if authentification is successful)
fn post_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Guild Inventory"
}
