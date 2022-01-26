use super::schema::rpg_systems;
use super::schema::titles;
use serde::Deserialize;
use serde::Serialize;

pub type Year = i16;

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "rpg_systems"]
#[primary_key(rpg_system_id)]
pub struct RpgSystem {
    pub rpg_system_id: i32,
    pub name: String,
    pub shortname: Option<String>,
}

#[derive(Insertable)]
#[table_name = "rpg_systems"]
pub struct NewRpgSystem<'a> {
    pub name: &'a str,
    pub shortname: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "titles"]
#[primary_key(title_id)]
#[belongs_to(RpgSystem, foreign_key = "rpg_system_by_id")]
pub struct Title {
    pub title_id: i32,
    pub name: String,
    pub rpg_system_by_id: i32,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}
