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
    #[column_name = "rpg_system_id"]
    pub id: Id,
    pub name: String,
    pub shortname: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "rpg_systems"]
pub struct NewRpgSystem {
    pub name: String,
    pub shortname: Option<String>,
}

#[derive(
    Identifiable, Queryable, Associations, Serialize, Deserialize, PartialEq, Debug, Clone,
)]
#[table_name = "titles"]
#[primary_key(title_id)]
#[belongs_to(RpgSystem, foreign_key = "rpg_system_by_id")]
pub struct Title {
    #[column_name = "title_id"]
    pub id: Id,
    pub name: String,
    #[column_name = "rpg_system_by_id"]
    pub rpg_system_id: Id,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "titles"]
pub struct NewTitle {
    pub name: String,
    #[column_name = "rpg_system_by_id"]
    pub rpg_system_id: Id,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct TitleWithRpgSystem {
    pub id: Id,
    pub name: String,
    pub rpg_system: RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: Year,
    pub coverimage: Option<String>,
}

impl From<(Title, RpgSystem)> for TitleWithRpgSystem {
    fn from((title, rpg_system): (Title, RpgSystem)) -> Self {
        TitleWithRpgSystem {
            id: title.id,
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
    #[column_name = "account_id"]
    pub id: Id,
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
            id: account.id,
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
    #[column_name = "guild_id"]
    pub id: Id,
    pub name: String,
    pub address: String,
}

#[derive(Insertable, AsChangeset, Deserialize, Clone)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub name: String,
    pub address: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
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

impl From<Account> for Owner {
    fn from(account: Account) -> Self {
        Owner::Member { id: account.id }
    }
}

impl From<Guild> for Owner {
    fn from(guild: Guild) -> Self {
        Owner::Guild { id: guild.id }
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
    #[column_name = "book_id"]
    pub id: Id,
    #[column_name = "title_by_id"]
    pub title_id: Id,
    pub owner: Owner,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl Queryable<books::SqlType, Mysql> for Book {
    type Row = (Id, Id, Option<Id>, Option<Id>, String, Id);

    fn build(row: Self::Row) -> Self {
        Book {
            id: row.0,
            title_id: row.1,
            owner: Owner::from((row.2, row.3)),
            quality: row.4,
            external_inventory_id: row.5,
        }
    }
}

#[derive(Insertable, Deserialize, Clone)]
#[table_name = "books"]
pub struct NewBook {
    #[column_name = "title_by_id"]
    pub title_id: Id,
    #[diesel(embed)]
    pub owner: Owner,
    // rentee: MemberOrGuild,
    pub quality: String,
    pub external_inventory_id: Id,
}

#[derive(Serialize, Debug, Clone)]
pub struct BookWithTitleWithRpgSystem {
    pub id: Id,
    pub title: TitleWithRpgSystem,
    pub owner: Owner,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl From<(Book, TitleWithRpgSystem)> for BookWithTitleWithRpgSystem {
    fn from((book, title): (Book, TitleWithRpgSystem)) -> Self {
        BookWithTitleWithRpgSystem {
            id: book.id,
            title,
            owner: book.owner,
            quality: book.quality,
            external_inventory_id: book.external_inventory_id,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct BookWithTitle {
    pub id: Id,
    pub title: Title,
    pub owner: Owner,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl From<(Book, Title)> for BookWithTitle {
    fn from((book, title): (Book, Title)) -> Self {
        BookWithTitle {
            id: book.id,
            title,
            owner: book.owner,
            quality: book.quality,
            external_inventory_id: book.external_inventory_id,
        }
    }
}

/// Allows creation of books, where the owner is derived from the token or endpoint.
#[derive(Deserialize, Clone)]
pub struct PostOwnedBook {
    pub title_id: Id,
    pub quality: String,
    pub external_inventory_id: Id,
}

impl PostOwnedBook {
    pub fn owned_by(self, owner: Owner) -> NewBook {
        NewBook {
            title_id: self.title_id,
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
        let (member_id, guild_id) = match self.owner {
            Owner::Member { id } => (Some(id), None),
            Owner::Guild { id } => (None, Some(id)),
        };
        (
            title_by_id.eq(self.title_id),
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
