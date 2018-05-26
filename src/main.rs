#[macro_use] extern crate mysql;
//#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate chrono;
extern crate actix_web;

mod dmos;
mod database;
mod error;
mod api;

use actix_web::{server, App, HttpRequest};


fn main() {
    let db = database::Database::new(String::from("mysql://root:thereIsNoPassword!@172.18.0.2/liberation")).unwrap();

    let state = api::AppState{db: db};
    server::new(move || vec![api::get_v1(state.clone())])
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
