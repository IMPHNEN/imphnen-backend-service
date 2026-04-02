use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, sqlx::FromRow)]
pub struct HackathonUserData {
    pub id: Uuid,
    pub email: String,
    pub fullname: String,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub skills: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
pub trait HackathonAuthService: Send + Sync {
    async fn signup(&self, email: String, password: String, fullname: String) -> Result<(), AppError>;
    async fn login(&self, email: String, password: String) -> Result<(AuthTokens, HackathonUserData), AppError>;
    async fn github_auth(&self, code: String) -> Result<(AuthTokens, HackathonUserData), AppError>;
    async fn get_session(&self, user_id: Uuid) -> Result<HackathonUserData, AppError>;
    async fn forgot_password(&self, email: String) -> Result<(), AppError>;
    async fn reset_password(&self, access_token: String, new_password: String) -> Result<(), AppError>;
}
