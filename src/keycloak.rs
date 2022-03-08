use crate::InternalError;
use jsonwebtoken::DecodingKey;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RealmMetadata {
    pub realm: String,
    pub public_key: String,
    // sic, Why Keycloak, why?
    #[serde(rename = "token-service")]
    pub token_service: String,
    #[serde(rename = "account-service")]
    pub account_service: String,
    #[serde(rename = "tokens-not-before")]
    _tokens_not_before: u32,
}

impl RealmMetadata {
    pub async fn fetch_new(keycloak_url: &str, realm: &str) -> Result<Self, InternalError> {
        let meta_data_endpoint = format!("{}/realms/{}", keycloak_url, realm);
        let meta = reqwest::get(meta_data_endpoint)
            .await
            .map_err(InternalError::KeycloakNotReachable)?
            .json::<RealmMetadata>()
            .await?;
        return Ok(meta);
    }

    pub fn decoding_key(&self) -> Result<DecodingKey, InternalError> {
        let key_with_headers = format!(
            "-----BEGIN PUBLIC KEY-----{}-----END PUBLIC KEY-----",
            self.public_key
        );
        DecodingKey::from_rsa_pem(key_with_headers.as_bytes())
            .map_err(InternalError::KeycloakKeyHasBadFormat)
    }
}
