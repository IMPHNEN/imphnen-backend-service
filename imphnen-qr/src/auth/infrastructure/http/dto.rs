use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::auth::domain::service::QrUserData;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: QrUserData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}
