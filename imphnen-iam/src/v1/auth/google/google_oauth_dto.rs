use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}