use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CertificateData {
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
