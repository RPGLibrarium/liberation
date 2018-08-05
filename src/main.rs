#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate chrono;
extern crate config;
extern crate failure;
extern crate futures;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod api;
mod auth;
#[macro_use]
mod database;
mod business;
mod error;
mod serde_formats;
mod settings;

use actix_web::{server, App, HttpRequest};
use settings::Settings;

fn main() {
    let settings = Settings::new().unwrap();
    let db = database::Database::from_settings(&settings.database).unwrap();

    let state = api::AppState { db: db };
    server::new(move || vec![api::get_v1(state.clone())])
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
