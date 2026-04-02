use crate::certificates::domain::entity::CertificateData;
use crate::certificates::domain::repository::CertificateRepository;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

#[derive(FromRow)]
struct CertificateRow {
	user_id: Uuid,
	fullname: String,
	email: String,
	avatar: Option<String>,
	team_id: Option<Uuid>,
	team_name: Option<String>,
	is_leader: Option<bool>,
	project_name: Option<String>,
	submission_status: Option<String>,
	winner_rank: Option<i32>,
	winner_prize: Option<String>,
}

impl From<CertificateRow> for CertificateData {
	fn from(r: CertificateRow) -> Self {
		Self {
			user_id: r.user_id,
			fullname: r.fullname,
			email: r.email,
			avatar: r.avatar,
			team_id: r.team_id,
			team_name: r.team_name,
			is_leader: r.is_leader,
			project_name: r.project_name,
			submission_status: r.submission_status,
			winner_rank: r.winner_rank,
			winner_prize: r.winner_prize,
		}
	}
}

pub struct PostgresCertificateRepository {
	pool: Arc<PgPool>,
}

impl PostgresCertificateRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl CertificateRepository for PostgresCertificateRepository {
	async fn find_by_user_id(
		&self,
		user_id: Uuid,
	) -> Result<Option<CertificateData>, AppError> {
		let row: Option<CertificateRow> = sqlx::query_as(
            "SELECT u.id as user_id, u.fullname, u.email, u.avatar, t.id as team_id, t.name as team_name, (t.leader_id = u.id) as is_leader, ps.project_name, ps.status as submission_status, w.rank as winner_rank, w.prize as winner_prize FROM hackathon_users u LEFT JOIN hackathon_team_members tm ON tm.user_id = u.id AND tm.status = 'active' LEFT JOIN hackathon_teams t ON t.id = tm.team_id LEFT JOIN hackathon_project_submissions ps ON ps.team_id = t.id LEFT JOIN hackathon_winners w ON w.team_id = t.id WHERE u.id = $1 LIMIT 1"
        )
        .bind(user_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(row.map(Into::into))
	}
}
