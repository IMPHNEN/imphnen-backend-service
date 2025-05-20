use crate::{ResourceEnum, make_thing};
use imphnen_iam::get_iso_date;
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

use super::GachaItemRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaItemSchema {
	pub id: Thing,
	pub name: String,
	pub image_url: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaItemSchema {
	fn default() -> Self {
		GachaItemSchema {
			id: make_thing(
				&ResourceEnum::GachaItems.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			image_url: String::new(),
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}

impl GachaItemSchema {
	pub fn from(dto: GachaItemRequestDto) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::GachaItems.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: dto.name,
			image_url: dto.image_url,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}
