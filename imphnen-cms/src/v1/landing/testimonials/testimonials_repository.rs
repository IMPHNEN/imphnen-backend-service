use super::{
	testimonials_dto::TestimonialsQueryDto, testimonials_schema::TestimonialsSchema,
};
use anyhow::{Result, bail};
use imphnen_libs::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use imphnen_utils::{DetailQueryBuilder, ListQueryBuilder, get_id, get_iso_date};
use serde_json;
use std::time::Instant;
use tracing::instrument;
use tracing::info;

pub struct TestimonialsRepository<'a> {
	state: &'a AppState,
}

impl<'a> TestimonialsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_testimonial_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TestimonialsQueryDto>>> {
		let now = Instant::now();
		let query = ListQueryBuilder::new(ResourceEnum::Testimonials.to_string())
			.with_select_fields(vec!["*", "user.* as user"])
			.with_pagination(meta.page, Some(10))
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.build();
		info!(query = %query, "Executing SurrealDB query");
		let res: Vec<TestimonialsQueryDto> =
			self.state.surrealdb_ws.query(query).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_testimonial_list' took: {elapsed:.2?}");
		}
		let data = ResponseListSuccessDto {
			data: res,
			meta: None,
		};
		Ok(data)
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_testimonial_by_id(
		&self,
		id: String,
	) -> Result<TestimonialsQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Testimonials.to_string())
			.with_id(&id)
			.with_condition("is_deleted = false")
			.with_select_fields(vec!["*", "user.* as user"]);
		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query");
		let result: Option<TestimonialsQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_testimonial_by_id' took: {elapsed:.2?}");
		}

		match result {
			Some(testimonial) => {
				if testimonial.is_deleted {
					bail!("Testimonial not found");
				}
				Ok(testimonial)
			}
			None => bail!("Testimonial not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<TestimonialsSchema> { // Change return type from String to TestimonialsSchema
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		info!(
			resource = %ResourceEnum::Testimonials.to_string(),
			payload = ?data,
			"Executing SurrealDB create"
		);
		let record: Option<TestimonialsSchema> = db
			.create(ResourceEnum::Testimonials.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_testimonial' took: {elapsed:.2?}");
		}

		match record {
			Some(created_testimonial) => Ok(created_testimonial), // Return the created testimonial
			None => bail!("Failed to create testimonial"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;

		let existing = self.query_testimonial_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Testimonial already deleted");
		}

		let merged = TestimonialsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			user: existing.user.id,
			..data
		};

		let record_key = get_id(&merged.id)?;
		info!(
			record_key = ?record_key,
			payload = ?merged,
			"Executing SurrealDB update"
		);
		let record: Option<TestimonialsSchema> =
			db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_testimonial' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success update testimonial".into()),
			None => bail!("Failed to update testimonial"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_testimonial(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let testimonial = self.query_testimonial_by_id(id).await?;
		if testimonial.is_deleted {
			bail!("Testimonial not found");
		}

		let record_key = get_id(&testimonial.id)?;
		info!(
			record_key = ?record_key,
			"Executing SurrealDB soft delete"
		);
		let record: Option<TestimonialsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_testimonial' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success delete testimonial".into()),
			None => bail!("Failed to delete testimonial"),
		}
	}
}
