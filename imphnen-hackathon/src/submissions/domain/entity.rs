use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SubmissionEntity {
	pub id: Uuid,
	pub team_id: Uuid,
	pub project_name: String,
	pub description: String,
	pub repository_url: String,
	pub demo_url: Option<String>,
	pub presentation_url: Option<String>,
	pub screenshots: Option<Vec<String>>,
	pub status: String,
	pub submitted_at: Option<DateTime<Utc>>,
	pub submitted_by: Uuid,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default)]
pub struct CreateSubmissionInput {
	pub project_name: String,
	pub description: String,
	pub repository_url: String,
	pub demo_url: Option<String>,
	pub presentation_url: Option<String>,
	pub screenshots: Option<Vec<String>>,
}

#[derive(Debug, Default)]
pub struct UpdateSubmissionInput {
	pub project_name: Option<String>,
	pub description: Option<String>,
	pub repository_url: Option<String>,
	pub demo_url: Option<String>,
	pub presentation_url: Option<String>,
	pub screenshots: Option<Vec<String>>,
}
