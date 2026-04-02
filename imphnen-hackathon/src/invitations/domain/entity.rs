use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InvitationEntity {
	pub id: Uuid,
	pub team_id: Uuid,
	pub inviter_id: Uuid,
	pub invitee_email: String,
	pub status: String,
	pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct InvitationWithDetails {
	pub id: Uuid,
	pub team_id: Uuid,
	pub team_name: String,
	pub inviter_id: Uuid,
	pub inviter_fullname: String,
	pub invitee_email: String,
	pub status: String,
	pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default)]
pub struct CreateInvitationInput {
	pub invitee_email: String,
}
