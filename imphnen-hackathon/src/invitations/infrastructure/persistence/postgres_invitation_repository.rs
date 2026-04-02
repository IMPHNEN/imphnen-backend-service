use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use imphnen_utils::errors::AppError;
use crate::invitations::domain::entity::*;
use crate::invitations::domain::repository::InvitationRepository;

#[derive(FromRow)]
struct InvitationRow {
    id: Uuid,
    team_id: Uuid,
    inviter_id: Uuid,
    invitee_email: String,
    status: String,
    created_at: Option<DateTime<Utc>>,
}

impl From<InvitationRow> for InvitationEntity {
    fn from(r: InvitationRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            inviter_id: r.inviter_id,
            invitee_email: r.invitee_email,
            status: r.status,
            created_at: r.created_at,
        }
    }
}

#[derive(FromRow)]
struct InvitationDetailsRow {
    id: Uuid,
    team_id: Uuid,
    team_name: String,
    inviter_id: Uuid,
    inviter_fullname: String,
    invitee_email: String,
    status: String,
    created_at: Option<DateTime<Utc>>,
}

impl From<InvitationDetailsRow> for InvitationWithDetails {
    fn from(r: InvitationDetailsRow) -> Self {
        Self {
            id: r.id,
            team_id: r.team_id,
            team_name: r.team_name,
            inviter_id: r.inviter_id,
            inviter_fullname: r.inviter_fullname,
            invitee_email: r.invitee_email,
            status: r.status,
            created_at: r.created_at,
        }
    }
}

pub struct PostgresInvitationRepository {
    pool: Arc<PgPool>,
}

impl PostgresInvitationRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvitationRepository for PostgresInvitationRepository {
    async fn create(&self, invitation_id: Uuid, team_id: Uuid, inviter_id: Uuid, invitee_email: &str) -> Result<InvitationEntity, AppError> {
        let row: InvitationRow = sqlx::query_as(
            "INSERT INTO hackathon_team_invitations (id, team_id, inviter_id, invitee_email, status, created_at) VALUES ($1, $2, $3, $4, 'pending', NOW()) RETURNING id, team_id, inviter_id, invitee_email, status, created_at"
        )
        .bind(invitation_id).bind(team_id).bind(inviter_id).bind(invitee_email)
        .fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<InvitationEntity>, AppError> {
        let row: Option<InvitationRow> = sqlx::query_as(
            "SELECT id, team_id, inviter_id, invitee_email, status, created_at FROM hackathon_team_invitations WHERE id = $1"
        )
        .bind(id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn find_pending_by_email(&self, email: &str) -> Result<Vec<InvitationWithDetails>, AppError> {
        let rows: Vec<InvitationDetailsRow> = sqlx::query_as(
            "SELECT i.id, i.team_id, t.name AS team_name, i.inviter_id, u.fullname AS inviter_fullname, i.invitee_email, i.status, i.created_at FROM hackathon_team_invitations i JOIN hackathon_teams t ON t.id = i.team_id JOIN hackathon_users u ON u.id = i.inviter_id WHERE i.invitee_email = $1 AND i.status = 'pending'"
        )
        .bind(email).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_invitations SET status = $1 WHERE id = $2")
            .bind(status).bind(id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn reject_pending_for_email_except(&self, email: &str, except_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_invitations SET status = 'rejected' WHERE invitee_email = $1 AND status = 'pending' AND id != $2")
            .bind(email).bind(except_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn add_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("INSERT INTO hackathon_team_members (id, team_id, user_id, role, status, joined_at) VALUES ($1, $2, $3, 'member', 'active', NOW())")
            .bind(Uuid::new_v4()).bind(team_id).bind(user_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn reject_pending_join_requests_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_join_requests SET status = 'rejected' WHERE user_id = $1 AND status = 'pending'")
            .bind(user_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn get_team_leader_id(&self, team_id: Uuid) -> Result<Option<Uuid>, AppError> {
        sqlx::query_scalar("SELECT leader_id FROM hackathon_teams WHERE id = $1")
            .bind(team_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_team_name(&self, team_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT name FROM hackathon_teams WHERE id = $1")
            .bind(team_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_user_email(&self, user_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT email FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_inviter_name(&self, user_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT fullname FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn active_member_count(&self, team_id: Uuid) -> Result<i64, AppError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_team_members WHERE team_id = $1 AND status = 'active'")
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
}
