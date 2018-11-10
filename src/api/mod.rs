mod dto;

pub use self::dto::*;

use actix_web::error::Error as ActixError;
use actix_web::error::InternalError;

use actix_web::server::HttpHandlerTask;
use actix_web::{
    fs, http, server, App, AsyncResponder, HttpMessage, HttpRequest, HttpResponse, Json, Responder,
};
use auth::roles::*;
use auth::{assert_roles, Claims, KeycloakCache};
use business as bus;
use database::*;
use futures::future::Future;

use core::num::ParseIntError;

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
fn get_rpg_systems(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    assert_roles(&_req, vec![])?;

    return bus::get_rpgsystems(&_req.state().db, &_req.state().kc)
        .and_then(|systems| Ok(Json(systems)));
    // This works because of reasons:
    // Response<Json<T>, Into<Error>> = impl Response
}

/// Get a requested RpgSystem (if authentification is successful)
fn get_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    assert_roles(&_req, vec![])?;

    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::get_rpgsystem(&_req.state().db, id).and_then(|system| Ok(Json(system)))
}

/// Insert a new RpgSystem (if authentification is successful)
fn post_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims: Option<Claims> =
        assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

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
fn put_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = _req.state().db.clone();
    let id: RpgSystemId = _req.match_info().query("systemid")?;

    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostRpgSystem| {
            obj.rpgsystem.id = Some(id);
            return Ok(obj);
        }).and_then(move |system: PutPostRpgSystem| bus::put_rpgsystem(&localdb, claims, &system))
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder())
}

/// Delete a given RpgSystem (if authentification is successful)
fn delete_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::delete_rpgsystem(&_req.state().db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Titles (if authentification is successful)
fn get_titles(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    assert_roles(&_req, vec![])?;

    bus::get_titles(&_req.state().db).and_then(|titles| Ok(Json(titles)))
}

/// Get a requested Title (if authentification is successful)
fn get_title(_req: HttpRequest<AppState>) -> impl Responder {
    let claims = assert_roles(&_req, vec![])?;

    let id: TitleId = _req.match_info().query("titleid")?;

    bus::get_title(&_req.state().db, claims, id).and_then(|title| Ok(Json(title)))
}

/// Insert a new Title (if authentification is successful)
fn post_title(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = _req.state().db.clone();
    Ok(_req
        .json()
        .from_err()
        .and_then(move |obj: dto::PutPostTitle| bus::post_title(&localdb, claims, obj))
        .and_then(|title_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/titles/{}", title_id))
                .finish())
        }).responder())
}

/// Update an existing Title (if authentification is successful)
fn put_title(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = _req.state().db.clone();
    let id: TitleId = _req.match_info().query("titleid")?;

    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostTitle| {
            obj.title.id = Some(id);
            Ok(obj)
        }).and_then(move |title: PutPostTitle| bus::put_title(&localdb, claims, title))
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder())
}

fn delete_title(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: TitleId = _req.match_info().query("titleid")?;

    bus::delete_title(&_req.state().db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Books (if authentification is successful)
fn get_books(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    bus::get_books(&_req.state().db, claims).and_then(|books| Ok(Json(books)))
}

/// Get a requested Book (if authentification is successful)
fn get_book(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let id: BookId = _req.match_info().query("bookid")?;

    bus::get_book(&_req.state().db, claims, id).and_then(|book| Ok(Json(book)))
}

/// Insert a new Book (if authentification is successful)
fn post_book(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = _req.state().db.clone();
    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostBook| bus::post_book(&localdb, claims, obj))
        .and_then(|book_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/books/{}", book_id))
                .finish())
        }).responder())
}

/// Update an existing Book (if authentification is successful)
fn put_book(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = _req.state().db.clone();
    let id: BookId = _req.match_info().query("bookid")?;

    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: PutPostBook| {
            obj.book.id = Some(id);
            Ok(obj)
        }).and_then(move |book: PutPostBook| bus::put_book(&localdb, claims, book))
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder())
}

fn delete_book(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: BookId = _req.match_info().query("bookid")?;

    bus::delete_book(&_req.state().db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Members (if authentification is successful)
fn get_members(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    bus::get_members(&_req.state().db, claims).and_then(|members| Ok(Json(members)))
}

/// Get a requested Member (if authentification is successful)
fn get_member(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    let id: MemberId = _req.match_info().query("memberid")?;

    bus::get_member(&_req.state().db, claims, id).and_then(|member| Ok(Json(member)))
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
fn get_guilds(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    bus::get_guilds(&_req.state().db, claims).and_then(|guilds| Ok(Json(guilds)))
}

/// Get a requested Guild (if authentification is successful)
fn get_guild(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    let id: GuildId = _req.match_info().query("guildid")?;

    bus::get_guild(&_req.state().db, claims, id).and_then(|guild| Ok(Json(guild)))
}

/// Insert a new Guild (if authentification is successful)
fn post_guild(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT])?;

    let localdb = _req.state().db.clone();

    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostGuild| bus::post_guild(&localdb, claims, obj))
        .and_then(|id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/guilds/{}", id))
                .finish())
        }).responder())
}

/// Update an existing Guild (if authentification is successful)
fn put_guild(_req: HttpRequest<AppState>) -> Result<impl Responder, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT])?;

    let localdb = _req.state().db.clone();
    let id: GuildId = _req.match_info().query("guildid")?;

    Ok(_req
        .json()
        .from_err()
        .and_then(move |mut obj: PutPostGuild| {
            obj.guild.id = Some(id);
            Ok(obj)
        }).and_then(move |guild: PutPostGuild| bus::put_guild(&localdb, claims, guild))
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder())
}

/// Get the inventory of a Guild (if authentification is successful)
fn get_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guild inventory by Id"
}

/// Insert into a guild's inventory (if authentification is successful)
fn post_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Guild Inventory"
}
