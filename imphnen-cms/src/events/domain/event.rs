use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EventEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub detail_link: String,
    pub price: f64,
    pub is_online: bool,
    pub is_deleted: bool,
    pub location: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
