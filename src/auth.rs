use actix::prelude::*;
use actix_web::{client, http, HttpMessage, HttpRequest};
use api::AppState;
use base64;
use database::type_aliases::*;
use error::Error;
use futures::Future;
use jsonwebtoken as jwt;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use openssl::rsa::*;
use settings::Keycloak as KeycloakSettings;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use url::Url;

pub mod roles {
    pub const ROLE_ADMIN: &str = "admin";
    pub const ROLE_LIBRARIAN: &str = "librarian";
    pub const ROLE_MEMBER: &str = "member";
    pub const ROLE_ARISTOCRAT: &str = "aristocrat";
}

#[derive(Deserialize, Debug, Clone)]
pub struct KeycloakUser {
    id: String,
    #[serde(rename = "createdTimestamp")]
    created_timestamp: u64,
    username: String,
    enabled: bool,
    totp: bool,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    #[serde(rename = "disableableCredentialTypes")]
    disableable_credential_types: Vec<String>,
    #[serde(rename = "requiredActions")]
    required_actions: Vec<String>,
    #[serde(rename = "notBefore")]
    not_before: u64,
    access: Access,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Access {
    #[serde(rename = "manageGroupMembership")]
    manage_group_membership: bool,
    view: bool,
    #[serde(rename = "mapRoles")]
    map_roles: bool,
    impersonate: bool,
    manage: bool,
}

#[derive(Deserialize, Debug)]
pub struct KeycloakMetaInfo {
    realm: String,
    public_key: String,
    #[serde(rename = "token-service")]
    token_service: String,
    #[serde(rename = "account-service")]
    account_service: String,
    #[serde(rename = "tokens-not-before")]
    tokens_not_before: u32,
}

#[derive(Clone, Debug)]
pub struct KeycloakCache {
    cache: Arc<Mutex<HashMap<ExternalId, KeycloakUser>>>,
    pk: Arc<Mutex<String>>,
}

pub struct Keycloak {
    keycloak_url: Url,
    realm: String,
    oauth_client: BasicClient,
    cache: KeycloakCache,
}

impl KeycloakCache {
    pub fn new() -> KeycloakCache {
        let empty_key: [u8; 0];
        KeycloakCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            pk: Arc::new(Mutex::new(String::from(""))),
        }
    }

    pub fn insert_user(&self, user: KeycloakUser) {
        self.cache.lock().unwrap().insert(user.id.clone(), user);
    }

    pub fn get_user(&self, user_id: &ExternalId) -> Result<Option<KeycloakUser>, Error> {
        Ok(self
            .cache
            .lock()
            .expect("Can not lock user cache mutex.")
            .get(user_id)
            .map(|user| (*user).clone()))
    }

    pub fn reset_users(&self) {}

    pub fn set_public_key(&self, public_key: String) {
        let mut pk = self.pk.lock().expect("Can not lock public_key mutex.");
        *pk = public_key;
    }

    pub fn get_public_key(&self) -> String {
        let pk = self.pk.lock().expect("Can not lock public_key mutex");
        return (*pk).clone();
    }
}

impl Actor for Keycloak {
    type Context = Context<Keycloak>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::new(5, 0), Keycloak::fetch);
    }
}

impl Keycloak {
    pub fn new(
        keycloak_url: Url,
        realm: String,
        client_id: String,
        client_secret: String,
        cache: KeycloakCache,
    ) -> Self {
        let token_url = TokenUrl::new(
            keycloak_url
                .join("realms/")
                .unwrap()
                .join(format!("{}/", realm.clone()).as_str())
                .unwrap()
                .join("protocol/openid-connect/token")
                .unwrap(),
        );

        let auth_url = AuthUrl::new(
            keycloak_url
                .join("realms/")
                .unwrap()
                .join(format!("{}/", realm.clone()).as_str())
                .unwrap()
                .join("protocol/openid-connect/auth")
                .unwrap(),
        );

        let kc = Keycloak {
            keycloak_url: keycloak_url.clone(),
            realm: realm.clone(),
            oauth_client: BasicClient::new(
                ClientId::new(client_id.clone()),
                Some(ClientSecret::new(client_secret.clone())),
                auth_url,
                Some(token_url),
            ),
            cache: cache,
        };

        return kc;
    }

    pub fn from_settings(settings: &KeycloakSettings, cache: KeycloakCache) -> Self {
        Keycloak::new(
            settings.url.clone(),
            settings.realm.clone(),
            settings.clientid.clone(),
            settings.clientsecret.clone(),
            cache,
        )
    }

