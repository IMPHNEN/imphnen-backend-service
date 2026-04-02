use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use imphnen_utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct QrUserData {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub provider: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
pub trait QrAuthService: Send + Sync {
    async fn register(&self, email: String, password: String, name: String) -> Result<(AuthTokens, QrUserData), AppError>;
    async fn login(&self, email: String, password: String) -> Result<(AuthTokens, QrUserData), AppError>;
    async fn google_callback(&self, code: String) -> Result<(AuthTokens, QrUserData), AppError>;
    async fn refresh_token(&self, refresh_token: String) -> Result<AuthTokens, AppError>;
}
