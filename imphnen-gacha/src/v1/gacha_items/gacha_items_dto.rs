use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value; // Add this import
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// Custom validator for image URLs
static IMAGE_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^https?://[^\s]+\.(jpg|jpeg|png|gif|webp)$").unwrap());

pub fn validate_image_url(url: &str) -> Result<(), ValidationError> {
	if IMAGE_URL_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_image_url"))
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemRequestDto {
	#[validate(length(min = 1, max = 100, message = "Item code must be between 1 and 100 characters"))]
	pub item_code: String,

	#[validate(length(min = 1, max = 100, message = "Item name must be between 1 and 100 characters"))]
	pub name: String,
	
	#[validate(length(min = 1, max = 500, message = "Description must be between 1 and 500 characters"))]
	pub description: String,

	#[validate(length(min = 1, max = 50, message = "Rarity must be between 1 and 50 characters"))]
	pub rarity: String,

	#[validate(length(min = 1, max = 50, message = "Type must be between 1 and 50 characters"))]
	pub type_: String,

	#[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
	pub category: String,

	#[validate(range(min = 0, message = "Value must be non-negative"))]
	pub value: i32,

	#[validate(range(min = 0.0, message = "Weight must be non-negative"))]
	pub weight: f64,

	#[validate(range(min = 0, message = "Stock must be non-negative"))]
	pub stock: i32,

	pub is_limited: bool,

	pub metadata: Option<Value>,

	#[validate(length(min = 1, message = "Image URL must not be empty"))]
	#[validate(custom(
		function = "validate_image_url",
		message = "Image URL must be a valid URL pointing to JPG, JPEG, PNG, GIF, or WebP image"
	))]
	pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemUpdateRequestDto {
	#[validate(length(min = 1, max = 100, message = "Item name must be between 1 and 100 characters"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	
	#[validate(length(min = 1, message = "Image URL must not be empty"))]
	#[validate(custom(
		function = "validate_image_url",
		message = "Image URL must be a valid URL pointing to JPG, JPEG, PNG, GIF, or WebP image"
	))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl GachaItemDto {
	pub fn from(dto: GachaItemSchema) -> Self {
		Self {
			id: dto.id.to_string(),
			name: dto.name,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}
