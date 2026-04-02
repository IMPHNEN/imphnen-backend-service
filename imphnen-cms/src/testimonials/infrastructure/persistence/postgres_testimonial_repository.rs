use crate::testimonials::domain::{
	repository::TestimonialRepository, testimonial::TestimonialEntity,
};
use async_trait::async_trait;
use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;
use imphnen_entities::seaorm::common::testimonials::{
	ActiveModel as TestimonialsActiveModel, Column as TestimonialsColumn,
	Entity as TestimonialsEntity,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, PaginatorTrait, QueryOrder};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresTestimonialRepository {
	db: Arc<DatabaseConnection>,
}

impl PostgresTestimonialRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl TestimonialRepository for PostgresTestimonialRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<TestimonialEntity>, AppError> {
		let page = params.page.max(1);
		let per_page = params.per_page.clamp(1, 100);

		let mut query = TestimonialsEntity::find()
			.filter(TestimonialsColumn::IsDeleted.eq(false))
			.find_also_related(UsersEntity);

		query = match params.sort_by.as_deref() {
			Some("updated_at") => match params.sort_direction {
				Some(SortDirection::Asc) => {
					query.order_by_asc(TestimonialsColumn::UpdatedAt)
				}
				_ => query.order_by_desc(TestimonialsColumn::UpdatedAt),
			},
			_ => match params.sort_direction {
				Some(SortDirection::Asc) => {
					query.order_by_asc(TestimonialsColumn::CreatedAt)
				}
				_ => query.order_by_desc(TestimonialsColumn::CreatedAt),
			},
		};

		let paginator = query.paginate(self.db.as_ref(), per_page as u64);
		let total = paginator
			.num_items()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		let rows = paginator
			.fetch_page((page - 1) as u64)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let data: Vec<TestimonialEntity> = rows
			.into_iter()
			.filter_map(|(t, u)| {
				u.map(|user| TestimonialEntity {
					id: t.id,
					user_id: t.user_id,
					user_fullname: format!(
						"{} {}",
						user.first_name.as_deref().unwrap_or(""),
						user.last_name.as_deref().unwrap_or("")
					)
					.trim()
					.to_string(),
					role: t.role,
					content: t.content,
					is_deleted: t.is_deleted,
					created_at: t.created_at.to_rfc3339(),
					updated_at: t.updated_at.to_rfc3339(),
				})
			})
			.collect();

		let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
		Ok(PaginatorResponse { data, meta })
	}

	async fn find_by_id(&self, id: Uuid) -> Result<TestimonialEntity, AppError> {
		let (testimonial, user) = TestimonialsEntity::find_by_id(id)
			.filter(TestimonialsColumn::IsDeleted.eq(false))
			.find_also_related(UsersEntity)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Testimonial not found".to_string()))?;

		let user = user.ok_or_else(|| {
			AppError::NotFoundError("User not found for testimonial".to_string())
		})?;

		Ok(TestimonialEntity {
			id: testimonial.id,
			user_id: testimonial.user_id,
			user_fullname: format!(
				"{} {}",
				user.first_name.as_deref().unwrap_or(""),
				user.last_name.as_deref().unwrap_or("")
			)
			.trim()
			.to_string(),
			role: testimonial.role,
			content: testimonial.content,
			is_deleted: testimonial.is_deleted,
			created_at: testimonial.created_at.to_rfc3339(),
			updated_at: testimonial.updated_at.to_rfc3339(),
		})
	}

	async fn create(
		&self,
		entity: TestimonialEntity,
	) -> Result<TestimonialEntity, AppError> {
		let active_model = TestimonialsActiveModel {
			id: ActiveValue::Set(entity.id),
			user_id: ActiveValue::Set(entity.user_id),
			role: ActiveValue::Set(entity.role.clone()),
			content: ActiveValue::Set(entity.content.clone()),
			is_deleted: ActiveValue::Set(false),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
		};

		let inserted = active_model
			.insert(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(TestimonialEntity {
			id: inserted.id,
			user_id: inserted.user_id,
			user_fullname: entity.user_fullname,
			role: inserted.role,
			content: inserted.content,
			is_deleted: inserted.is_deleted,
			created_at: inserted.created_at.to_rfc3339(),
			updated_at: inserted.updated_at.to_rfc3339(),
		})
	}

	async fn update(&self, entity: TestimonialEntity) -> Result<(), AppError> {
		let mut active_model: TestimonialsActiveModel =
			TestimonialsEntity::find_by_id(entity.id)
				.one(self.db.as_ref())
				.await
				.map_err(|e| AppError::InternalServerError(e.to_string()))?
				.ok_or_else(|| AppError::NotFoundError("Testimonial not found".to_string()))?
				.into();

		active_model.role = ActiveValue::Set(entity.role);
		active_model.content = ActiveValue::Set(entity.content);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		let mut active_model: TestimonialsActiveModel =
			TestimonialsEntity::find_by_id(id)
				.one(self.db.as_ref())
				.await
				.map_err(|e| AppError::InternalServerError(e.to_string()))?
				.ok_or_else(|| AppError::NotFoundError("Testimonial not found".to_string()))?
				.into();

		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
