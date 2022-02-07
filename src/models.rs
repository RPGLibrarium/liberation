use super::schema::rpg_systems;
use super::schema::titles;
use super::schema::accounts;
use super::schema::guilds;
use serde::Deserialize;
use serde::Serialize;

pub type Year = i16;

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[table_name = "rpg_systems"]
#[primary_key(rpg_system_id)]
pub struct RpgSystem {
    pub rpg_system_id: i32,
    pub name: String,
    pub shortname: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "rpg_systems"]
pub struct NewRpgSystem {
    pub name: String,
    pub shortname: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "titles"]
pub struct NewTitle {
    pub name: String,
    pub rpg_system_by_id: i32,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

#[derive(Identifiable, Queryable, Deserialize, PartialEq, Serialize, Debug, Clone)]
#[table_name = "accounts"]
#[primary_key(account_id)]
pub struct Account {
    pub account_id: i32,
    pub active: bool,
    pub external_id: String,
    pub username: String,
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

/// DTO with less information about a user
#[derive(Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub active: bool,
    pub full_name: String,
}

impl From<Account> for User {
    fn from(account: Account) -> Self {
        User {
            id: account.account_id,
            active: account.active,
            full_name: account.full_name,
        }
    }
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub active: bool,
    pub external_id: String,
    pub username: String,
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct NewAccountPost {
    pub username: String,
}

// TODO: Unsure which info should be modifiable
// #[derive(Insertable, Deserialize, Clone)]
// #[table_name = "accounts"]
// pub struct UpdateAccount{
//     pub username: String,
//     pub given_name: String,
//     pub family_name: String,
//     pub email: String,
// }

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "guilds"]
#[primary_key(guild_id)]
pub struct Guild {
    pub guild_id: i32,
    pub external_id: String,
    pub name: String,
    pub address: String,
    pub contact_by_account_id: i32,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub external_id: String,
    pub name: String,
    pub address: String,
    pub contact_by_account_id: i32,
}
