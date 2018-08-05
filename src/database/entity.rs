use super::*;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub type EntityId = Id;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntityType {
    Member,
    Guild,
}

impl EntityType {
    pub fn from_str(s: &str) -> Result<EntityType, String> {
        match s {
            "member" => Ok(EntityType::Member),
            "guild" => Ok(EntityType::Guild),
            _ => Err(String::from("Expected 'member' or 'guild'")),
        }
    }

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
