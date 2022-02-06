use config::Config;
use oauth2::{ClientId, ClientSecret};
use serde::Deserialize;
use liberation::error::InternalError;

const DEFAULT_BIND: &str = "127.0.0.1:8080";

fn default_bind() -> String { DEFAULT_BIND.to_string() }

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: String,
    #[serde(default = "default_bind")]
    pub bind: String,
    /// Set's the public key for verification manually
    pub jwt_public_key: Option<String>,
    pub keycloak: Option<Keycloak>,
}

#[derive(Debug, Deserialize)]
pub struct Keycloak {
    pub url: String,
    pub realm: String,
    pub client_id: Option<ClientId>,
    pub client_secret: Option<ClientSecret>,
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
