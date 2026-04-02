use crate::submissions::domain::entity::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmissionResponse {
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

impl From<SubmissionEntity> for SubmissionResponse {
	fn from(e: SubmissionEntity) -> Self {
		Self {
			id: e.id,
			team_id: e.team_id,
			project_name: e.project_name,
			description: e.description,
			repository_url: e.repository_url,
			demo_url: e.demo_url,
			presentation_url: e.presentation_url,
			screenshots: e.screenshots,
			status: e.status,
			submitted_at: e.submitted_at,
			submitted_by: e.submitted_by,
			created_at: e.created_at,
			updated_at: e.updated_at,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateSubmissionRequest {
	pub project_name: String,
	pub description: String,
	pub repository_url: String,
	pub demo_url: Option<String>,
	pub presentation_url: Option<String>,
	pub screenshots: Option<Vec<String>>,
}

impl From<CreateSubmissionRequest> for CreateSubmissionInput {
	fn from(r: CreateSubmissionRequest) -> Self {
		Self {
			project_name: r.project_name,
			description: r.description,
			repository_url: r.repository_url,
			demo_url: r.demo_url,
			presentation_url: r.presentation_url,
			screenshots: r.screenshots,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSubmissionRequest {
	pub project_name: Option<String>,
	pub description: Option<String>,
	pub repository_url: Option<String>,
	pub demo_url: Option<String>,
	pub presentation_url: Option<String>,
	pub screenshots: Option<Vec<String>>,
}

impl From<UpdateSubmissionRequest> for UpdateSubmissionInput {
	fn from(r: UpdateSubmissionRequest) -> Self {
		Self {
			project_name: r.project_name,
			description: r.description,
			repository_url: r.repository_url,
			demo_url: r.demo_url,
			presentation_url: r.presentation_url,
			screenshots: r.screenshots,
		}
	}
}
