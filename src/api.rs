use actix_web::{server, App, HttpRequest};
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


fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
