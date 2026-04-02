use crate::winners::domain::entity::WinnerData;
use crate::winners::domain::repository::WinnerRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use imphnen_utils::errors::AppError;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(FromRow)]
struct WinnerRow {
	id: Uuid,
	team_id: Uuid,
	team_name: String,
	rank: i32,
	prize: Option<String>,
	announced_at: Option<DateTime<Utc>>,
	created_at: Option<DateTime<Utc>>,
}

impl From<WinnerRow> for WinnerData {
	fn from(r: WinnerRow) -> Self {
		Self {
			id: r.id,
			team_id: r.team_id,
			team_name: r.team_name,
			rank: r.rank,
			prize: r.prize,
			announced_at: r.announced_at,
			created_at: r.created_at,
		}
	}
}

pub struct PostgresWinnerRepository {
	pool: Arc<PgPool>,
}

impl PostgresWinnerRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl WinnerRepository for PostgresWinnerRepository {
	async fn list_winners(&self) -> Result<Vec<WinnerData>, AppError> {
		let rows: Vec<WinnerRow> = sqlx::query_as(
            "SELECT w.id, w.team_id, t.name as team_name, w.rank, w.prize, w.announced_at, w.created_at FROM hackathon_winners w JOIN hackathon_teams t ON w.team_id = t.id ORDER BY w.rank ASC"
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(rows.into_iter().map(Into::into).collect())
	}
}
