use crate::error::InternalError;
use config::Config;
use serde::Deserialize;

fn default_bind() -> String {
    "127.0.0.1:8080".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    /// The server uses this database. Only mysql is supported a the moment.
    ///
    /// ```toml
    /// database = "mysql://liberation:liberation@127.0.0.1:3306/liberation"
    /// ```
    pub database: String,

    /// The server binds to this address. Defaults to `127.0.0.1:8080`.
    ///
    /// ```toml
    /// bind = 127.0.0.1:8080
    /// ```
    #[serde(default = "default_bind")]
    pub bind: String,

    /// See [AuthenticationSettings] for more details.
    pub authentication: AuthenticationSettings,
}

/// The authentication is done with [JWT tokens](https://jwt.io/) that must be signed by the
/// authorization server (probably Keycloak). Only one of the following variants is accepted.
///
/// ## Keycloak
/// The public key is fetched automatically from keycloak and renewed periodically.
///
/// ```toml
/// [authentication.keycloak]
/// url = "https://sso.rpg-librarium.de/"
/// realm = "Liberation"
/// ```
///
/// ## Static
/// The public key is set constant at runtime. This might be helpful when testing or migrating to
/// another authentication server.
///
/// ```toml
/// [authentication.static]
/// public_key = "MIIBIjANBgkqhkiG9w0BAQEFAA..."
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthenticationSettings {
    Static {
        public_key: String,
    },
    Keycloak {
        url: String,
        realm: String,
        /// the key is renewed every so often (in seconds)
        renew_interval_s: u64,
    },
}

impl Settings {
    pub fn with_file(file: String) -> Result<Self, InternalError> {
        let mut settings = Config::default();

        settings
            .merge(config::File::with_name(file.as_str()))
            .unwrap()
            .merge(config::Environment::with_prefix("LIBERATION"))
            .unwrap();

        settings
            .try_into::<Settings>()
            .map_err(InternalError::ConfigError)
    }
}
