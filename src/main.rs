#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate base64;
extern crate chrono;
extern crate config;
extern crate core;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate oauth2;
extern crate openssl;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
extern crate url_serde;

mod api;
mod auth;
mod business;
mod database;
mod error;
mod serde_formats;
mod settings;

use actix::{Actor, System};
use actix_web::server;
use auth::KeycloakCache;
use settings::Settings;

fn main() {
    env_logger::init();

    info!("retrieving settings ...");
    let settings = Settings::new().unwrap();
    info!("initializing DB ...");
    let db = database::Database::from_settings(&settings.database).unwrap();

    info!("initializing keycloak ...");
    let kc: KeycloakCache = KeycloakCache::new();
    let kc_actor = auth::Keycloak::from_settings(&settings.keycloak, kc.clone());

    let state = api::AppState {
        db: db,
        kc: kc.clone(),
    };

    let sys = System::new("server");
    kc_actor.start();

    server::new(move || vec![api::get_v1(state.clone()), api::get_static()])
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    sys.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
