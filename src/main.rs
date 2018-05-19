#[macro_use] extern crate mysql;
//#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate chrono;
mod dmos;
mod database;
mod error;

fn main() {
    let db = database::Database::new(String::from("mysql://root:thereIsNoPassword!@172.18.0.2/liberation")).unwrap();


    let name = String::from("DSA 4.1");
    let system = db.insert_rpg_system(name).unwrap();
    println!("Inserted: {:?}", system);


}
