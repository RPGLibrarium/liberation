mod dto;

pub use self::dto::*;

use actix_files as fs;
use actix_service::IntoNewService;
use actix_web::body::Body;
use actix_web::dev::HttpServiceFactory;
use actix_web::error::DispatchError::Service;
use actix_web::web::{route, Json};
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
        .service(
            web::scope("/rpgsystems")
                .service(
                    web::resource("")
                        .route(web::get().to(get_rpg_systems))
                        .route(web::post().to(post_rpg_system)),
                )
                .service(
                    web::resource("/{systemid}")
                        .route(web::get().to(get_rpg_system))
                        .route(web::put().to(put_rpg_system))
                        .route(web::delete().to(delete_rpg_system)),
                ),
        )
        .service(
            web::scope("/titles")
                .service(
                    web::resource("")
                        .route(web::get().to(get_titles))
                        .route(web::post().to(post_rpg_system)),
                )
                .service(
                    web::resource("/{titleid}")
                        .route(web::get().to(get_title))
                        .route(web::put().to(put_title))
                        .route(web::delete().to(delete_title)),
                ),
        )
        .service(
            web::scope("/books")
                .service(
                    web::resource("")
                        .route(web::get().to(get_books))
                        .route(web::post().to(post_book)),
                )
                .service(
                    web::resource("/{bookid}")
                        .route(web::get().to(get_book))
                        .route(web::put().to(put_book))
                        .route(web::delete().to(delete_book)),
                ),
        )
        .service(
            web::scope("/guilds")
                .service(
                    web::resource("")
                        .route(web::get().to(get_guilds))
                        .route(web::post().to(post_guild)),
                )
                .service(
                    web::scope("/{guildid}")
                        .service(
                            web::resource("")
                                .route(web::get().to(get_guild))
                                .route(web::put().to(put_guild)),
                        )
                        .service(
                            web::resource("/inventory")
                                .route(web::get().to(get_guild_inventory))
                                .route(web::post().to(post_guild_inventory)),
                        ),
                ),
        )
        .service(
            web::scope("members")
                .service(web::resource("").route(web::get().to(get_members)))
                .service(
                    web::scope("/{memberid}")
                        .service(web::resource("/").route(web::get().to(get_member)))
                        .service(
                            web::resource("/inventory")
                                .route(web::get().to(get_member_inventory))
                                .route(web::post().to(post_member_inventory)),
                        ),
                ),
        )
}

// Responder<Item = Into<AsyncResult<HttpResponse>>, Error = Into<Error>>
// - HttpResponse
// - Box<Future<Item = Responder, Error = Error>>
// - Json<T>
// https://actix.rs/actix-web/actix_web/trait.Responder.html

fn get_rpg_systems(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    assert_roles(&_req, vec![])?;

    return bus::get_rpgsystems(&state.db).and_then(|systems| Ok(HttpResponse::Ok().json(systems)));
    // This works because of reasons:
    // Response<Json<T>, Into<Error>> = impl Response
}

fn get_rpg_system(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims: Option<Claims> = assert_roles(&_req, vec![])?;

    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;

    bus::get_rpgsystem(&state.db, claims, id).and_then(|system| Ok(HttpResponse::Ok().json(system)))
}

fn post_rpg_system(
    state: web::Data<AppState>,
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

fn put_rpg_system(
    state: web::Data<AppState>,
    json: web::Json<PutPostRpgSystem>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let localdb = state.db.clone();
    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;
    let mut rpg_system = json.into_inner();
    rpg_system.rpgsystem.id = Some(id);
    bus::put_rpgsystem(&localdb, claims, &rpg_system).and_then(|()| Ok(HttpResponse::Ok().finish()))
}

fn delete_rpg_system(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;

    bus::delete_rpgsystem(&state.db, claims, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Titles (if authentification is successful)
fn get_titles(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    assert_roles(&_req, vec![])?;

    bus::get_titles(&state.db).and_then(|titles| Ok(HttpResponse::Ok().json(titles)))
}

/// Get a requested Title (if authentification is successful)
fn get_title(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![])?;

    let id: TitleId = _req.match_info().query("titleid").parse::<TitleId>()?;

    bus::get_title(&state.db, claims, id).and_then(|title| Ok(HttpResponse::Ok().json(title)))
}

/// Insert a new Title (if authentification is successful)
fn post_title(
    state: web::Data<AppState>,
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
fn put_title(
    state: web::Data<AppState>,
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

fn delete_title(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: TitleId = _req.match_info().query("titleid").parse::<TitleId>()?;

    bus::delete_title(&state.db, claims, id).and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Books (if authentification is successful)
fn get_books(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    bus::get_books(&state.db, claims).and_then(|books| Ok(HttpResponse::Ok().json(books)))
}

/// Get a requested Book (if authentification is successful)
fn get_book(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN, ROLE_MEMBER])?;

    let id: BookId = _req.match_info().query("bookid").parse::<BookId>()?;

    bus::get_book(&state.db, claims, id).and_then(|book| Ok(HttpResponse::Ok().json(book)))
}

/// Insert a new Book (if authentification is successful)
fn post_book(
    state: web::Data<AppState>,
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
fn put_book(
    state: web::Data<AppState>,
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

fn delete_book(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_LIBRARIAN])?;

    let id: BookId = _req.match_info().query("bookid").parse::<BookId>()?;

    bus::delete_book(&state.db, claims, id).and_then(|_| Ok(HttpResponse::NoContent().finish()))
}

/// Get all Members (if authentification is successful)
fn get_members(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    bus::get_members(&state.db, claims).and_then(|members| Ok(HttpResponse::Ok().json(members)))
}

/// Get a requested Member (if authentification is successful)
fn get_member(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(
        &_req,
        vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_LIBRARIAN, ROLE_MEMBER],
    )?;

    let id: MemberId = _req.match_info().query("memberid").parse::<MemberId>()?;

    bus::get_member(&state.db, claims, id).and_then(|member| Ok(HttpResponse::Ok().json(member)))
}

/// Get the inventory of a Member (if authentification is successful)
fn get_member_inventory(state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

/// Insert into a member's inventory (if authentification is successful)
fn post_member_inventory(state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

/// Get all Guilds (if authentification is successful)
fn get_guilds(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    bus::get_guilds(&state.db, claims).and_then(|guilds| Ok(HttpResponse::Ok().json(guilds)))
}

/// Get a requested Guild (if authentification is successful)
fn get_guild(state: web::Data<AppState>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    let claims = assert_roles(&_req, vec![ROLE_ADMIN, ROLE_ARISTOCRAT, ROLE_MEMBER])?;

    let id: GuildId = _req.match_info().query("guildid").parse::<GuildId>()?;

    bus::get_guild(&state.db, claims, id).and_then(|guild| Ok(HttpResponse::Ok().json(guild)))
}

/// Insert a new Guild (if authentification is successful)
fn post_guild(
    state: web::Data<AppState>,
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
fn put_guild(
    state: web::Data<AppState>,
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
fn get_guild_inventory(state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}

/// Insert into a guild's inventory (if authentification is successful)
fn post_guild_inventory(state: web::Data<AppState>, _req: HttpRequest) -> HttpResponse {
    HttpResponse::NotImplemented().finish()
}
