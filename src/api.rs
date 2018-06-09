use actix_web::{server, App, HttpRequest, Responder};
use database::Database;

#[derive(Clone)]
pub struct AppState{
    pub db: Database
}

pub fn get_v1(state: AppState) -> Box<server::HttpHandler> {
    App::with_state(state)
    .prefix("/v1")
    .boxed()
}

fn get_rpg_systems(_req: HttpRequest<AppState>) -> impl Responder {
    "GET RpgSystem"
}

fn get_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    "GET RpgSystem by Id"
}

fn post_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    "POST RpgSystem"
}

fn put_rpg_system(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT RpgSystem"
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

fn put_Book(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT Book"
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
