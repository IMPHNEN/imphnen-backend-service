use imphnen_libs::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::Uuid;
use surrealdb::sql::Thing;

use super::testimonials_dto::{
	TestimonialsCreateRequestDto, TestimonialsQueryDto, TestimonialsUpdateRequestDto,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestimonialsSchema {
	pub id: Thing,
	pub user: Thing,
	pub role: String,
	pub content: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for TestimonialsSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Testimonials.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			role: String::new(),
			content: String::new(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl TestimonialsSchema {
	pub fn from(dto: TestimonialsQueryDto) -> Self {
		Self {
			id: dto.id,
			user: dto.user.id,
			role: dto.role,
			content: dto.content,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}

	pub fn create(payload: TestimonialsCreateRequestDto, user_id: &Thing) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Testimonials.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: user_id.clone(),
			role: payload.role,
			content: payload.content,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(
		payload: TestimonialsUpdateRequestDto,
		id: String,
		user_id: &Thing,
	) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Testimonials.to_string(), &id),
			role: payload.role,
			content: payload.content,
			updated_at: get_iso_date(),
			user: user_id.clone(),
			..Default::default()
		}
	}
}
