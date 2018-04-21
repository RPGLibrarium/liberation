#[macro_use]
extern crate mysql;

mod dmos;
mod database;

fn main() {
    let db = database::Database::new(String::from("mysql://root:thereIsNoPassword!@172.18.0.2/liberation")).unwrap();

    let name = String::from("DSA 4.2");
    let system = db.insert_rpg_system(name).unwrap();
    println!("Inserted: {:?}", system);


}
