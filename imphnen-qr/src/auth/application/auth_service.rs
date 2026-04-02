use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use crate::common::qr_jwt::QrJwtService;
use crate::config::QrConfig;
use super::super::domain::service::{QrAuthService, AuthTokens, QrUserData};

pub struct QrAuthServiceImpl {
    pool: Arc<PgPool>,
    jwt: Arc<QrJwtService>,
    config: Arc<QrConfig>,
}

impl QrAuthServiceImpl {
    pub fn new(pool: Arc<PgPool>, jwt: Arc<QrJwtService>, config: Arc<QrConfig>) -> Self {
        Self { pool, jwt, config }
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<QrUserData, AppError> {
        sqlx::query_as::<_, QrUserData>(
            "SELECT id, email, name, role, provider, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<serde_json::Value>, AppError> {
        sqlx::query_scalar::<_, serde_json::Value>(
            "SELECT row_to_json(u) FROM (SELECT id, email, name, role, provider, password FROM users WHERE email = $1) u"
        )
        .bind(email)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    fn make_tokens(&self, user_id: Uuid, role: &str) -> Result<AuthTokens, AppError> {
        Ok(AuthTokens {
            access_token: self.jwt.generate_token(user_id, role)?,
            refresh_token: self.jwt.generate_refresh_token(user_id, role)?,
        })
    }
}

#[async_trait]
impl QrAuthService for QrAuthServiceImpl {
    async fn register(&self, email: String, password: String, name: String) -> Result<(AuthTokens, QrUserData), AppError> {
        let existing = self.find_user_by_email(&email).await?;
        if existing.is_some() {
            return Err(AppError::ConflictError("Email already registered".to_string()));
        }
        let hashed = bcrypt::hash(&password, 10)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let user = sqlx::query_as::<_, QrUserData>(
            "INSERT INTO users (email, password, name, role, provider) VALUES ($1, $2, $3, 'user', 'local') RETURNING id, email, name, role, provider, created_at, updated_at"
        )
        .bind(&email)
        .bind(&hashed)
        .bind(&name)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let tokens = self.make_tokens(user.id, &user.role)?;
        Ok((tokens, user))
    }

    async fn login(&self, email: String, password: String) -> Result<(AuthTokens, QrUserData), AppError> {
        let row = self.find_user_by_email(&email).await?
            .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;
        let provider = row["provider"].as_str().unwrap_or("local");
        if provider != "local" {
            return Err(AppError::AuthenticationError("Account uses social login".to_string()));
        }
        let stored_hash = row["password"].as_str()
            .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;
        let valid = bcrypt::verify(&password, stored_hash)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        if !valid {
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }
        let user_id: Uuid = row["id"].as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or_else(|| AppError::InternalServerError("Invalid user ID".to_string()))?;
        let user = self.find_user_by_id(user_id).await?;
        let tokens = self.make_tokens(user.id, &user.role)?;
        Ok((tokens, user))
    }

    async fn google_callback(&self, code: String) -> Result<(AuthTokens, QrUserData), AppError> {
        let http = reqwest::Client::new();
        let token_res: serde_json::Value = http
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code.as_str()),
                ("client_id", self.config.google_client_id.as_str()),
                ("client_secret", self.config.google_client_secret.as_str()),
                ("redirect_uri", self.config.google_redirect_url.as_str()),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        if token_res.get("error").is_some() {
            return Err(AppError::BadRequestError("Google OAuth error".to_string()));
        }
        let access_token = token_res["access_token"].as_str()
            .ok_or_else(|| AppError::InternalServerError("Missing access token from Google".to_string()))?;
        let google_user: serde_json::Value = http
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let email = google_user["email"].as_str()
            .ok_or_else(|| AppError::InternalServerError("Missing email from Google".to_string()))?;
        let name = google_user["name"].as_str().unwrap_or(email);
        let provider_id = google_user["id"].as_str().unwrap_or("");
        let user = sqlx::query_as::<_, QrUserData>(
            "INSERT INTO users (email, name, role, provider, provider_id) VALUES ($1, $2, 'user', 'google', $3)
             ON CONFLICT (email) DO UPDATE SET provider_id = EXCLUDED.provider_id, updated_at = NOW()
             RETURNING id, email, name, role, provider, created_at, updated_at"
        )
        .bind(email)
        .bind(name)
        .bind(provider_id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let tokens = self.make_tokens(user.id, &user.role)?;
        Ok((tokens, user))
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<AuthTokens, AppError> {
        let claims = self.jwt.verify_token(&refresh_token)?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::AuthenticationError("Invalid token subject".to_string()))?;
        let user = self.find_user_by_id(user_id).await?;
        Ok(AuthTokens {
            access_token: self.jwt.generate_token(user.id, &user.role)?,
            refresh_token,
        })
    }
}
