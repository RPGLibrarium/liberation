use actix_web::{server, App, HttpRequest, HttpMessage, HttpResponse, Responder, Result, http, Json, AsyncResponder};
use database::Database;
use futures::future::Future;
use dtos;
use dmos;
use error;

#[derive(Clone)]
pub struct AppState{
    pub db: Database
}

pub fn get_v1(state: AppState) -> Box<server::HttpHandler> {
    App::with_state(state)
    .prefix("/v1")
    .route("/rpgsystems", http::Method::GET, get_rpg_systems)
    .route("/rpgsystem/{systemid}", http::Method::GET, get_rpg_system)
    .route("/rpgsystems", http::Method::POST, post_rpg_system)
    .route("/rpgsystem/{systemid}", http::Method::PUT, put_rpg_system)

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
    .route("/members/{memberid}/inventory", http::Method::GET, get_member_inventory)
    .route("/members/{memberid}/inventory", http::Method::POST, post_member_inventory)

    .route("/guilds", http::Method::GET, get_guilds)
    .route("/guilds/{guildid}", http::Method::GET, get_guild)
    .route("/guilds", http::Method::POST, post_guild)
    .route("/guilds/{guildid}", http::Method::PUT, put_guild)
    .route("/guilds/{guildid}/inventory", http::Method::GET, get_guild_inventory)
    .route("/guilds/{guildid}/inventory", http::Method::POST, post_guild_inventory)

    .boxed()
}

fn get_rpg_systems(_req: HttpRequest<AppState>) -> impl Responder {
    _req.state().db.get_rpg_systems()
        .and_then(|systems| {
             Ok(Json(dtos::GetRpgSystems{rpgsystems: systems}))
        })
}

fn get_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {

    "GET RpgSystem by Id"
}

fn post_rpg_system(_req: HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = error::Error>> {
    let localdb = _req.state().db.clone();
    _req.json()
        .from_err()
        .and_then( move |obj : dtos::PutPostRpgSystem| {
            localdb.insert_rpg_system(obj.rpgsystem.name)
        }).and_then(|new_system|
            Ok(HttpResponse::Created().header("Location", format!("v1/rpgsystems/{}", new_system.id)).finish())
        ).responder()
}

fn put_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT RpgSystem"
}

fn get_titles(_req: HttpRequest<AppState>) -> impl Responder {
    "GET titles"
}
fn get_title(_req: HttpRequest<AppState>) -> impl Responder {
    "GET titles/<id>"
}
fn post_title(_req: HttpRequest<AppState>) -> impl Responder {
    "POST titles"
}
fn put_title(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT titles/<id>"
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
