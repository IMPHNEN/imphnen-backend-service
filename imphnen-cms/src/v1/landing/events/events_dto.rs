use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// Custom URL validator that ensures valid HTTP/HTTPS URLs
static VALID_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^https?://[^\s$.?#].[^\s]*$").unwrap());

pub fn validate_url(url: &str) -> Result<(), ValidationError> {
	if VALID_URL_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_url"))
	}
}

// Custom validator for future dates
pub fn validate_future_date(end_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    let now = Utc::now();
    if end_date > &now {
        Ok(())
    } else {
        Err(ValidationError::new("future_date"))
    }
}

// Custom validator for event date ranges (for combined validation)
pub fn validate_date_range(start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    if start_date <= end_date {
        Ok(())
    } else {
        Err(ValidationError::new("date_range"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct EventsCreateRequestDto {
	#[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
	pub name: String,
	
	#[validate(length(min = 1, max = 1000, message = "Description must be between 1 and 1000 characters"))]
	pub description: String,
	
	#[validate(custom(
		function = "validate_url",
		message = "Detail link must be a valid HTTP/HTTPS URL"
	))]
	pub detail_link: String,
	
	#[validate(range(min = 0.0, max = 1_000_000.0, message = "Price must be between 0 and 1,000,000"))]
	pub price: f64,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	#[validate(custom(
	    function = "validate_future_date",
	    message = "End date must be in the future"
	))]
	pub end_date: DateTime<Utc>,

	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub start_date: DateTime<Utc>,

	#[validate(length(max = 200, message = "Location name cannot exceed 200 characters"))]
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
	pub id: String,
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
			id: self.id,
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
