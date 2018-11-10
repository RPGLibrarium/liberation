use super::*;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
    // TODO: document possible strings
    /// Converts a string describing an EntityType to an EntityType
    pub fn from_str(s: &str) -> Result<EntityType, String> {
        match s {
            "member" => Ok(EntityType::Member),
            "guild" => Ok(EntityType::Guild),
            _ => Err(String::from("Expected 'member' or 'guild'")),
        }
    }

    /// Converts an EntityType to a corresponding string
    pub fn to_string(&self) -> String {
        match self {
            EntityType::Member => String::from("member"),
            EntityType::Guild => String::from("guild"),
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
