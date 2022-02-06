use std::collections::HashMap;
use log::{debug, info};
use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::error::InternalError;
use crate::keycloak::RealmMetadata;

pub struct LiveUsers {
    oauth_client: BasicClient,
    users_endpoint: String,
    users: Mutex<HashMap<String, User>>,
}

#[derive(Debug, Deserialize)]
struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
}

impl LiveUsers {
    pub async fn new(
        keycloak_url: &str,
        realm: &str,
        client_id: ClientId,
        client_secret: ClientSecret,
    ) -> Result<Self, InternalError> {
        use oauth2::{AuthUrl, TokenUrl};
        let keycloak = RealmMetadata::fetch_new(&keycloak_url, &realm).await?;
        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            AuthUrl::new(format!("{}/auth", keycloak.token_service)).expect("Invalid url"),
            Some(TokenUrl::new(format!("{}/token", keycloak.token_service)).expect("Invalid url")),
        );

        let users_endpoint = format!("{}/auth/admin/realms/{}/users", keycloak_url, realm);

        let live_users = LiveUsers {
            oauth_client: client,
            users_endpoint,
            users: Mutex::new(HashMap::default()),
        };

        debug!("Initializing live users");
        live_users.update().await?;
        debug!("Live users connection successful.");
        Ok(live_users)
    }

    pub async fn update(&self) -> Result<(), InternalError> {
        use oauth2::reqwest::async_http_client;
        let token_result = self.oauth_client
            .exchange_client_credentials()
            .request_async(async_http_client).await
            .map_err(|e| InternalError::KeycloakAuthenticationFailed(Box::new(e)))?;

        let client = Client::new();
        let keycloak_users = client.get(self.users_endpoint.as_str())
            .bearer_auth(token_result.access_token().secret())
            .send().await
            .map_err(InternalError::KeycloakNotReachable)?
            .json::<Vec<User>>().await?;

        let user_map = keycloak_users.into_iter()
            .map(|user| (user.id.clone(), user))
            .collect::<HashMap<String, User>>();

        info!("Updating user cache.");
        (*self.users.lock().await) = user_map;
        Ok(())
    }
}
