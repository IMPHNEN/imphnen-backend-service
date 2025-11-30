use imphnen_utils::get_iso_date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::testimonials_dto::{
        TestimonialsCreateRequestDto, TestimonialsQueryDto, TestimonialsUpdateRequestDto,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestimonialsSchema {
        pub id: String,
        pub user_id: String,
        pub role: String,
        pub content: String,
        pub is_deleted: bool,
        pub created_at: String,
        pub updated_at: String,
}

impl Default for TestimonialsSchema {
        fn default() -> Self {
                Self {
                        id: Uuid::new_v4().to_string(),
                        user_id: Uuid::new_v4().to_string(),
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
                        user_id: dto.user_id,
                        role: dto.role,
                        content: dto.content,
                        is_deleted: dto.is_deleted,
                        created_at: dto.created_at,
                        updated_at: dto.updated_at,
                }
        }

        pub fn create(payload: TestimonialsCreateRequestDto, user_id: &str) -> Self {
                Self {
                        id: Uuid::new_v4().to_string(),
                        user_id: user_id.to_string(),
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
                user_id: &str,
        ) -> Self {
                Self {
                        id,
                        role: payload.role,
                        content: payload.content,
                        updated_at: get_iso_date(),
                        user_id: user_id.to_string(),
                        ..Default::default()
                }
        }
}
