use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

// Custom validator for image URLs
pub fn validate_image_url(url: &str) -> Result<(), ValidationError> {
	lazy_static! {
		static ref IMAGE_URL_REGEX: Regex = Regex::new(r"^https?://[^\s]+\.(jpg|jpeg|png|gif|webp)$").unwrap();
	}
	if IMAGE_URL_REGEX.is_match(url) {
		Ok(())
	} else {
		Err(ValidationError::new("invalid_image_url"))
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemRequestDto {
	#[validate(length(min = 1, max = 100, message = "Item name must be between 1 and 100 characters"))]
	pub name: String,
	
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
			id: dto.id.id.to_raw(),
			name: dto.name,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}
