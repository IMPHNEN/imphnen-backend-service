use crate::admin::domain::entity::*;
use crate::admin::domain::repository::AdminRepository;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresAdminRepository {
	pool: Arc<PgPool>,
}

impl PostgresAdminRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl AdminRepository for PostgresAdminRepository {
	async fn list_users(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminUserRow>, i64), AppError> {
		let offset = (page - 1) * limit;
		let pattern = search.as_deref().map(|s| format!("%{}%", s));
		let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_users WHERE ($1::text IS NULL OR email ILIKE $1 OR fullname ILIKE $1)")
            .bind(&pattern).fetch_one(self.pool.as_ref()).await.unwrap_or(0);
		let users: Vec<AdminUserRow> = sqlx::query_as("SELECT id, email, fullname, avatar, is_active, is_admin, created_at FROM hackathon_users WHERE ($1::text IS NULL OR email ILIKE $1 OR fullname ILIKE $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(&pattern).bind(limit).bind(offset)
            .fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok((users, total))
	}

	async fn get_user(&self, user_id: Uuid) -> Result<Option<AdminUserRow>, AppError> {
		sqlx::query_as("SELECT id, email, fullname, avatar, is_active, is_admin, created_at FROM hackathon_users WHERE id = $1")
            .bind(user_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
	}

	async fn set_admin(&self, user_id: Uuid, is_admin: bool) -> Result<(), AppError> {
		sqlx::query("UPDATE hackathon_users SET is_admin = $1 WHERE id = $2")
			.bind(is_admin)
			.bind(user_id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
		sqlx::query("DELETE FROM hackathon_users WHERE id = $1")
			.bind(user_id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn list_teams(
		&self,
		page: i64,
		limit: i64,
		search: Option<String>,
	) -> Result<(Vec<AdminTeamRow>, i64), AppError> {
		let offset = (page - 1) * limit;
		let pattern = search.as_deref().map(|s| format!("%{}%", s));
		let total: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM hackathon_teams WHERE ($1::text IS NULL OR name ILIKE $1)",
		)
		.bind(&pattern)
		.fetch_one(self.pool.as_ref())
		.await
		.unwrap_or(0);
		let teams: Vec<AdminTeamRow> = sqlx::query_as("SELECT id, name, city, visibility, leader_id, created_at FROM hackathon_teams WHERE ($1::text IS NULL OR name ILIKE $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(&pattern).bind(limit).bind(offset)
            .fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok((teams, total))
	}

	async fn delete_team(&self, team_id: Uuid) -> Result<(), AppError> {
		sqlx::query("DELETE FROM hackathon_teams WHERE id = $1")
			.bind(team_id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn list_submissions(
		&self,
		page: i64,
		limit: i64,
		status: Option<String>,
	) -> Result<(Vec<AdminSubmissionRow>, i64), AppError> {
		let offset = (page - 1) * limit;
		let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_project_submissions WHERE ($1::text IS NULL OR status = $1)")
            .bind(&status).fetch_one(self.pool.as_ref()).await.unwrap_or(0);
		let subs: Vec<AdminSubmissionRow> = sqlx::query_as("SELECT id, team_id, project_name, status, submitted_at, created_at FROM hackathon_project_submissions WHERE ($1::text IS NULL OR status = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
            .bind(&status).bind(limit).bind(offset)
            .fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok((subs, total))
	}

	async fn set_winner(
		&self,
		team_id: Uuid,
		rank: i32,
		prize: Option<String>,
	) -> Result<(), AppError> {
		sqlx::query("INSERT INTO hackathon_winners (id, team_id, rank, prize, announced_at, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW()) ON CONFLICT (team_id) DO UPDATE SET rank = $3, prize = $4, updated_at = NOW()")
            .bind(Uuid::new_v4()).bind(team_id).bind(rank).bind(prize)
            .execute(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn remove_winner(&self, team_id: Uuid) -> Result<(), AppError> {
		sqlx::query("DELETE FROM hackathon_winners WHERE team_id = $1")
			.bind(team_id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn list_winners(&self) -> Result<Vec<WinnerRow>, AppError> {
		sqlx::query_as("SELECT id, team_id, rank, prize, created_at FROM hackathon_winners ORDER BY rank ASC")
            .fetch_all(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
	}
}
