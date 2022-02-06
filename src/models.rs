use super::schema::rpg_systems;
use super::schema::titles;
use super::schema::members;
use super::schema::guilds;
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

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "members"]
#[primary_key(member_id)]
pub struct Member{
    pub member_id: i32,
    pub external_id: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "members"]
pub struct NewMember {
    pub external_id: String,
}

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "guilds"]
#[primary_key(guild_id)]
pub struct Guild {
    pub guild_id: i32,
    pub external_guild_name: String,
    pub name: String,
    pub address: String,
    pub contact_by_member_id: i32,
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub external_guild_name: String,
    pub name: String,
    pub address: String,
    pub contact_by_member_id: i32,
}
