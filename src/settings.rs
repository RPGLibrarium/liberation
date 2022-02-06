use config::Config;
use oauth2::{ClientId, ClientSecret};
use serde::Deserialize;
use crate::error::InternalError;


fn default_bind() -> String { "127.0.0.1:8080".to_string() }

fn default_aristocrat_role() -> String { "liberation:aristocrat".to_string() }

fn default_member_role() -> String { "liberation:member".to_string() }

fn default_librarian_role_prefix() -> String { "liberation:librarian:".to_string() }

fn default_role_mapping() -> RoleMapping {
    RoleMapping {
        aristocrat_role: default_aristocrat_role(),
        member_role: default_member_role(),
        librarian_role_prefix: default_librarian_role_prefix(),
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: String,
    #[serde(default = "default_bind")]
    pub bind: String,
    /// Set's the public key for verification manually
    pub jwt_public_key: Option<String>,
    pub keycloak: Option<Keycloak>,
    #[serde(default = "default_role_mapping")]
    pub role_mapping: RoleMapping,
}

#[derive(Debug, Deserialize)]
pub struct Keycloak {
    pub url: String,
    pub realm: String,
    pub client_id: Option<ClientId>,
    pub client_secret: Option<ClientSecret>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoleMapping {
    #[serde(default = "default_aristocrat_role")]
    pub aristocrat_role: String,
    #[serde(default = "default_member_role")]
    pub member_role: String,
    #[serde(default = "default_librarian_role_prefix")]
    pub librarian_role_prefix: String,
}

impl Settings {
    pub fn with_file(file: String) -> Result<Self, InternalError> {
        let mut settings = Config::default();

        settings.merge(config::File::with_name(file.as_str())).unwrap()
            .merge(config::Environment::with_prefix("LIBERATION")).unwrap();

        settings.try_into::<Settings>()
            .map_err(InternalError::ConfigError)
    }
}
