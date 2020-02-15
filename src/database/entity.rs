use super::*;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use crate::error::Error::{IllegalState, EnumFromStringError};

/// Id type for Entity
pub type EntityId = Id;

/// EntityType describes whether an entity is a person or an organisation
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityType {
    /// Member of a guild
    Member,
    /// Guild
    Guild,
}

impl EntityType {
    /// Converts a string describing an EntityType to an EntityType
    /// possible values: "member", "guild"
    pub fn from_str(s: &str) -> Result<EntityType, Error> {
        match s {
            "member" => Ok(EntityType::Member),
            "guild" => Ok(EntityType::Guild),
            _ => Err(EnumFromStringError(String::from("Expected 'member' or 'guild'"))),
        }
    }

    /// Converts an EntityType to a corresponding string
    pub fn to_string(&self) -> String {
        match self {
            EntityType::Member => String::from("member"),
            EntityType::Guild => String::from("guild"),
        }
    }

    /// Based on the given type, select the correct of both given Ids.
    pub fn select_entity_id(&self, member_id: Option<MemberId>, guild_id: Option<GuildId>)
                            -> Result<EntityId, Error> {
        match match self {
            EntityType::Member => member_id,
            EntityType::Guild => guild_id,
        } {
            Some(x) => Ok(x),
            None => Err(IllegalState("Field 'owner_member' or 'owner_guild' is not set according to 'owner_type'.")),
        }
    }
}

impl Serialize for EntityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for EntityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        return EntityType::from_str(s.as_str()).map_err(de::Error::custom);
    }
}

pub fn to_guild_id(id: EntityId, entity_type: EntityType) -> Option<EntityId> {
    match entity_type {
        EntityType::Member => None,
        EntityType::Guild => Some(id),
    }
}

pub fn to_member_id(id: EntityId, entity_type: EntityType) -> Option<EntityId> {
    match entity_type {
        EntityType::Member => Some(id),
        EntityType::Guild => None,
    }
}
