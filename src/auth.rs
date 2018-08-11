use error::Error;
use futures::Future;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenUrl};
use settings::Keycloak as KeycloakSettings;
use url::Url;

pub struct Keycloak {
    keycloak_url: Url,
    realm: String,
    oauth_client: BasicClient,
}

impl Keycloak {
    pub fn new(keycloak_url: Url, realm: String, client_id: String, client_secret: String) -> Self {
        let tokenUrl = TokenUrl::new(
            keycloak_url
                .join("realms/")
                .unwrap()
                .join(format!("{}/", realm.clone()).as_str())
                .unwrap()
                .join("protocol/openid-connect/token")
                .unwrap(),
        );

        let authUrl = AuthUrl::new(
            keycloak_url
                .join("realms/")
                .unwrap()
                .join(format!("{}/", realm.clone()).as_str())
                .unwrap()
                .join("protocol/openid-connect/auth")
                .unwrap(),
        );

        Keycloak {
            keycloak_url: keycloak_url.clone(),
            realm: realm.clone(),
            oauth_client: BasicClient::new(
                ClientId::new(client_id.clone()),
                Some(ClientSecret::new(client_secret.clone())),
                authUrl,
                Some(tokenUrl),
            ),
        }
    }

    pub fn from_settings(settings: &KeycloakSettings) -> Self {
        Keycloak::new(
            settings.url.clone(),
            settings.realm.clone(),
            settings.clientid.clone(),
            settings.clientsecret.clone(),
        )
    }

    pub fn get_keycloak_users(&self) -> Result<(), Error> {
        let token_result = self.oauth_client.exchange_client_credentials()?;

        let user_url = self
            .keycloak_url
            .join("admin/realms/")
            .unwrap()
            .join(format!("{}/", self.realm).as_str())
            .unwrap()
            .join("users")
            .unwrap();

        // .header("Authorization", format!("Bearer {}", token_result.access_token().secret()))
        // .header("host", "localhost:8081")

        Ok(())
    }
}
pub struct Token {}
