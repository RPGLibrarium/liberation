#[macro_use]
extern crate log;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

mod api;
mod auth;
mod business;
mod database;
mod error;
mod serde_formats;
mod settings;

use actix_web::{web, App, HttpServer};
use api::{get_static, get_v1};
use auth::KeycloakCache;
use settings::Settings;
use actix_web::middleware::Logger;
use actix::{System, Actor};
use mysql::prelude::TextQuery;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("retrieving settings ...");
    let settings = Settings::new().unwrap();
    info!("initializing DB ...");
    let db = database::Database::from_settings(&settings.database).unwrap();

    info!("initializing keycloak ...");
    let kc: KeycloakCache = KeycloakCache::new();
    let kc_actor = auth::Keycloak::from_settings(&settings.keycloak, kc.clone());

   let state = api::AppState {
        db,
        kc: kc.clone(),
    };

    let serve_static_files = settings.serve_static_files;
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .data(state.clone())
            .service(get_v1());
        if serve_static_files {
            app = app.service(get_static());
        }
        app
    })
        .bind(format!("0.0.0.0:{}", settings.port))?
        .run()
        .await
}

