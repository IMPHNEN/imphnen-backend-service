use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use imphnen_utils::errors::AppError;
use crate::teams::domain::entity::*;
use crate::teams::domain::repository::TeamRepository;

#[derive(FromRow)]
pub(crate) struct TeamRow {
    pub id: Uuid, pub name: String, pub description: Option<String>, pub city: String,
    pub visibility: String, pub logo: Option<String>, pub banner: Option<String>,
    pub leader_id: Uuid, pub created_at: Option<DateTime<Utc>>, pub updated_at: Option<DateTime<Utc>>,
}

impl From<TeamRow> for TeamEntity {
    fn from(r: TeamRow) -> Self {
        Self { id: r.id, name: r.name, description: r.description, city: r.city, visibility: r.visibility,
               logo: r.logo, banner: r.banner, leader_id: r.leader_id, created_at: r.created_at, updated_at: r.updated_at }
    }
}

#[derive(FromRow)]
pub(crate) struct UserRow {
    pub id: Uuid, pub email: String, pub fullname: String, pub avatar: Option<String>,
    pub phone_number: Option<String>, pub location: Option<String>, pub bio: Option<String>,
    pub skills: Option<Vec<String>>, pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>, pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserRow> for TeamUserInfo {
    fn from(r: UserRow) -> Self {
        Self { id: r.id, email: r.email, fullname: r.fullname, avatar: r.avatar,
               phone_number: r.phone_number, location: r.location, bio: r.bio,
               skills: r.skills, is_active: r.is_active, created_at: r.created_at, updated_at: r.updated_at }
    }
}

pub struct PostgresTeamRepository { pub(crate) pool: Arc<PgPool> }
impl PostgresTeamRepository { pub fn new(pool: Arc<PgPool>) -> Self { Self { pool } } }

#[async_trait]
impl TeamRepository for PostgresTeamRepository {
    async fn create(&self, id: Uuid, leader_id: Uuid, input: CreateTeamInput) -> Result<TeamEntity, AppError> {
        let now = Utc::now();
        let row: TeamRow = sqlx::query_as(
            "INSERT INTO hackathon_teams (id, name, description, city, visibility, logo, banner, leader_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id, name, description, city, visibility, logo, banner, leader_id, created_at, updated_at"
        )
        .bind(id).bind(&input.name).bind(&input.description).bind(&input.city)
        .bind(&input.visibility).bind(&input.logo).bind(&input.banner).bind(leader_id).bind(now).bind(now)
        .fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TeamEntity>, AppError> {
        let row: Option<TeamRow> = sqlx::query_as(
            "SELECT id, name, description, city, visibility, logo, banner, leader_id, created_at, updated_at FROM hackathon_teams WHERE id = $1"
        )
        .bind(id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<TeamEntity>, AppError> {
        let rows: Vec<TeamRow> = sqlx::query_as(
            "SELECT t.id, t.name, t.description, t.city, t.visibility, t.logo, t.banner, t.leader_id, t.created_at, t.updated_at FROM hackathon_teams t JOIN hackathon_team_members tm ON tm.team_id = t.id WHERE tm.user_id = $1 AND tm.status = 'active'"
        )
        .bind(user_id).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_leader(&self, leader_id: Uuid) -> Result<Option<TeamUserInfo>, AppError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, email, fullname, avatar, phone_number, location, bio, skills, is_active, created_at, updated_at FROM hackathon_users WHERE id = $1"
        )
        .bind(leader_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(row.map(Into::into))
    }

    async fn get_members(&self, team_id: Uuid) -> Result<Vec<TeamMemberEntity>, AppError> {
        #[derive(FromRow)]
        struct MemberRow {
            id: Uuid, team_id: Uuid, user_id: Uuid, role: String, status: String, joined_at: Option<DateTime<Utc>>,
            user_email: String, user_fullname: String, user_avatar: Option<String>,
            user_phone_number: Option<String>, user_location: Option<String>, user_bio: Option<String>,
            user_skills: Option<Vec<String>>, user_is_active: Option<bool>,
            user_created_at: Option<DateTime<Utc>>, user_updated_at: Option<DateTime<Utc>>,
        }
        let rows: Vec<MemberRow> = sqlx::query_as(
            "SELECT tm.id, tm.team_id, tm.user_id, tm.role, tm.status, tm.joined_at, u.email as user_email, u.fullname as user_fullname, u.avatar as user_avatar, u.phone_number as user_phone_number, u.location as user_location, u.bio as user_bio, u.skills as user_skills, u.is_active as user_is_active, u.created_at as user_created_at, u.updated_at as user_updated_at FROM hackathon_team_members tm JOIN hackathon_users u ON tm.user_id = u.id WHERE tm.team_id = $1 AND tm.status = 'active' ORDER BY tm.role DESC, tm.joined_at ASC"
        )
        .bind(team_id).fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(rows.into_iter().map(|r| TeamMemberEntity {
            id: r.id, team_id: r.team_id, user_id: r.user_id, role: r.role, status: r.status, joined_at: r.joined_at,
            user: TeamUserInfo { id: r.user_id, email: r.user_email, fullname: r.user_fullname, avatar: r.user_avatar,
                phone_number: r.user_phone_number, location: r.user_location, bio: r.user_bio,
                skills: r.user_skills, is_active: r.user_is_active, created_at: r.user_created_at, updated_at: r.user_updated_at },
        }).collect())
    }

    async fn add_member(&self, team_id: Uuid, user_id: Uuid, role: &str) -> Result<(), AppError> {
        let now = Utc::now();
        sqlx::query("INSERT INTO hackathon_team_members (id, team_id, user_id, role, status, joined_at) VALUES ($1, $2, $3, $4, 'active', $5) ON CONFLICT (team_id, user_id) DO NOTHING")
            .bind(Uuid::new_v4()).bind(team_id).bind(user_id).bind(role).bind(now)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn remove_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM hackathon_team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id).bind(user_id)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn get_member_count(&self, team_id: Uuid) -> Result<i64, AppError> {
        sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_team_members WHERE team_id = $1 AND status = 'active'")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn is_member(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_team_members WHERE team_id = $1 AND user_id = $2 AND status = 'active')")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn is_leader(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_teams WHERE id = $1 AND leader_id = $2)")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn user_active_team_name(&self, user_id: Uuid) -> Result<Option<String>, AppError> {
        sqlx::query_scalar("SELECT t.name FROM hackathon_teams t JOIN hackathon_team_members tm ON tm.team_id = t.id WHERE tm.user_id = $1 AND tm.status = 'active' LIMIT 1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn team_has_submission(&self, team_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_project_submissions WHERE team_id = $1)")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn reject_pending_invitations_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let email: Option<String> = sqlx::query_scalar("SELECT email FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        if let Some(email) = email {
            sqlx::query("UPDATE hackathon_team_invitations SET status = 'rejected' WHERE invitee_email = $1 AND status = 'pending'")
                .bind(email).execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        }
        Ok(())
    }

    async fn reject_pending_join_requests_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE hackathon_team_join_requests SET status = 'rejected' WHERE user_id = $1 AND status = 'pending'")
            .bind(user_id).execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }

    async fn get_leaders_batch(&self, leader_ids: Vec<Uuid>) -> Result<Vec<TeamUserInfo>, AppError> {
        self.leaders_batch_query(leader_ids).await
    }

    async fn get_member_counts_batch(&self, team_ids: Vec<Uuid>) -> Result<Vec<(Uuid, i64)>, AppError> {
        self.member_counts_batch_query(team_ids).await
    }

    async fn get_submitted_team_ids(&self, team_ids: Vec<Uuid>) -> Result<Vec<Uuid>, AppError> {
        self.submitted_team_ids_query(team_ids).await
    }

    async fn update(&self, id: Uuid, input: UpdateTeamInput) -> Result<TeamEntity, AppError> {
        self.update_query(id, input).await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM hackathon_teams WHERE id = $1")
            .bind(id).execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(result.rows_affected() > 0)
    }

    async fn browse(&self, input: BrowseTeamsInput) -> Result<(Vec<TeamEntity>, i64), AppError> {
        self.browse_query(input).await
    }
}
