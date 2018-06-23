#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate chrono;
extern crate actix_web;
extern crate failure;
extern crate futures;

mod dmos;
mod dtos;
mod database;
mod error;
mod api;
mod serde_formats;
mod auth;

use actix_web::{server, App, HttpRequest};


fn main() {
    let db = database::Database::new(String::from("mysql://root:thereIsNoPassword!@127.0.1.1:33061/liberation")).unwrap();

    let state = api::AppState{db: db};
    server::new(move || vec![api::get_v1(state.clone())])
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
