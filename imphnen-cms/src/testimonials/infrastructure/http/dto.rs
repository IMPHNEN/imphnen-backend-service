use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;
use crate::testimonials::domain::testimonial::TestimonialEntity;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct TestimonialsCreateRequestDto {
    #[zod(min_length(1), max_length(100))]
    pub role: String,
    #[zod(min_length(1), max_length(1000))]
    pub content: String,
}

impl ZodValidate for TestimonialsCreateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct TestimonialsUpdateRequestDto {
    #[zod(min_length(1), max_length(100))]
    pub role: String,
    #[zod(min_length(1), max_length(1000))]
    pub content: String,
}

impl ZodValidate for TestimonialsUpdateRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
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

impl From<TestimonialEntity> for TestimonialsListItemDto {
    fn from(e: TestimonialEntity) -> Self {
        TestimonialsListItemDto {
            id: e.id.to_string(),
            user_id: e.user_id.to_string(),
            user_fullname: e.user_fullname,
            role: e.role,
            content: e.content,
            created_at: e.created_at,
            is_deleted: e.is_deleted,
        }
    }
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

impl From<TestimonialEntity> for TestimonialsDetailItemDto {
    fn from(e: TestimonialEntity) -> Self {
        TestimonialsDetailItemDto {
            id: e.id.to_string(),
            user_id: e.user_id.to_string(),
            user_fullname: e.user_fullname,
            role: e.role,
            content: e.content,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
