use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "app_migration_status")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(column_type = "Text")]
	pub resource_type: String,

	#[sea_orm(column_type = "Text")]
	pub status: String,

	#[sea_orm(column_type = "Json", default = "null")]
	pub validation_results: Option<serde_json::Value>,

	#[sea_orm(column_type = "Text", default = "null")]
	pub last_error: Option<String>,

	#[sea_orm(column_type = "Integer", default = 0)]
	pub total_records: i32,

	#[sea_orm(column_type = "Integer", default = 0)]
	pub validated_records: i32,

	#[sea_orm(column_type = "Integer", default = 0)]
	pub failed_records: i32,

	#[sea_orm(column_type = "Integer", default = 0)]
	pub skipped_records: i32,

	#[sea_orm(column_type = "Text", default = "null")]
	pub validation_mode: Option<String>,

	#[sea_orm(column_type = "Timestamp", default = "now()")]
	pub last_validated_at: DateTime<Utc>,

	#[sea_orm(column_type = "Timestamp", default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(column_type = "Timestamp", default = "now()")]
	pub updated_at: DateTime<Utc>,

	#[sea_orm(column_type = "Timestamp", default = "null")]
	pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod status {
	pub const PENDING: &str = "pending";
	pub const IN_PROGRESS: &str = "in_progress";
	pub const COMPLETED: &str = "completed";
	pub const FAILED: &str = "failed";
	pub const PARTIAL: &str = "partial";
	pub const SKIPPED: &str = "skipped";
}

pub mod validation_mode {
	pub const FULL: &str = "full";
	pub const INCREMENTAL: &str = "incremental";
	pub const QUICK_CHECK: &str = "quick_check";
}

pub mod resource_type {
	pub const USERS: &str = "users";
	pub const ROLES: &str = "roles";
	pub const PERMISSIONS: &str = "permissions";
	pub const ROLES_PERMISSIONS: &str = "roles_permissions";
	pub const GACHA_ITEMS: &str = "gacha_items";
	pub const GACHA_CLAIMS: &str = "gacha_claims";
	pub const GACHA_ROLLS: &str = "gacha_rolls";
	pub const GACHA_CREDITS: &str = "gacha_credits";
	pub const NOTIFICATIONS: &str = "notifications";
	pub const AUDIT_LOG: &str = "audit_log";
	pub const SESSIONS: &str = "sessions";
	pub const OTP_CACHE: &str = "otp_cache";
	pub const USERS_CACHE: &str = "users_cache";
	pub const RATE_LIMIT: &str = "rate_limit";
	pub const TESTIMONIALS: &str = "testimonials";
	pub const MENTORS: &str = "mentors";
	pub const EVENTS: &str = "events";
}
