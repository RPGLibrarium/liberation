#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate chrono;
extern crate config;
extern crate failure;
extern crate futures;
extern crate oauth2;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
extern crate url_serde;

mod api;
mod auth;
#[macro_use]
mod database;
mod business;
mod error;
mod serde_formats;
mod settings;

use actix_web::{actix, server, App, HttpRequest};
use settings::Settings;
use std::sync::Arc;

fn main() {
     env_logger::init();

    info!("retrieving settings ...");
    let settings = Settings::new().unwrap();
    info!("initializing DB ...");
    let db = database::Database::from_settings(&settings.database).unwrap();
    info!("initializing keycloak ...");
    let kclk = auth::Keycloak::from_settings(&settings.keycloak);

    let state = api::AppState {
        db: db,
        kc: Arc::new(kclk),
    };

    info!("starting http server ...");
    server::new(move || vec![api::get_v1(state.clone()), api::get_static()])
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
    info!("shutting down ... bye!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
