use sea_orm::prelude::*;
use sea_orm::{ActiveValue, ActiveModelTrait, QueryOrder};
use std::time::Instant;
use tracing::instrument;
use uuid::Uuid; // Added Uuid import
use imphnen_entities::seaorm::common::testimonials::{Entity as TestimonialsEntity, Column as TestimonialsColumn};
use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;
use imphnen_libs::{AppState, MetaRequestDto, ResponseListSuccessDto, AppStatePostgresExt};
use imphnen_utils::AppError;
use imphnen_utils::Result;
use crate::testimonials::testimonials_schema::TestimonialsSchema;
use crate::testimonials::testimonials_dto::TestimonialsQueryDto;

pub struct TestimonialsRepository<'a> {
	state: &'a AppState,
}

impl<'a> TestimonialsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	fn get_db(&self) -> &DatabaseConnection {
		self.state.postgres_db()
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_testimonial_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TestimonialsQueryDto>>> {
		let now = Instant::now();
		let db = self.get_db();

		let page = meta.page.unwrap_or(1);
		let per_page = meta.per_page.unwrap_or(10);

		let mut query = TestimonialsEntity::find()
			.filter(TestimonialsColumn::IsDeleted.eq(false))
            .find_also_related(UsersEntity);		// Apply sorting
		if let Some(sort_by) = &meta.sort_by {
			let order = meta.order.as_deref().unwrap_or("asc");
			match sort_by.as_str() {
				"created_at" => {
					if order == "desc" {
						query = query.order_by_desc(TestimonialsColumn::CreatedAt);
					} else {
						query = query.order_by_asc(TestimonialsColumn::CreatedAt);
					}
				}
					"updated_at" => {
						if order == "desc" {
							query = query.order_by_desc(TestimonialsColumn::UpdatedAt);
						} else {
							query = query.order_by_asc(TestimonialsColumn::UpdatedAt);
					}
				}
				_ => {
					query = query.order_by_desc(TestimonialsColumn::CreatedAt);
				}
			}
		} else {
			query = query.order_by_desc(TestimonialsColumn::CreatedAt);
		}

		let paginator = query.paginate(db, per_page);
		let total_pages = paginator.num_pages().await?;
		let testimonials = paginator.fetch_page(page - 1).await?;

		let data: Vec<TestimonialsQueryDto> = testimonials
			.into_iter()
			.filter_map(|(testimonial, user)| {
				user.map(|u| TestimonialsQueryDto {
					id: testimonial.id.to_string(),
					user_id: testimonial.user_id.to_string(),
					user_fullname: format!("{} {}", u.first_name.as_deref().unwrap_or(""), u.last_name.as_deref().unwrap_or("")).trim().to_string(),
					role: testimonial.role,
					content: testimonial.content,
					is_deleted: testimonial.is_deleted,
					created_at: testimonial.created_at.to_rfc3339(),
					updated_at: testimonial.updated_at.to_rfc3339(),
				})
			})
			.collect();

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_testimonial_list' took: {elapsed:.2?}");
		}

		let response = ResponseListSuccessDto {
			data,
			meta: Some(imphnen_entities::MetaResponseDto {
				page: Some(page),
				per_page: Some(per_page),
				total: Some(total_pages),
			}),
		};

		Ok(response)
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_testimonial_by_id(
		&self,
		id: Uuid,
	) -> Result<TestimonialsQueryDto> {
		let now = Instant::now();
		let db = self.get_db();

		let (testimonial, user) = TestimonialsEntity::find_by_id(id)
			.filter(TestimonialsColumn::IsDeleted.eq(false))
			.find_also_related(UsersEntity)
			.one(db)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Testimonial not found"))?;

		let user = user.ok_or_else(|| anyhow::anyhow!("User not found for testimonial"))?;

		let result = TestimonialsQueryDto {
			id: testimonial.id.to_string(),
			user_id: testimonial.user_id.to_string(),
			user_fullname: format!("{} {}", user.first_name.as_deref().unwrap_or(""), user.last_name.as_deref().unwrap_or("")).trim().to_string(),
			role: testimonial.role,
			content: testimonial.content,
			is_deleted: testimonial.is_deleted,
			created_at: testimonial.created_at.to_rfc3339(),
			updated_at: testimonial.updated_at.to_rfc3339(),
		};

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_testimonial_by_id' took: {elapsed:.2?}");
		}

		Ok(result)
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<TestimonialsSchema> {
		let now = Instant::now();
		let db = self.get_db();

		let active_model = imphnen_entities::seaorm::common::testimonials::ActiveModel {
			id: ActiveValue::Set(Uuid::parse_str(&data.id)?),
			user_id: ActiveValue::Set(Uuid::parse_str(&data.user_id)?),
			role: ActiveValue::Set(data.role.clone()),
			content: ActiveValue::Set(data.content.clone()),
			is_deleted: ActiveValue::Set(data.is_deleted),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
		};

		let inserted = active_model.insert(db).await?;
		let created_testimonial = TestimonialsSchema {
			id: inserted.id.to_string(),
			user_id: inserted.user_id.to_string(),
			role: inserted.role,
			content: inserted.content,
			is_deleted: inserted.is_deleted,
			created_at: inserted.created_at.to_rfc3339(),
			updated_at: inserted.updated_at.to_rfc3339(),
		};

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_testimonial' took: {elapsed:.2?}");
		}

		Ok(created_testimonial)
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = self.get_db();

		let existing = self.query_testimonial_by_id(Uuid::parse_str(&data.id)?).await?;
		if existing.is_deleted {
			return Err(AppError::BadRequestError("Testimonial already deleted".to_string()));
		}

		let mut active_model: imphnen_entities::seaorm::common::testimonials::ActiveModel = TestimonialsEntity::find_by_id(Uuid::parse_str(&data.id)?)
			.one(db)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Testimonial not found"))?
			.into();

		active_model.role = ActiveValue::Set(data.role.clone());
		active_model.content = ActiveValue::Set(data.content.clone());
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model.update(db).await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_testimonial' took: {elapsed:.2?}");
		}

		Ok("Success update testimonial".into())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_testimonial(&self, id: Uuid) -> Result<String> {
		let now = Instant::now();
		let db = self.get_db();

		let testimonial = self.query_testimonial_by_id(id).await?;
		if testimonial.is_deleted {
			return Err(AppError::NotFoundError("Testimonial not found".to_string()));
		}

		let mut active_model: imphnen_entities::seaorm::common::testimonials::ActiveModel = TestimonialsEntity::find_by_id(id)
			.one(db)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Testimonial not found".to_string()))?
			.into();

		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model.update(db).await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_testimonial' took: {elapsed:.2?}");
		}

		Ok("Success delete testimonial".into())
	}
}
