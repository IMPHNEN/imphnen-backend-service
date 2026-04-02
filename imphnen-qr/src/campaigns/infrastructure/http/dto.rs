use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CampaignResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub created_by: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
