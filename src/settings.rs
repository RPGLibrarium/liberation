use config::{Config, ConfigError, Environment, File};
use url::Url;
use url_serde;

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub hostname: Option<String>, //default 127.0.0.1 by mysql
    pub port: Option<u16>,
    pub username: Option<String>, //default None by mysql
    pub password: Option<String>, //default None by mysql
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Keycloak {
    #[serde(with = "url_serde")]
    pub url: Url,
    pub realm: String,
    pub clientid: String,
    pub clientsecret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub serve_static_files: bool,
    pub database: Database,
    pub keycloak: Keycloak,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/defaults")).unwrap();

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))
            .unwrap();

        s.merge(Environment::with_prefix("LIBERATION").separator("_"))
            .unwrap();

        s.try_into()
    }
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
pub struct TestSettings {
    pub debug: bool,
    pub database: Database,
}

#[cfg(test)]
impl TestSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/defaults")).unwrap();

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))
            .unwrap();

        s.merge(File::with_name("config/test").required(false))
            .unwrap();

        s.set("database.database", "")?;

        s.merge(Environment::with_prefix("LIBERATION").separator("_"))
            .unwrap();

        s.try_into()
    }
}
