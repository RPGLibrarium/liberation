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

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "rpg_systems"]
pub struct NewRpgSystem {
    pub name: String,
    pub shortname: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "titles"]
pub struct NewTitle {
    pub name: String,
    pub rpg_system_by_id: i32,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}
