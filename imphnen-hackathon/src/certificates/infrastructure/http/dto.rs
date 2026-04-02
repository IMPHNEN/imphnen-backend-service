use crate::certificates::domain::entity::CertificateData;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct CertificateResponse {
	pub user_id: Uuid,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub team_id: Option<Uuid>,
	pub team_name: Option<String>,
	pub is_leader: Option<bool>,
	pub project_name: Option<String>,
	pub submission_status: Option<String>,
	pub winner_rank: Option<i32>,
	pub winner_prize: Option<String>,
}

impl From<CertificateData> for CertificateResponse {
	fn from(d: CertificateData) -> Self {
		Self {
			user_id: d.user_id,
			fullname: d.fullname,
			email: d.email,
			avatar: d.avatar,
			team_id: d.team_id,
			team_name: d.team_name,
			is_leader: d.is_leader,
			project_name: d.project_name,
			submission_status: d.submission_status,
			winner_rank: d.winner_rank,
			winner_prize: d.winner_prize,
		}
	}
}
