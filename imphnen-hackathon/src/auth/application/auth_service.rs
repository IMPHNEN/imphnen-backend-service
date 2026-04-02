use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, TimeZone};
use sqlx::PgPool;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::common::supabase_client::SupabaseClient;
use crate::config::HackathonConfig;
use super::super::domain::service::{HackathonAuthService, AuthTokens, HackathonUserData};

fn is_registration_closed() -> bool {
    let deadline = Utc.with_ymd_and_hms(2025, 11, 30, 16, 29, 0).unwrap();
    Utc::now() >= deadline
}

pub struct HackathonAuthServiceImpl {
    pool: Arc<PgPool>,
    jwt: Arc<HackathonJwtService>,
    supabase: Arc<SupabaseClient>,
    config: Arc<HackathonConfig>,
}

impl HackathonAuthServiceImpl {
    pub fn new(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>, supabase: Arc<SupabaseClient>, config: Arc<HackathonConfig>) -> Self {
        Self { pool, jwt, supabase, config }
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<HackathonUserData, AppError> {
        sqlx::query_as::<_, HackathonUserData>(
            "SELECT id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at FROM hackathon_users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
    }

    async fn get_or_create_active_user(&self, user_id: Uuid, email: &str, fullname: &str) -> Result<HackathonUserData, AppError> {
        let now = Utc::now();
        sqlx::query_as::<_, HackathonUserData>(
            "INSERT INTO hackathon_users (id, email, fullname, is_active, created_at, updated_at)
             VALUES ($1, LOWER($2), $3, true, $4, $5)
             ON CONFLICT (email) DO UPDATE SET is_active = true, updated_at = NOW()
             RETURNING id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at"
        )
        .bind(user_id)
        .bind(email)
        .bind(fullname)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_or_create_github_user(&self, email: &str, fullname: &str, avatar: Option<&str>) -> Result<HackathonUserData, AppError> {
        let existing: Option<HackathonUserData> = sqlx::query_as::<_, HackathonUserData>(
            "SELECT id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at FROM hackathon_users WHERE LOWER(email) = LOWER($1)"
        )
        .bind(email)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if let Some(user) = existing {
            return Ok(user);
        }

        if is_registration_closed() {
            return Err(AppError::BadRequestError("Registration is closed.".to_string()));
        }

        let now = Utc::now();
        let user_id = Uuid::new_v4();
        sqlx::query_as::<_, HackathonUserData>(
            "INSERT INTO hackathon_users (id, email, fullname, avatar, is_active, created_at, updated_at)
             VALUES ($1, LOWER($2), $3, $4, true, $5, $6)
             ON CONFLICT (email) DO UPDATE SET avatar = COALESCE(hackathon_users.avatar, EXCLUDED.avatar), is_active = true, updated_at = NOW()
             RETURNING id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at"
        )
        .bind(user_id)
        .bind(email)
        .bind(fullname)
        .bind(avatar)
        .bind(now)
        .bind(now)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}

#[async_trait]
impl HackathonAuthService for HackathonAuthServiceImpl {
    async fn signup(&self, email: String, password: String, fullname: String) -> Result<(), AppError> {
        if is_registration_closed() {
            return Err(AppError::BadRequestError("Registration is closed.".to_string()));
        }
        let data = self.supabase.signup(&email, &password, &fullname, &self.config.frontend_url).await?;
        let user_id_str = data["user"]["id"].as_str()
            .or_else(|| data["id"].as_str())
            .ok_or_else(|| AppError::InternalServerError("Missing user ID in signup response".to_string()))?;
        let user_uuid = Uuid::parse_str(user_id_str)
            .map_err(|_| AppError::InternalServerError("Invalid user ID format".to_string()))?;
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO hackathon_users (id, email, fullname, is_active, created_at, updated_at) VALUES ($1, LOWER($2), $3, false, $4, $5) ON CONFLICT (email) DO NOTHING"
        )
        .bind(user_uuid)
        .bind(email)
        .bind(fullname)
        .bind(now)
        .bind(now)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn login(&self, email: String, password: String) -> Result<(AuthTokens, HackathonUserData), AppError> {
        let data = self.supabase.login(&email, &password).await?;
        let email_confirmed = data["user"]["email_confirmed_at"].as_str().map(|s| !s.is_empty()).unwrap_or(false);
        if !email_confirmed {
            return Err(AppError::AuthenticationError("Please confirm your email before logging in.".to_string()));
        }
        let user_id_str = data["user"]["id"].as_str()
            .ok_or_else(|| AppError::InternalServerError("Missing user ID".to_string()))?;
        let user_id = Uuid::parse_str(user_id_str).map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;
        let fullname = data["user"]["user_metadata"]["fullname"].as_str()
            .or_else(|| data["user"]["user_metadata"]["full_name"].as_str())
            .unwrap_or(&email).to_string();
        let user = self.get_or_create_active_user(user_id, &email, &fullname).await?;
        let tokens = AuthTokens {
            access_token: self.jwt.generate_token(user.id)?,
            refresh_token: self.jwt.generate_refresh_token(user.id)?,
        };
        Ok((tokens, user))
    }

    async fn github_auth(&self, code: String) -> Result<(AuthTokens, HackathonUserData), AppError> {
        let http = reqwest::Client::new();
        let token_data: serde_json::Value = http.post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&[("client_id", &self.config.github_client_id), ("client_secret", &self.config.github_client_secret), ("code", &code)])
            .send().await.map_err(|e| AppError::InternalServerError(e.to_string()))?
            .json().await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        if token_data.get("error").is_some() {
            return Err(AppError::BadRequestError("GitHub OAuth error".to_string()));
        }
        let access_token = token_data["access_token"].as_str()
            .ok_or_else(|| AppError::InternalServerError("Missing access token from GitHub".to_string()))?;
        let github_user: serde_json::Value = http.get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "imphnen-hackathon-api")
            .send().await.map_err(|e| AppError::InternalServerError(e.to_string()))?
            .json().await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let github_id = github_user["id"].as_i64()
            .ok_or_else(|| AppError::InternalServerError("Missing GitHub user ID".to_string()))?;
        let username = github_user["login"].as_str().unwrap_or("user");
        let email = match github_user["email"].as_str().filter(|e| !e.is_empty()) {
            Some(e) => e.to_string(),
            None => {
                let emails: Vec<serde_json::Value> = http.get("https://api.github.com/user/emails")
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("User-Agent", "imphnen-hackathon-api")
                    .send().await.map_err(|e| AppError::InternalServerError(e.to_string()))?
                    .json().await.unwrap_or_default();
                emails.iter().find(|e| e["primary"].as_bool().unwrap_or(false))
                    .or_else(|| emails.iter().find(|e| e["verified"].as_bool().unwrap_or(false)))
                    .and_then(|e| e["email"].as_str()).map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{}+{}@users.noreply.github.com", github_id, username))
            }
        };
        let fullname = github_user["name"].as_str().or_else(|| github_user["login"].as_str()).unwrap_or("GitHub User").to_string();
        let avatar = github_user["avatar_url"].as_str();
        let user = self.get_or_create_github_user(&email, &fullname, avatar).await?;
        let tokens = AuthTokens {
            access_token: self.jwt.generate_token(user.id)?,
            refresh_token: self.jwt.generate_refresh_token(user.id)?,
        };
        Ok((tokens, user))
    }

    async fn get_session(&self, user_id: Uuid) -> Result<HackathonUserData, AppError> {
        self.get_user_by_id(user_id).await
    }

    async fn forgot_password(&self, email: String) -> Result<(), AppError> {
        self.supabase.recover_password(&email, &self.config.frontend_url).await
    }

    async fn reset_password(&self, access_token: String, new_password: String) -> Result<(), AppError> {
        if new_password.len() < 6 {
            return Err(AppError::BadRequestError("Password must be at least 6 characters long".to_string()));
        }
        self.supabase.update_password(&access_token, &new_password).await
    }
}
