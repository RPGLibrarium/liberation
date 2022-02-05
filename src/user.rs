use log::{debug, info};
use oauth2::basic::BasicClient;
use oauth2::TokenResponse;
use reqwest::Client;
use crate::{Authenticator, InternalError};
use crate::keycloak::RealmMetadata;

pub struct LiveUsers {
    oauth_client: BasicClient,
    users_endpoint: String,
}

struct Users {}

impl LiveUsers {
    pub async fn new(
        keycloak_url: String,
        realm: String,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, InternalError> {
        use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl};
        let keycloak = RealmMetadata::fetch_new(&keycloak_url, &realm).await?;
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(format!("{}/auth", keycloak.token_service)).expect("Invalid url"),
            Some(TokenUrl::new(format!("{}/token", keycloak.token_service)).expect("Invalid url")),
        );

        let users_endpoint = format!("{}/auth/admin/realms/{}/users", keycloak_url, realm);

        let live_users = LiveUsers {
            oauth_client: client,
            users_endpoint,
        };

        debug!("Testing live users connection");
        live_users.update().await?;
        debug!("Live users connection successful.");
        Ok(live_users)
    }

    pub async fn update(&self) -> Result<(), InternalError> {
        use oauth2::reqwest::async_http_client;
        let token_result = self.oauth_client
            .exchange_client_credentials()
            .request_async(async_http_client)
            .await
            .map_err(|e| InternalError::KeycloakAuthenticationFailed(Box::new(e)))?;

        let client = Client::new();
        let result = client.get(self.users_endpoint.as_str())
            .bearer_auth(token_result.access_token().secret())
            .send().await
            .map_err(InternalError::KeycloakNotReachable)?;
        info!("{}", result.text().await.unwrap());

        // TODO: support refresh tokens
        Ok(())
    }
}
