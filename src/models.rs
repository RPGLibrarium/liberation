use super::schema::accounts;
use super::schema::books;
use super::schema::guilds;
use super::schema::librarians;
use super::schema::rpg_systems;
use super::schema::titles;
use diesel::mysql::Mysql;
use diesel::query_builder::AsChangeset;
use diesel::{ExpressionMethods, Insertable, Queryable};
use serde::Deserialize;
use serde::Serialize;

pub type Year = i16;
pub type Id = i32;

#[derive(Deserialize)]
pub struct QueryOptions {
    #[serde(default = "bool::default")] // Defaults to false
    pub recursive: bool,
}

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[table_name = "rpg_systems"]
#[primary_key(rpg_system_id)]
pub struct RpgSystem {
    pub rpg_system_id: Id,
    pub name: String,
    pub shortname: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "rpg_systems"]
pub struct NewRpgSystem {
    pub name: String,
    pub shortname: String,
}

#[derive(
    Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone,
)]
#[table_name = "titles"]
#[primary_key(title_id)]
#[belongs_to(RpgSystem, foreign_key = "rpg_system_by_id")]
pub struct Title {
    pub title_id: Id,
    pub name: String,
    pub rpg_system_by_id: Id,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "titles"]
pub struct NewTitle {
    pub name: String,
    pub rpg_system_by_id: Id,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct RecursiveTitle {
    pub title_id: Id,
    pub name: String,
    pub rpg_system: RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

impl From<(Title, RpgSystem)> for RecursiveTitle {
    fn from((title, rpg_system): (Title, RpgSystem)) -> Self {
        RecursiveTitle {
            title_id: title.title_id,
            name: title.name,
            rpg_system,
            language: title.language,
            publisher: title.publisher,
            year: title.year,
            coverimage: title.coverimage,
        }
    }
}

#[derive(Identifiable, Queryable, Deserialize, PartialEq, Serialize, Debug, Clone)]
#[table_name = "accounts"]
#[primary_key(account_id)]
pub struct Account {
    pub account_id: Id,
    pub active: bool,
    pub external_id: String,
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

/// DTO with less information about a user
#[derive(Serialize, Debug)]
pub struct User {
    pub id: Id,
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
    pub full_name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct AccountActive {
    pub is_active: bool,
}

#[derive(Identifiable, Queryable, PartialEq, Serialize, Deserialize, Debug)]
#[table_name = "guilds"]
#[primary_key(guild_id)]
pub struct Guild {
    pub guild_id: Id,
    pub name: String,
    pub address: String,
    pub contact_by_account_id: Id,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub name: String,
    pub address: String,
    pub contact_by_account_id: Id,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[serde(tag = "type")]
pub enum Owner {
    #[serde(rename = "member")]
    Member { id: Id },
    #[serde(rename = "guild")]
    Guild { id: Id },
}

impl From<(Option<Id>, Option<Id>)> for Owner {
    fn from(ids: (Option<Id>, Option<Id>)) -> Self {
        match ids {
            (Some(id), None) => Self::Member { id },
            (None, Some(id)) => Self::Guild { id },
            _ => panic!("database contains owner member and guild!"),
        }
    }
}

impl Into<(Option<Id>, Option<Id>)> for Owner {
    fn into(self) -> (Option<Id>, Option<Id>) {
        match self {
            Self::Member { id } => (Some(id), None),
            Self::Guild { id } => (None, Some(id)),
        }
    }
}

// This maps the owner to the correct colums. `cargo expand` does really help here.
impl Insertable<books::table> for Owner {
    type Values = <(
        Option<diesel::dsl::Eq<books::owner_member_by_id, Id>>,
        Option<diesel::dsl::Eq<books::owner_guild_by_id, Id>>,
    ) as Insertable<books::table>>::Values;

    fn values(self) -> Self::Values {
        use crate::schema::books::dsl::*;
        //use diesel::dsl::*;
        match self {
            Owner::Member { id } => (Some(owner_member_by_id.eq(id)), None),
            Owner::Guild { id } => (None, Some(owner_guild_by_id.eq(id))),
        }
        .values()
    }
}

// Borrowed variant, which is also needed
impl<'insert> Insertable<books::table> for &'insert Owner {
    type Values = <(
        Option<diesel::dsl::Eq<books::owner_member_by_id, Id>>,
        Option<diesel::dsl::Eq<books::owner_guild_by_id, Id>>,
    ) as Insertable<books::table>>::Values;

    fn values(self) -> Self::Values {
        use crate::schema::books::dsl::*;
        //use diesel::dsl::*;
        match self {
            Owner::Member { id } => (Some(owner_member_by_id.eq(*id)), None),
            Owner::Guild { id } => (None, Some(owner_guild_by_id.eq(*id))),
        }
        .values()
    }
}

#[derive(Identifiable, Associations, Serialize, Debug)]
#[table_name = "books"]
#[primary_key(book_id)]
// Associations seem buggy, they dont generate code, when there are multiple belongs_to.
// Perhaps its also because of the foreign key being part of an embedded field.
#[belongs_to(Account, foreign_key=owner_member_by_id)]
//#[belongs_to(Guild, foreign_key=owner_guild_by_id)]
pub struct Book {
    pub book_id: Id,
    pub title_by_id: Id,
    pub owner: Owner,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl Queryable<books::SqlType, Mysql> for Book {
    type Row = (Id, Id, Option<Id>, Option<Id>, String, Id);

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
    pub title_by_id: Id,
    #[diesel(embed)]
    pub owner: Owner,
    // rentee: MemberOrGuild,
    pub quality: String,
    pub external_inventory_id: Id,
}

/// Allows creation of books, where the owner is derived from the token or endpoint.
#[derive(Deserialize, Clone)]
pub struct PostOwnedBook {
    pub title_by_id: Id,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl PostOwnedBook {
    pub fn owned_by(self, owner: Owner) -> NewBook {
        NewBook {
            title_by_id: self.title_by_id,
            owner,
            quality: self.quality,
            external_inventory_id: self.external_inventory_id,
        }
    }
}

impl AsChangeset for NewBook {
    type Target = books::table;
    type Changeset = <(
        diesel::dsl::Eq<books::title_by_id, Id>,
        diesel::dsl::Eq<books::owner_member_by_id, Option<Id>>,
        diesel::dsl::Eq<books::owner_guild_by_id, Option<Id>>,
        diesel::dsl::Eq<books::quality, String>,
        diesel::dsl::Eq<books::external_inventory_id, Id>,
    ) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        use crate::schema::books::dsl::*;
        let (member_id, guild_id) = self.owner.into();
        (
            title_by_id.eq(self.title_by_id),
            owner_member_by_id.eq(member_id),
            owner_guild_by_id.eq(guild_id),
            quality.eq(self.quality),
            external_inventory_id.eq(self.external_inventory_id),
        )
            .as_changeset()
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "librarians"]
pub struct NewLibrarian {
    account_id: Id,
    guild_id: Id,
}

#[derive(Queryable, Clone)]
pub struct Librarian {
    _permission_id: Id,
    pub account_id: Id,
    pub guild_id: Id,
}
