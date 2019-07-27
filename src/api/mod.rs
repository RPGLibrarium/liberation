mod dto;

pub use self::dto::*;

use actix_files as fs;
use actix_service::IntoNewService;
use actix_web::body::Body;
use actix_web::dev::HttpServiceFactory;
use actix_web::error::DispatchError::Service;
use actix_web::web::Json;
use actix_web::{http, web, App, HttpMessage, HttpRequest, HttpResponse, Responder, Scope};
use auth::roles::*;
use auth::{assert_roles, Claims, KeycloakCache};
use business as bus;
use database::*;
use futures::future::Future;

/// Handling of external modules
#[derive(Clone)]
pub struct AppState {
    /// Underlaying database
    pub db: Database,
    /// Keycloak for authentification
    pub kc: KeycloakCache,
}

/// Getter for web folder
pub fn get_static() -> Scope {
    web::scope("/web").service(fs::Files::new("/", "./web").index_file("index.html"))
}

pub fn get_v1() -> Scope {
    web::scope("/v1")
        .service(get_rpg_systems)
        .service(get_rpg_system)
        .service(post_rpg_system)
        .service(put_rpg_system)
        .service(delete_rpg_system)
        .service(get_titles)
        .service(get_title)
        .service(post_title)
        .service(put_title)
        .service(get_books)
        .service(get_book)
        .service(post_book)
        .service(put_book)
        .service(get_members)
        .service(get_member)
        .service(get_member_inventory)
        .service(post_member_inventory)
        .service(get_guilds)
        .service(get_guild)
        .service(post_guild)
        .service(put_guild)
        .service(get_guild_inventory)
        .service(post_guild_inventory)
}

// Responder<Item = Into<AsyncResult<HttpResponse>>, Error = Into<Error>>
// - HttpResponse
// - Box<Future<Item = Responder, Error = Error>>
// - Json<T>
// https://actix.rs/actix-web/actix_web/trait.Responder.html

#[get("/rpgsystems")]
fn get_rpg_systems(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    assert_roles(&_req, vec![])?;

    return bus::get_rpgsystems(&state.db).and_then(|systems| Ok(HttpResponse::Ok().json(systems)));
    // This works because of reasons:
    // Response<Json<T>, Into<Error>> = impl Response
}

#[get("/rpgsystems/{systemid}")]
fn get_rpg_system(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims: Option<Claims> = assert_roles(&_req, vec![])?;

    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;

    bus::get_rpgsystem(&state.db, claims, id).and_then(|system| Ok(HttpResponse::Ok().json(system)))
}

#[post("/rpgsystems")]
fn post_rpg_system(
    state: AppState,
    json: web::Json<PutPostRpgSystem>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims: Option<Claims> =
        assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = state.db.clone();

    bus::post_rpgsystem(&localdb, claims, json.into_inner()).and_then(|system_id| {
        Ok(HttpResponse::Created()
            .header("Location", format!("v1/rpgsystems/{}", system_id))
            .finish())
    })
}

#[put("/rpgsystems/{systemid}")]
fn put_rpg_system(
    state: AppState,
    json: web::Json<PutPostRpgSystem>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = state.db.clone();
    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;
    let mut rpg_system = json.into_inner();
    rpg_system.rpgsystem.id = Some(id);
    bus::put_rpgsystem(&localdb, claims, &system).and_then(|()| Ok(HttpResponse::Ok().finish()))
}

