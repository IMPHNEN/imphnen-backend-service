use imphnen_iam::v1::users::UsersSchema;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestimonialsCreateRequestDto {
	#[validate(length(min = 1, message = "Role is required"))]
	pub role: String,

	#[validate(length(
		min = 1,
		max = 500,
		message = "Content must be between 1 and 500 characters"
	))]
	pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestimonialsUpdateRequestDto {
	#[validate(length(min = 1, message = "Role is required"))]
	pub role: String,

	#[validate(length(
		min = 1,
		max = 500,
		message = "Content must be between 1 and 500 characters"
	))]
	pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestimonialsListItemDto {
	pub id: String,
	pub user_id: String,
	pub user_fullname: String,
	pub role: String,
	pub content: String,
	pub created_at: String,
	pub is_deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestimonialsDetailItemDto {
	pub id: String,
	pub user_id: String,
	pub user_fullname: String,
	pub role: String,
	pub content: String,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestimonialsQueryDto {
	pub id: Thing,
	pub user: UsersSchema,
	pub role: String,
	pub content: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl TestimonialsQueryDto {
	pub fn from(self) -> TestimonialsListItemDto {
		TestimonialsListItemDto {
			id: self.id.id.to_raw(),
			user_id: self.user.id.id.to_raw(),
			user_fullname: self.user.fullname,
			role: self.role,
			content: self.content,
			created_at: self.created_at,
			is_deleted: self.is_deleted,
		}
	}
}
