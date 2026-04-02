use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use imphnen_utils::errors::AppError;
use crate::chat::domain::entity::*;
use crate::chat::domain::repository::ChatRepository;

#[derive(FromRow)]
struct MessageRow {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    message: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<MessageRow> for MessageEntity {
    fn from(r: MessageRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            user_id: r.user_id,
            message: r.message,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(FromRow)]
struct MessageWithUserRow {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    user_fullname: String,
    user_avatar: Option<String>,
    message: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<MessageWithUserRow> for MessageWithUser {
    fn from(r: MessageWithUserRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            user_id: r.user_id,
            user_fullname: r.user_fullname,
            user_avatar: r.user_avatar,
            message: r.message,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(FromRow)]
struct UserInfoRow {
    fullname: String,
    avatar: Option<String>,
}

pub struct PostgresChatRepository {
    pool: Arc<PgPool>,
}

impl PostgresChatRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepository for PostgresChatRepository {
    async fn find_team_messages(&self, team_id: Uuid) -> Result<Vec<MessageWithUser>, AppError> {
        let rows: Vec<MessageWithUserRow> = sqlx::query_as(
            "SELECT m.id, m.team_id, m.user_id, u.fullname AS user_fullname, u.avatar AS user_avatar, m.message, m.created_at, m.updated_at FROM hackathon_team_messages m JOIN hackathon_users u ON u.id = m.user_id WHERE m.team_id = $1 ORDER BY m.created_at ASC"
        )
        .bind(team_id).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create_message(&self, id: Uuid, team_id: Uuid, user_id: Uuid, message: &str) -> Result<MessageEntity, AppError> {
        let now = Utc::now();
        let row: MessageRow = sqlx::query_as(
            "INSERT INTO hackathon_team_messages (id, team_id, user_id, message, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, team_id, user_id, message, created_at, updated_at"
        )
        .bind(id).bind(team_id).bind(user_id).bind(message).bind(now).bind(now)
        .fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.into())
    }

    async fn find_message_by_id(&self, id: Uuid) -> Result<Option<MessageEntity>, AppError> {
        let row: Option<MessageRow> = sqlx::query_as(
            "SELECT id, team_id, user_id, message, created_at, updated_at FROM hackathon_team_messages WHERE id = $1"
        )
        .bind(id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn delete_message(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM hackathon_team_messages WHERE id = $1")
            .bind(id).execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn get_user_info(&self, user_id: Uuid) -> Result<Option<(String, Option<String>)>, AppError> {
        let row: Option<UserInfoRow> = sqlx::query_as(
            "SELECT fullname, avatar FROM hackathon_users WHERE id = $1"
        )
        .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(|r| (r.fullname, r.avatar)))
    }

    async fn is_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_team_members WHERE team_id = $1 AND user_id = $2 AND status = 'active')")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn is_team_leader(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_teams WHERE id = $1 AND leader_id = $2)")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
