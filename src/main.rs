#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate chrono;
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

use actix_web::{server, App, HttpRequest};

fn main() {
    let db = database::Database::new(String::from(
        "mysql://root:thereIsNoPassword!@127.0.1.1:33061/liberation",
    )).unwrap();

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