#[delete("/rpgsystems/{systemid}")]
fn delete_rpg_system(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;

    bus::delete_rpgsystem(&state.db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Titles (if authentification is successful)
#[get("/titles")]
fn get_titles(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    assert_roles(&_req, vec![])?;

    bus::get_titles(&state.db).and_then(|titles| Ok(HttpResponse::Ok().json(titles)))
}

/// Get a requested Title (if authentification is successful)
#[get("/titles/{titleid}")]
fn get_title(state: AppState, _req: HttpRequest) -> HttpResponse {
    let claims = assert_roles(&_req, vec![])?;

    let id: TitleId = _req.match_info().query("titleid").parse::<TitleId>()?;

    bus::get_title(&state.db, claims, id).and_then(|title| Ok(HttpResponse::Ok().json(title)))
}

/// Insert a new Title (if authentification is successful)
#[post("/titles")]
fn post_title(
    state: AppState,
    json: web::Json<PutPostTitle>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = state.db.clone();

    bus::post_title(&localdb, claims, json.into_inner()).and_then(|title_id| {
        Ok(HttpResponse::Created()
            .header("Location", format!("v1/titles/{}", title_id))
            .finish())
    })
}

/// Update an existing Title (if authentification is successful)
#[put("/titles/{titleid}")]
fn put_title(
    state: AppState,
    json: web::Json<PutPostTitle>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = state.db.clone();
    let id: TitleId = _req.match_info().query("titleid").parse::<TitleId>()?;

    let mut title = json.into_inner();
    title.title.id = Some(id);

    bus::put_title(&localdb, claims, title).and_then(|()| Ok(HttpResponse::Ok().finish()))
}

#[delete("/titles/{titleid}")]
fn delete_title(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: TitleId = _req.match_info().query("titleid").parse::<TitleId>()?;

    bus::delete_title(&state.db, claims, id).and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Books (if authentification is successful)
#[get("/books")]
fn get_books(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    bus::get_books(&state.db, claims).and_then(|books| Ok(HttpResponse::Ok().json(books)))
}

/// Get a requested Book (if authentification is successful)
#[get("/books/{bookid}")]
fn get_book(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let id: BookId = _req.match_info().query("bookid").parse::<BookId>()?;

    bus::get_book(&state.db, claims, id).and_then(|book| Ok(HttpResponse::Ok().json(book)))
}

/// Insert a new Book (if authentification is successful)
#[post("/books")]
fn post_book(
    state: AppState,
    json: web::Json<PutPostBook>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = state.db.clone();

    bus::post_book(&localdb, claims, json.into_inner()).and_then(|book_id| {
        Ok(HttpResponse::Created()
            .header("Location", format!("v1/books/{}", book_id))
            .finish())
    })
}

/// Update an existing Book (if authentification is successful)
#[put("/books/{bookid}")]
fn put_book(
    state: AppState,
    json: web::Json<PutPostBook>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let localdb = state.db.clone();
    let id: BookId = _req.match_info().query("bookid").parse::<BookId>()?;

    let mut book = json.into_inner();
    book.book.id = Some(id);

    bus::put_book(&localdb, claims, book).and_then(|()| Ok(HttpResponse::Ok().finish()))
}

#[delete("/books/{bookid}")]
fn delete_book(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: BookId = _req.match_info().query("bookid").parse::<BookId>()?;

    bus::delete_book(&state.db, claims, id).and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Members (if authentification is successful)
#[get("/members")]
fn get_members(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    bus::get_members(&state.db, claims).and_then(|members| Ok(HttpResponse::Ok().json(members)))
}

/// Get a requested Member (if authentification is successful)
#[get("/members/{memberid}")]
fn get_member(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    let id: MemberId = _req.match_info().query("memberid").parse::<MemberId>()?;

    bus::get_member(&state.db, claims, id).and_then(|member| Ok(HttpResponse::Ok().json(member)))
}

/// Get the inventory of a Member (if authentification is successful)
#[get("/members/{memberid}/inventory")]
fn get_member_inventory(state: AppState, _req: HttpRequest) -> HttpResponse {
    "GET members/<id>/inventory"
}

/// Insert into a member's inventory (if authentification is successful)
#[post("/members/{memberid}/inventory")]
fn post_member_inventory(state: AppState, _req: HttpRequest) -> HttpResponse {
    "POST members/<id>/inventory"
}

/// Get all Guilds (if authentification is successful)
#[get("/guilds")]
fn get_guilds(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    bus::get_guilds(&state.db, claims).and_then(|guilds| Ok(HttpResponse::Ok().json(guilds)))
}

/// Get a requested Guild (if authentification is successful)
#[get("/guilds/{guildid}")]
fn get_guild(state: AppState, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    let id: GuildId = _req.match_info().query("guildid").parse::<GuildId>()?;

    bus::get_guild(&state.db, claims, id).and_then(|guild| Ok(HttpResponse::Ok().json(guild)))
}

/// Insert a new Guild (if authentification is successful)
#[post("/guilds")]
fn post_guild(
    state: AppState,
    json: web::Json<PutPostGuild>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT])?;

    let localdb = state.db.clone();

    bus::post_guild(&localdb, claims, json.into_inner()).and_then(|id| {
        Ok(HttpResponse::Created()
            .header("Location", format!("v1/guilds/{}", id))
            .finish())
    })
}

/// Update an existing Guild (if authentification is successful)
#[put("/guilds/{guildid}")]
fn put_guild(
    state: AppState,
    json: web::Json<PutPostGuild>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT])?;

    let localdb = state.db.clone();
    let id: GuildId = _req.match_info().query("guildid").parse::<GuildId>()?;

    let mut guild = json.into_inner();
    guild.guild.id = Some(id);

    bus::put_guild(&localdb, claims, guild).and_then(|()| Ok(HttpResponse::Ok().finish()))
}

/// Get the inventory of a Guild (if authentification is successful)
#[get("/guilds/{guildid}/inventory")]
fn get_guild_inventory(state: AppState, _req: HttpRequest) -> HttpResponse {
    "GET Guild inventory by Id"
}

/// Insert into a guild's inventory (if authentification is successful)
#[post("/guilds/{guildid}/inventory")]
fn post_guild_inventory(state: AppState, _req: HttpRequest) -> HttpResponse {
    "POST Guild Inventory"
}
