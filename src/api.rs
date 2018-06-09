use actix_web::{server, App, HttpRequest, Responder, http};
use database::Database;

#[derive(Clone)]
pub struct AppState{
    pub db: Database
}

pub fn get_v1(state: AppState) -> Box<server::HttpHandler> {
    App::with_state(state)
    .prefix("/v1")
    .resource("/", |r| r.f(index))
    .resource("/titles", |r| r.method(http::Method::GET).f(get_titles))
    .boxed()
}


fn index(_req: HttpRequest<AppState>) -> impl Responder {
    "Hello world!"
}

fn get_titles(_req: HttpRequest<AppState>) -> impl Responder {
    "GET titles"
}
fn get_title(_req: HttpRequest<AppState>) -> impl Responder {
    "GET titles/<id>"
}
fn create_title(_req: HttpRequest<AppState>) -> impl Responder {
    "POST titles"
}
fn update_title(_req: HttpRequest<AppState>) -> impl Responder {
    "PUT titles/<id>"
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
fn add_to_member_inventory(_req: HttpRequest<AppState>) -> impl Responder {
    "POST members/<id>/inventory"
}