    pub fn fetch(kc: &mut Self, _ctx: &mut Context<Keycloak>) {
        let token_result = kc.oauth_client.exchange_client_credentials().unwrap();

        let user_url = kc
            .keycloak_url
            .join("admin/realms/")
            .unwrap()
            .join(format!("{}/", kc.realm).as_str())
            .unwrap()
            .join("users")
            .unwrap();

        let cloned_cache = kc.cache.clone();

        Arbiter::spawn(
            client::get(user_url) // <- Create request builder
                .no_default_headers()
                .header(
                    "Authorization",
                    format!("Bearer {}", token_result.access_token().secret()),
                ).header("host", "localhost:8081")
                .finish()
                .unwrap()
                .send() // <- Send http request
                .map_err(|err| Error::KeycloakConnectionError(err))
                .and_then(|response| {
                    debug!("response: {:?}", response);
                    response.json().map_err(|err| Error::JsonPayloadError(err))
                }).map_err(|err| panic!("Unexpected KeycloakError {}", err))
                .and_then(|users: Vec<KeycloakUser>| {
                    //info!("users: {:?}", users);
                    users.into_iter().for_each(move |user| {
                        cloned_cache.insert_user(user);
                    });
                    println!("Fetched users");
                    //info!("users: {:?}", move cloned_cache2);
                    Ok(())
                }),
        );

        let key_url = kc
            .keycloak_url
            .join("realms/")
            .unwrap()
            .join(format!("{}/", kc.realm).as_str())
            .unwrap();

        println!("{}", key_url.as_str());

        let cloned_cache = kc.cache.clone();

        Arbiter::spawn(
            client::get(key_url) // <- Create request builder
                .no_default_headers()
                .header("host", "localhost:8081")
                .finish()
                .unwrap()
                .send() // <- Send http request
                .map_err(|err| Error::KeycloakConnectionError(err))
                .and_then(|response| response.json().map_err(|err| Error::JsonPayloadError(err)))
                .map_err(|err| panic!("Unexpected KeycloakError {}", err))
                .and_then(move |response: KeycloakMetaInfo| {
                    cloned_cache.set_public_key(response.public_key);
                    Ok(())
                }),
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: String,
    pub roles: Vec<String>,
    pub name: String,
    pub email: String,
    // ... whatever!
}

pub fn get_claims_for_req(req: &HttpRequest<AppState>) -> Result<Option<Claims>, Error> {
    match req.headers().get(http::header::AUTHORIZATION) {
        None => {
            debug!("No Authorization Header provided");
            Ok(None)
        }
        Some(header_val) => match header_val.to_str() {
            Err(_) => {
                debug!("Authorization header could not be converted to string");
                Err(Error::InvalidAuthenticationError)
            }
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.replacen("Bearer ", "", 1);
                    let pubkey = req.state().kc.get_public_key();
                    let pk_der_asn1 = base64::decode(pubkey.as_str())
                        .expect("JWT checking: invalid base64 encoding of Keycloak public key)");
                    let pk = Rsa::public_key_from_der(pk_der_asn1.as_slice())
                        .expect("JWT checking: invalid ASN.1 format of Keycloak public key");
                    let pk_der_pkcs1 = pk
                        .public_key_to_der_pkcs1()
                        .expect("JWT checking: converting public key to DER PKCS1 failed");

                    match jwt::decode::<Claims>(
                        &token,
                        pk_der_pkcs1.as_slice(),
                        &jwt::Validation::new(jwt::Algorithm::RS256),
                    ) {
                        Err(e) => {
                            error!("JWT validation failed: {:?}", e);
                            Err(Error::InvalidAuthenticationError)
                        }
                        Ok(token_data) => {
                            debug!("Successfully verified JWT: {:?}", token_data);
                            let token_claims: Claims = token_data.claims;
                            Ok(Some(token_claims))
                        }
                    }
                } else {
                    Err(Error::InvalidAuthenticationError)
                }
            }
        },
    }
}

pub fn assert_roles(
    req: &HttpRequest<AppState>,
    roles: Vec<&str>,
) -> Result<Option<Claims>, Error> {
    let claims = get_claims_for_req(req)?;

    match roles.is_empty() {
        true => Ok(claims),
        false => match claims {
            Some(cl) => {
                for role in roles.iter() {
                    //let roleString = String::from(*role);
                    if cl.roles.contains(&String::from(*role)) {
                        return Ok(Some(cl));
                    }
                }
                Err(Error::YouShallNotPassError)
            }
            None => Err(Error::SpeakFriendAndEnterError),
        },
    }
}
