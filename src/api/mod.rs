mod dto;

pub use self::dto::*;

use actix_web::error as actix_error;
use actix_web::{
    http, server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, Json,
    Responder, ResponseError, Result,
};
use auth::Token;
use database::*;
use error;
use futures::future::Future;

use business as bus;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

pub fn get_v1(state: AppState) -> Box<server::HttpHandler> {
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
        )
        .route("/titles", http::Method::GET, get_titles)
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
        )
        .route(
            "/members/{memberid}/inventory",
            http::Method::POST,
            post_member_inventory,
        )
        .route("/guilds", http::Method::GET, get_guilds)
        .route("/guilds/{guildid}", http::Method::GET, get_guild)
        .route("/guilds", http::Method::POST, post_guild)
        .route("/guilds/{guildid}", http::Method::PUT, put_guild)
        .route(
            "/guilds/{guildid}/inventory",
            http::Method::GET,
            get_guild_inventory,
        )
        .route(
            "/guilds/{guildid}/inventory",
            http::Method::POST,
            post_guild_inventory,
        )
        .boxed()
}

fn get_rpg_systems(_req: HttpRequest<AppState>) -> impl Responder {
    bus::get_rpgsystems(&_req.state().db, Token {}).and_then(|systems| Ok(Json(systems)))
}

// fn get_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
//     "GET rpg_system"
// }
fn get_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder> {
    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::get_rpgsystem(&_req.state().db, Token {}, id)
        .and_then(|system| Ok(Json(system)))
        .map_err(Error::from)
}

// fn post_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
//     "POST rpg_system"
// }
fn post_rpg_system(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let localdb = _req.state().db.clone();
    _req.json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostRpgSystem| {
            bus::post_rpgsystem(&localdb, Token {}, &mut obj).map_err(Error::from)
        })
        .and_then(|system_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/rpgsystems/{}", system_id))
                .finish())
        })
        .map_err(Error::from)
        .responder()
}

// fn put_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
//     "PUT rpg_system"
// }
fn put_rpg_system(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let localdb = _req.state().db.clone();
    let id: Result<RpgSystemId> = _req.match_info()
        .query("systemid")
        .map_err(actix_error::ErrorBadRequest);

    _req.json()
        .from_err()
        .and_then(|mut obj: dto::PutPostRpgSystem| {
            obj.rpgsystem.id = Some(id?);
            return Ok(obj);
        })
        .and_then(move |system: PutPostRpgSystem| {
            bus::put_rpgsystem(&localdb, Token {}, &system).map_err(Error::from)
        })
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder()
}

fn delete_rpg_system(_req: HttpRequest<AppState>) -> Result<impl Responder> {
    let id: RpgSystemId = _req.match_info().query("systemid")?;

    bus::delete_rpgsystem(&_req.state().db, Token {}, id)
        .and_then(|_| Ok(HttpResponse::NoContent().finish()))
        .map_err(Error::from)
}

// fn get_titles(_req: HttpRequest<AppState>) -> impl Responder {
//     "GET titles"
//
fn get_titles(_req: HttpRequest<AppState>) -> impl Responder {
    bus::get_titles(&_req.state().db, Token {}).and_then(|titles| Ok(Json(titles)))
}

// fn get_title(_req: HttpRequest<AppState>) -> impl Responder {
//     "GET titles/<id>"
// }
fn get_title(_req: HttpRequest<AppState>) -> Result<impl Responder> {
    let id: TitleId = _req.match_info().query("titleid")?;

    bus::get_title(&_req.state().db, Token {}, id)
        .and_then(|title| Ok(Json(title)))
        .map_err(Error::from)
}

// fn post_title(_req: HttpRequest<AppState>) -> impl Responder {
//     "POST titles"
// }
fn post_title(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let localdb = _req.state().db.clone();
    _req.json()
        .from_err()
        .and_then(move |mut obj: dto::PutPostTitle| {
            bus::post_title(&localdb, Token {}, &mut obj).map_err(Error::from)
        })
        .and_then(|system_id| {
            Ok(HttpResponse::Created()
                .header("Location", format!("v1/titles/{}", system_id))
                .finish())
        })
        .map_err(Error::from)
        .responder()
}

// fn put_title(_req: HttpRequest<AppState>) -> impl Responder {
//     "PUT titles/<id>"
// }
fn put_title(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let localdb = _req.state().db.clone();
    let id: Result<TitleId> = _req.match_info()
        .query("titleid")
        .map_err(actix_error::ErrorBadRequest);

    _req.json()
        .from_err()
        .and_then(|mut obj: dto::PutPostTitle| {
            obj.title.id = Some(id?);
            return Ok(obj);
        })
        .and_then(move |title: PutPostTitle| {
            bus::put_title(&localdb, Token {}, &title).map_err(Error::from)
        })
        .and_then(|()| Ok(HttpResponse::Ok().finish()))
        .responder()
}

fn get_books(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Books"
}

fn get_book(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Book by Id"
}

fn post_book(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Book"
}

fn put_book(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT Book"
}

fn get_members(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members"
}

fn get_member(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members/<id>"
}

fn get_member_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "GET members/<id>/inventory"
}

fn post_member_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST members/<id>/inventory"
}

fn get_guilds(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guilds"
}

fn get_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guild by Id"
}

fn get_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "GET Guild inventory by Id"
}

fn post_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Guild"
}

fn put_guild(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT Guild"
}

fn post_guild_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST Guild Inventory"
}
