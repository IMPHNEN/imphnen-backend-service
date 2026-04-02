use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CampaignEntity {
	pub id: Uuid,
	pub name: String,
	pub url: String,
	pub is_active: bool,
	pub created_by: Uuid,
	pub expires_at: DateTime<Utc>,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

pub struct CreateCampaignInput {
	pub name: String,
	pub url: String,
	pub created_by: Uuid,
	pub qr_code_data: Vec<u8>,
}
