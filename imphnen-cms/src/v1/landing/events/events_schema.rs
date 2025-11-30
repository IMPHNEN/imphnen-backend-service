use imphnen_utils::get_iso_date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::events_dto::{
	EventsCreateRequestDto, EventsQueryDto, EventsUpdateRequestDto,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventsSchema {
	pub id: String,
	pub price: f64,
	pub is_online: bool,
	pub is_deleted: bool,
	pub name: String,
	pub end_date: String,
	pub start_date: String,
	pub created_at: String,
	pub updated_at: String,
	pub description: String,
	pub detail_link: String,
	pub location: Option<String>,
}

impl Default for EventsSchema {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			name: String::new(),
			description: String::new(),
			detail_link: String::new(),
			price: 0.0,
			location: None,
			is_online: false,
			is_deleted: false,
			start_date: String::new(),
			end_date: String::new(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl EventsSchema {
	pub fn from(dto: EventsQueryDto) -> Self {
		Self {
			id: dto.id,
			name: dto.name,
			description: dto.description,
			detail_link: dto.detail_link,
			price: dto.price,
			location: dto.location,
			is_online: dto.is_online,
			is_deleted: false,
			start_date: dto.start_date,
			end_date: dto.end_date,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}

	pub fn create(payload: EventsCreateRequestDto) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			name: payload.name,
			description: payload.description,
			detail_link: payload.detail_link,
			price: payload.price,
			location: payload.location,
			is_online: payload.is_online,
			is_deleted: false,
			end_date: payload.end_date.to_string(),
			start_date: payload.start_date.to_string(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(payload: EventsUpdateRequestDto, id: String) -> Self {
		Self {
			id,
			name: payload.name,
			price: payload.price,
			location: payload.location,
			is_online: payload.is_online,
			description: payload.description,
			detail_link: payload.detail_link,
			end_date: payload.end_date.to_string(),
			start_date: payload.start_date.to_string(),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}
}
