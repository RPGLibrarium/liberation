use crate::database::{DAO, Database, BetterFromRow};
use mysql::{Value, FromRowError, Row};
use chrono::{NaiveDate, Utc};
use crate::error::Error;
use crate::error::Error::{EnumFromStringError, IllegalState};
use serde::{Serialize, Deserialize};
use mysql::prelude::FromRow;

pub type Id = u64;
pub type Year = i16;
pub type Date = NaiveDate;

#[derive(Clone, Serialize, Deserialize)]
pub struct RpgSystemData {
    pub name: String,
    pub abbreviation: Option<String>,
}

impl From<RpgSystemData> for Vec<(String, Value)> {
    fn from(data: RpgSystemData) -> Self {
        params! {
            "name" => &data.name,
            "shortname" => &data.abbreviation
        }
    }
}

impl BetterFromRow for RpgSystemData {
    fn from_row(mut row: Row) -> Result<Self, Error> where
        Self: Sized {
        Ok(RpgSystemData {
            name: row.take_opt("name").ok_or(IllegalState("Expected column name not found"))??,
            abbreviation: row.take_opt("shortname").ok_or(IllegalState("Expected column shortname not found"))??
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RpgSystem {
    pub id: Id,
    pub name: String,
    pub abbreviation: Option<String>,
}

impl DAO for RpgSystem {
    const TABLE_NAME: &'static str = "rpg_systems";
    const IDENTIFIER_COLUMN: &'static str = "rpg_system_id";
    type Data = RpgSystemData;
    type Identifier = Id;

    fn construct(id: Self::Identifier, data: Self::Data, database: &Database) -> Result<Self, Error> {
        Ok(RpgSystem {
            id,
            name: data.name,
            abbreviation: data.abbreviation,
        })
    }

    fn deconstruct(self) -> (Self::Identifier, Self::Data) {
        (self.id, RpgSystemData { name: self.name.clone(), abbreviation: self.abbreviation.clone() })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TitleData {
    pub name: String,
    pub rpg_system: <RpgSystem as DAO>::Identifier,
    pub language: String,
    pub publisher: String,
    pub year: u16,
}

impl From<TitleData> for Vec<(String, Value)> {
    fn from(data: TitleData) -> Self {
        params! {
            "name" => &data.name,
            "rpg_system_by_id" => data.rpg_system,
            "language" => &data.language,
            "publisher" => &data.publisher,
            "year" => data.year,
        }
    }
}

impl BetterFromRow for TitleData {
    fn from_row(mut row: Row) -> Result<Self, Error> where
        Self: Sized {
        Ok(TitleData {
            name: row.take_opt("name").ok_or(IllegalState("Expected column name not found"))??,
            rpg_system: row.take_opt("rpg_system_by_id").ok_or(IllegalState("Expected column rpg_system not found"))??,
            language: row.take_opt("language").ok_or(IllegalState("Expected column language not found"))??,
            publisher: row.take_opt("publisher").ok_or(IllegalState("Expected column publisher not found"))??,
            year: row.take_opt("year").ok_or(IllegalState("Expected column year not found"))??,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: Id,
    pub name: String,
    pub rpg_system: RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: u16,
}

impl DAO for Title {
    const TABLE_NAME: &'static str = "titles";
    const IDENTIFIER_COLUMN: &'static str = "title_id";
    type Data = TitleData;
    type Identifier = Id;

    fn construct(id: Self::Identifier, data: Self::Data, database: &Database) -> Result<Self, Error> {
        let rpg_system = database.get::<RpgSystem>(&data.rpg_system)?
            .ok_or(Error::IllegalState("Resolving rpg_system_by_id failed."))?;

        Ok(Title {
            id,
            name: data.name,
            rpg_system,
            language: data.language,
            publisher: data.publisher,
            year: data.year,
        })
    }

    fn deconstruct(self) -> (Self::Identifier, Self::Data) {
        (self.id.clone(), TitleData {
            name: self.name.clone(),
            rpg_system: self.rpg_system.id.clone(),
            language: self.language.clone(),
            publisher: self.publisher.clone(),
            year: self.year.clone(),
        })
    }
}

/// EntityType describes whether an entity is a person or an organisation
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum BookState {
    /// Free for rental by everybody,
    Free,
    /// Rented
    Rented,
    /// Reserved, can only be rented by next person in queue
    Reserved,
    /// Lost, might respawn some day but not available for rental at the moment
    Lost,
    /// Destroyed in all eternity
    Destroyed,
}

impl BookState {
    /// Converts a string describing a BookState to a BookState
    /// possible values: "free", "rented", "reserved", "lost", "destroyed"
    pub fn from_str(s: String) -> Result<BookState, Error> {
        match s.as_str() {
            "free" => Ok(BookState::Free),
            "rented" => Ok(BookState::Rented),
            "reserved" => Ok(BookState::Reserved),
            "lost" => Ok(BookState::Lost),
            "destroyed" => Ok(BookState::Destroyed),
            _ => Err(EnumFromStringError(String::from("Expected 'free' or 'rented', 'reserved', 'lost', 'destroyed'."))),
        }
    }

    /// Converts an EntityType to a corresponding string
    pub fn to_string(&self) -> String {
        match self {
            BookState::Free => String::from("free"),
            BookState::Rented => String::from("rented"),
            BookState::Reserved => String::from("reserved"),
            BookState::Lost => String::from("lost"),
            BookState::Destroyed => String::from("destroyed"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BookData {
    pub title: <Title as DAO>::Identifier,
    pub inventory_id: Id,
    pub quality: String,
    pub owner_member_id: Option<Id>,
    pub owner_guild_id: Option<Id>,
    pub rentee_member_id: Option<Id>,
    pub rentee_guild_id: Option<Id>,
    pub state: BookState,
    pub state_since: NaiveDate,
}

impl From<BookData> for Vec<(String, Value)> {
    fn from(data: BookData) -> Self {
        params! {
            "title_by_id" => data.title,
            "external_inventory_id" => data.inventory_id,
            "quality" => data.quality,
            "owner_member_by_id" => data.owner_member_id,
            "owner_guild_by_id" => data.owner_guild_id,
            "rentee_member_by_id" => data.rentee_member_id,
            "rentee_guild_by_id" => data.rentee_guild_id,
            "state" => data.state.to_string(),
            "state_since" => data.state_since
        }
    }
}

impl BetterFromRow for BookData {
    fn from_row(mut row: Row) -> Result<Self, Error> where
        Self: Sized {
        Ok(BookData {
            title: row.take_opt("title_by_id").ok_or(IllegalState("Expected column title not found"))??,
            inventory_id: row.take_opt("external_inventory_id").ok_or(IllegalState("Expected column publisher not found"))??,
            quality: row.take_opt("quality").ok_or(IllegalState("Expected column publisher not found"))??,
            owner_member_id: row.take_opt("owner_member_by_id").ok_or(IllegalState("Expected column owner_member_by_id not found"))??,
            owner_guild_id: row.take_opt("owner_guild_by_id").ok_or(IllegalState("Expected column owner_guild_by_id not found"))??,
            rentee_member_id: row.take_opt("rentee_member_by_id").ok_or(IllegalState("Expected column rentee_member_by_id not found"))??,
            rentee_guild_id: row.take_opt("rentee_guild_by_id").ok_or(IllegalState("Expected column rentee_guild_by_id not found"))??,
            state: BookState::from_str(row.take_opt("state").ok_or(IllegalState("Expected column state not found"))??)?,
            state_since: row.take_opt("state_since").ok_or(IllegalState("Expected column state_since not found"))??,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Id,
    pub title: Title,
    pub inventory_id: u64,
    pub quality: String,
    pub owned_by: Owner,
    pub rented_by: Owner,
    pub state: BookState,
    pub state_since: NaiveDate,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Owner {
    Member { member: Member },
    Guild { guild: Guild },
}

impl Owner {
    pub fn from_ids(member_id: Option<Id>, guild_id: Option<Id>) -> Result<Owner, Error> {
        match (member_id, guild_id) {
            (Some(_), None) => Ok(Owner::Member { member: Member {} }),
            (None, Some(_)) => Ok(Owner::Guild { guild: Guild {} }),
            (None, None) => Err(Error::IllegalState("None of guild or member id where set.")),
            (Some(_), Some(_)) => Err(Error::IllegalState("Both of guild and member id where set."))
        }
    }
}

impl DAO for Book {
    const TABLE_NAME: &'static str = "books";
    const IDENTIFIER_COLUMN: &'static str = "book_id";
    type Data = BookData;
    type Identifier = Id;

    fn construct(id: Self::Identifier, data: Self::Data, database: &Database) -> Result<Self, Error> {
        let title = database.get::<Title>(&data.title)?
            .ok_or(Error::IllegalState("Resolving title_by_id failed."))?;

        Ok(Book {
            id,
            title,
            inventory_id: data.inventory_id,
            quality: data.quality,
            owned_by: Owner::from_ids(data.owner_member_id, data.owner_guild_id)?,
            rented_by: Owner::from_ids(data.rentee_member_id, data.rentee_guild_id)?,
            state: data.state,
            state_since: data.state_since,

        })
    }

    fn deconstruct(self) -> (Self::Identifier, Self::Data) {
        (self.id.clone(), BookData {
            title: self.title.id,
            inventory_id: self.inventory_id,
            quality: self.quality,
            owner_member_id: if let Owner::Member { member} = &self.owned_by { Some(1) } else { None },
            owner_guild_id: if let Owner::Guild { guild } = &self.owned_by { Some(1) } else { None },
            rentee_member_id: if let Owner::Member { member } = &self.rented_by { Some(1) } else { None },
            rentee_guild_id: if let Owner::Guild { guild } = &self.rented_by { Some(1) } else { None },
            state: self.state,
            state_since: self.state_since,
        })
    }
}

/// TODO: Placeholder, because we don't need them now
#[derive(Clone, Serialize, Deserialize)]
pub struct Member {}

/// TODO: Placeholder, because we don't need them now
#[derive(Clone, Serialize, Deserialize)]
pub struct Guild {}
