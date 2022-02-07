use diesel::mysql::Mysql;
use diesel::{ExpressionMethods, Insertable, Queryable};
use diesel::query_builder::AsChangeset;
use super::schema::rpg_systems;
use super::schema::titles;
use super::schema::accounts;
use super::schema::guilds;
use super::schema::books;
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

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(tag = "type")]
pub enum Owner {
    #[serde(rename = "member")]
    Member { id: i32 },
    #[serde(rename = "guild")]
    Guild { id: i32 },
}

impl From<(Option<i32>, Option<i32>)> for Owner {
    fn from(ids: (Option<i32>, Option<i32>)) -> Self {
        match ids {
            (Some(id), None) => Self::Member { id },
            (None, Some(id)) => Self::Guild { id },
            _ => panic!("database contains owner member and guild!")
        }
    }
}

impl Into<(Option<i32>, Option<i32>)> for Owner {
    fn into(self) -> (Option<i32>, Option<i32>) {
        match self {
            Self::Member {id} => (Some(id), None),
            Self::Guild{id} => (None, Some(id)),
        }
    }
}


// This maps the owner to the correct colums. `cargo expand` does really help here.
impl Insertable<books::table> for Owner {
    type Values = <(
        Option<diesel::dsl::Eq<books::owner_member_by_id, i32>>,
        Option<diesel::dsl::Eq<books::owner_guild_by_id, i32>>,
    ) as Insertable<books::table>>::Values;

    fn values(self) -> Self::Values {
        use crate::schema::books::dsl::*;
        //use diesel::dsl::*;
        match self {
            Owner::Member { id } => (Some(owner_member_by_id.eq(id)), None),
            Owner::Guild { id } => (None, Some(owner_guild_by_id.eq(id))),
        }.values()
    }
}

// Borrowed variant, which is also needed
impl<'insert> Insertable<books::table> for &'insert Owner {
    type Values = <(
        Option<diesel::dsl::Eq<books::owner_member_by_id, i32>>,
        Option<diesel::dsl::Eq<books::owner_guild_by_id, i32>>,
    ) as Insertable<books::table>>::Values;

    fn values(self) -> Self::Values {
        use crate::schema::books::dsl::*;
        //use diesel::dsl::*;
        match self {
            Owner::Member { id } => (Some(owner_member_by_id.eq(*id)), None),
            Owner::Guild { id } => (None, Some(owner_guild_by_id.eq(*id))),
        }.values()
    }
}

#[derive(Identifiable, Serialize, Debug)]
#[table_name = "books"]
#[primary_key(book_id)]
pub struct Book {
    pub book_id: i32,
    pub title_by_id: i32,
    pub owner: Owner,
    pub quality: String,
    pub external_inventory_id: i32,
}

impl Queryable<books::SqlType, Mysql> for Book {
    type Row = (i32, i32, Option<i32>, Option<i32>, String, i32);

    fn build(row: Self::Row) -> Self {
        Book {
            book_id: row.0,
            title_by_id: row.1,
            owner: Owner::from((row.2, row.3)),
            quality: row.4,
            external_inventory_id: row.5,
        }
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "books"]
pub struct NewBook {
    pub book_id: i32,
    pub title_by_id: i32,
    #[diesel(embed)]
    pub owner: Owner,
    // rentee: MemberOrGuild,
    pub quality: String,
    pub external_inventory_id: i32,
}

impl AsChangeset for NewBook {
    type Target = books::table;
    type Changeset = <(
        diesel::dsl::Eq<books::book_id, i32>,
        diesel::dsl::Eq<books::title_by_id, i32>,
        diesel::dsl::Eq<books::owner_member_by_id, Option<i32>>,
        diesel::dsl::Eq<books::owner_guild_by_id, Option<i32>>,
        diesel::dsl::Eq<books::quality, String>,
        diesel::dsl::Eq<books::external_inventory_id, i32>,
    ) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        use crate::schema::books::dsl::*;
        let (member_id, guild_id) = self.owner.into();
        (
            book_id.eq(self.book_id),
            title_by_id.eq(self.title_by_id),
            owner_member_by_id.eq(member_id),
            owner_guild_by_id.eq(guild_id),
            quality.eq(self.quality),
            external_inventory_id.eq(self.external_inventory_id)
        ).as_changeset()
    }
}
