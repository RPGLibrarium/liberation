use actix_web::{actix, client, HttpMessage};
use database::type_aliases::*;
use error::Error;
use futures::future::FutureResult;
use futures::Future;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenUrl};
use settings::Keycloak as KeycloakSettings;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct KeycloakUser {
    id: String,
    createdTimestamp: u32,
    username: String,
    enabled: bool,
    totp: bool,
    emailVerified: bool,
    disableableCredentialTypes: Vec<String>,
    requiredActions: Vec<String>,
    notBefore: u32,
    access: Access,
}

#[derive(Deserialize, Debug)]
pub struct Access {
    manageGroupMembership: bool,
    view: bool,
    mapRoles: bool,
    impersonate: bool,
    manage: bool,
}

pub struct UserCache {}

impl UserCache {}

pub struct Keycloak {
    keycloak_url: Url,
    realm: String,
    oauth_client: BasicClient,
    cache: HashMap<ExternalId, KeycloakUser>,
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

        let mut kc = Keycloak {
            keycloak_url: keycloak_url.clone(),
            realm: realm.clone(),
            oauth_client: BasicClient::new(
                ClientId::new(client_id.clone()),
                Some(ClientSecret::new(client_secret.clone())),
                authUrl,
                Some(tokenUrl),
            ),
            cache: HashMap::new(),
        };

        kc.fetch().unwrap();

        return kc;
    }

    pub fn from_settings(settings: &KeycloakSettings) -> Self {
        Keycloak::new(
            settings.url.clone(),
            settings.realm.clone(),
            settings.clientid.clone(),
            settings.clientsecret.clone(),
        )
    }

    pub fn fetch(&mut self) -> Result<(), Error> {
        let token_result = self.oauth_client.exchange_client_credentials().unwrap();

        let user_url = self
            .keycloak_url
            .join("admin/realms/")
            .unwrap()
            .join(format!("{}/", self.realm).as_str())
            .unwrap()
            .join("users")
            .unwrap();
        println!("{:?}", token_result.access_token().secret());

        let users = client::get(user_url)   // <- Create request builder
            .no_default_headers()
            .header("Authorization", format!("Bearer {}", token_result.access_token().secret()))
            .header("host", "localhost:8081")
        .finish().unwrap()
        .send()                               // <- Send http request
        .map_err(|err| Error::KeycloakConnectionError(err))
        .and_then(|response| response.json().map_err(|err| Error::JsonPayloadError(err)))
        .map(|obj: Vec<KeycloakUser>| obj).wait()?;
        panic!("here");
        println!("{:?}", users);

        Ok(())
    }

    pub fn get_user(
        &self,
        userId: &ExternalId,
    ) -> impl Future<Item = Option<&KeycloakUser>, Error = Error> {
        FutureResult::from(Ok(self.cache.get(userId)))
    }
}
pub struct Token {}
