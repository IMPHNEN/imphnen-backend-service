use crate::submissions::domain::entity::*;
use crate::submissions::domain::repository::SubmissionRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use imphnen_utils::errors::AppError;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(FromRow)]
struct SubmissionRow {
	id: Uuid,
	team_id: Uuid,
	project_name: String,
	description: String,
	repository_url: String,
	demo_url: Option<String>,
	presentation_url: Option<String>,
	screenshots: Option<Vec<String>>,
	status: String,
	submitted_at: Option<DateTime<Utc>>,
	submitted_by: Uuid,
	created_at: Option<DateTime<Utc>>,
	updated_at: Option<DateTime<Utc>>,
}

impl From<SubmissionRow> for SubmissionEntity {
	fn from(r: SubmissionRow) -> Self {
		Self {
			id: r.id,
			team_id: r.team_id,
			project_name: r.project_name,
			description: r.description,
			repository_url: r.repository_url,
			demo_url: r.demo_url,
			presentation_url: r.presentation_url,
			screenshots: r.screenshots,
			status: r.status,
			submitted_at: r.submitted_at,
			submitted_by: r.submitted_by,
			created_at: r.created_at,
			updated_at: r.updated_at,
		}
	}
}

pub struct PostgresSubmissionRepository {
	pool: Arc<PgPool>,
}
impl PostgresSubmissionRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl SubmissionRepository for PostgresSubmissionRepository {
	async fn create(
		&self,
		team_id: Uuid,
		user_id: Uuid,
		input: CreateSubmissionInput,
	) -> Result<SubmissionEntity, AppError> {
		let id = Uuid::new_v4();
		let now = Utc::now();
		let row: SubmissionRow = sqlx::query_as(
            "INSERT INTO hackathon_project_submissions (id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_by, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'draft', $9, $10, $11) RETURNING id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_at, submitted_by, created_at, updated_at"
        )
        .bind(id).bind(team_id).bind(&input.project_name).bind(&input.description)
        .bind(&input.repository_url).bind(&input.demo_url).bind(&input.presentation_url)
        .bind(&input.screenshots).bind(user_id).bind(now).bind(now)
        .fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(row.into())
	}

	async fn find_by_team(
		&self,
		team_id: Uuid,
	) -> Result<Option<SubmissionEntity>, AppError> {
		let row: Option<SubmissionRow> = sqlx::query_as(
            "SELECT id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_at, submitted_by, created_at, updated_at FROM hackathon_project_submissions WHERE team_id = $1 LIMIT 1"
        )
        .bind(team_id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(row.map(Into::into))
	}

	async fn find_by_id(&self, id: Uuid) -> Result<SubmissionEntity, AppError> {
		let row: SubmissionRow = sqlx::query_as(
            "SELECT id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_at, submitted_by, created_at, updated_at FROM hackathon_project_submissions WHERE id = $1"
        )
        .bind(id).fetch_optional(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError("Submission not found".to_string()))?;
		Ok(row.into())
	}

	async fn update(
		&self,
		id: Uuid,
		input: UpdateSubmissionInput,
	) -> Result<SubmissionEntity, AppError> {
		let mut sets = vec!["updated_at = $1".to_string()];
		let mut idx = 2usize;
		if input.project_name.is_some() {
			sets.push(format!("project_name = ${}", idx));
			idx += 1;
		}
		if input.description.is_some() {
			sets.push(format!("description = ${}", idx));
			idx += 1;
		}
		if input.repository_url.is_some() {
			sets.push(format!("repository_url = ${}", idx));
			idx += 1;
		}
		if input.demo_url.is_some() {
			sets.push(format!("demo_url = ${}", idx));
			idx += 1;
		}
		if input.presentation_url.is_some() {
			sets.push(format!("presentation_url = ${}", idx));
			idx += 1;
		}
		if input.screenshots.is_some() {
			sets.push(format!("screenshots = ${}", idx));
			idx += 1;
		}
		let sql = format!(
			"UPDATE hackathon_project_submissions SET {} WHERE id = ${} RETURNING id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_at, submitted_by, created_at, updated_at",
			sets.join(", "),
			idx
		);
		let mut q = sqlx::query_as::<_, SubmissionRow>(&sql).bind(Utc::now());
		if let Some(v) = input.project_name {
			q = q.bind(v);
		}
		if let Some(v) = input.description {
			q = q.bind(v);
		}
		if let Some(v) = input.repository_url {
			q = q.bind(v);
		}
		if let Some(v) = input.demo_url {
			q = q.bind(v);
		}
		if let Some(v) = input.presentation_url {
			q = q.bind(v);
		}
		if let Some(v) = input.screenshots {
			q = q.bind(v);
		}
		q.bind(id)
			.fetch_one(self.pool.as_ref())
			.await
			.map(Into::into)
			.map_err(|e| AppError::InternalServerError(e.to_string()))
	}

	async fn update_status(
		&self,
		id: Uuid,
		status: &str,
	) -> Result<SubmissionEntity, AppError> {
		let row: SubmissionRow = sqlx::query_as(
            "UPDATE hackathon_project_submissions SET status = $1, submitted_at = CASE WHEN $1 = 'submitted' THEN NOW() ELSE submitted_at END, updated_at = NOW() WHERE id = $2 RETURNING id, team_id, project_name, description, repository_url, demo_url, presentation_url, screenshots, status, submitted_at, submitted_by, created_at, updated_at"
        )
        .bind(status).bind(id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(row.into())
	}

	async fn is_team_leader(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<bool, AppError> {
		sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM hackathon_teams WHERE id = $1 AND leader_id = $2)",
		)
		.bind(team_id)
		.bind(user_id)
		.fetch_one(self.pool.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))
	}

	async fn is_team_member(
		&self,
		team_id: Uuid,
		user_id: Uuid,
	) -> Result<bool, AppError> {
		sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM hackathon_team_members WHERE team_id = $1 AND user_id = $2 AND status = 'active')")
            .bind(team_id).bind(user_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
	}

	async fn team_member_count(&self, team_id: Uuid) -> Result<i64, AppError> {
		sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_team_members WHERE team_id = $1 AND status = 'active'")
            .bind(team_id).fetch_one(self.pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))
	}
}
