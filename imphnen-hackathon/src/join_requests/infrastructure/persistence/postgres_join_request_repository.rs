use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use imphnen_utils::errors::AppError;
use crate::join_requests::domain::entity::*;
use crate::join_requests::domain::repository::JoinRequestRepository;

#[derive(FromRow)]
struct JoinRequestRow {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    message: String,
    status: String,
    created_at: Option<DateTime<Utc>>,
}

impl From<JoinRequestRow> for JoinRequestEntity {
    fn from(r: JoinRequestRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            user_id: r.user_id,
            message: r.message,
            status: r.status,
            created_at: r.created_at,
        }
    }
}

#[derive(FromRow)]
struct JoinRequestDetailsRow {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    user_fullname: String,
    user_email: String,
    user_avatar: Option<String>,
    message: String,
    status: String,
    created_at: Option<DateTime<Utc>>,
}

impl From<JoinRequestDetailsRow> for JoinRequestWithDetails {
    fn from(r: JoinRequestDetailsRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            user_id: r.user_id,
            user_fullname: r.user_fullname,
            user_email: r.user_email,
            user_avatar: r.user_avatar,
            message: r.message,
            status: r.status,
            created_at: r.created_at,
        }
    }
}

pub struct PostgresJoinRequestRepository {
    pool: Arc<PgPool>,
}

impl PostgresJoinRequestRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JoinRequestRepository for PostgresJoinRequestRepository {
    async fn create(&self, id: Uuid, team_id: Uuid, user_id: Uuid, message: &str) -> Result<JoinRequestEntity, AppError> {
        let row: JoinRequestRow = sqlx::query_as(
            "INSERT INTO hackathon_team_join_requests (id, team_id, user_id, message, status, created_at) VALUES ($1, $2, $3, $4, 'pending', NOW()) RETURNING id, team_id, user_id, message, status, created_at"
        )
        .bind(id).bind(team_id).bind(user_id).bind(message)
        .fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<JoinRequestEntity>, AppError> {
        let row: Option<JoinRequestRow> = sqlx::query_as(
            "SELECT id, team_id, user_id, message, status, created_at FROM hackathon_team_join_requests WHERE id = $1"
        )
        .bind(id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<JoinRequestWithDetails>, AppError> {
        let rows: Vec<JoinRequestDetailsRow> = sqlx::query_as(
            "SELECT r.id, r.team_id, r.user_id, u.fullname AS user_fullname, u.email AS user_email, u.avatar AS user_avatar, r.message, r.status, r.created_at FROM hackathon_team_join_requests r JOIN hackathon_users u ON u.id = r.user_id WHERE r.user_id = $1 ORDER BY r.created_at DESC"
        )
        .bind(user_id).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_pending_by_team(&self, team_id: Uuid) -> Result<Vec<JoinRequestWithDetails>, AppError> {
        let rows: Vec<JoinRequestDetailsRow> = sqlx::query_as(
            "SELECT r.id, r.team_id, r.user_id, u.fullname AS user_fullname, u.email AS user_email, u.avatar AS user_avatar, r.message, r.status, r.created_at FROM hackathon_team_join_requests r JOIN hackathon_users u ON u.id = r.user_id WHERE r.team_id = $1 AND r.status = 'pending' ORDER BY r.created_at ASC"
        )
        .bind(team_id).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_join_requests SET status = $1 WHERE id = $2")
            .bind(status).bind(id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn add_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("INSERT INTO hackathon_team_members (id, team_id, user_id, role, status, joined_at) VALUES ($1, $2, $3, 'member', 'active', NOW())")
            .bind(Uuid::new_v4()).bind(team_id).bind(user_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn reject_pending_invitations_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let email: Option<String> = sqlx::query_scalar("SELECT email FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        if let Some(email) = email {
            sqlx::query("UPDATE hackathon_team_invitations SET status = 'rejected' WHERE invitee_email = $1 AND status = 'pending'")
                .bind(email)
                .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        }
        Ok(())
    }

    async fn reject_other_pending_for_user(&self, user_id: Uuid, except_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_join_requests SET status = 'rejected' WHERE user_id = $1 AND status = 'pending' AND id != $2")
            .bind(user_id).bind(except_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn get_team_leader_id(&self, team_id: Uuid) -> Result<Option<Uuid>, AppError> {
        sqlx::query_scalar("SELECT leader_id FROM hackathon_teams WHERE id = $1")
            .bind(team_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_user_email(&self, user_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT email FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn team_exists(&self, team_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_teams WHERE id = $1)")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn team_has_submission(&self, team_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_project_submissions WHERE team_id = $1)")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn user_active_team_name(&self, user_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT t.name FROM hackathon_teams t JOIN hackathon_team_members m ON m.team_id = t.id WHERE m.user_id = $1 AND m.status = 'active' LIMIT 1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn active_member_count(&self, team_id: Uuid) -> Result<i64, AppError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_team_members WHERE team_id = $1 AND status = 'active'")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn pending_request_exists(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_team_join_requests WHERE team_id = $1 AND user_id = $2 AND status = 'pending')")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
