use lazy_static::lazy_static;
lazy_static! {
	pub static ref VALID_URL_REGEX: regex::Regex =
		regex::Regex::new(r"^https?://").unwrap();
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct EventsCreateRequestDto {
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,

	#[validate(length(min = 1, message = "Description is required"))]
	pub description: String,

	#[validate(url(message = "Detail link must be a valid URL"))]
	pub detail_link: String,

	#[validate(range(min = 0.0, message = "Price cannot be negative"))]
	pub price: f64,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub end_date: DateTime<Utc>,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub start_date: DateTime<Utc>,

	pub location: Option<String>,
	pub is_online: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct EventsUpdateRequestDto {
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub end_date: DateTime<Utc>,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub start_date: DateTime<Utc>,

	#[validate(range(min = 0.0, message = "Price cannot be negative"))]
	pub price: f64,
	pub is_online: bool,
	pub description: String,
	#[validate(url(message = "Detail link must be a valid URL"))]
	pub detail_link: String,
	pub location: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsListItemDto {
	pub id: String,
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	pub is_online: bool,
	pub start_date: String,
	pub end_date: String,
	pub created_at: String,
	pub location: Option<String>,
	pub is_deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsDetailItemDto {
	pub id: String,
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	pub is_online: bool,
	pub start_date: String,
	pub end_date: String,
	pub created_at: String,
	pub updated_at: String,
	pub location: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventsQueryDto {
	pub id: Thing,
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	pub is_online: bool,
	pub is_deleted: bool,
	pub start_date: String,
	pub end_date: String,
	pub created_at: String,
	pub updated_at: String,
	pub location: Option<String>,
}

impl EventsQueryDto {
	pub fn from(self) -> EventsListItemDto {
		EventsListItemDto {
			id: self.id.id.to_raw(),
			name: self.name,
			description: self.description,
			detail_link: self.detail_link,
			price: self.price,
			location: self.location,
			is_online: self.is_online,
			start_date: self.start_date,
			end_date: self.end_date,
			created_at: self.created_at,
			is_deleted: self.is_deleted,
		}
	}
}
