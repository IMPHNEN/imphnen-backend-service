use super::{
	testimonials_dto::TestimonialsQueryDto, testimonials_schema::TestimonialsSchema,
};
use anyhow::{Result, bail};
use imphnen_libs::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use imphnen_utils::{DetailQueryBuilder, ListQueryBuilder, get_id, get_iso_date};

pub struct TestimonialsRepository<'a> {
	state: &'a AppState,
}

impl<'a> TestimonialsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_testimonial_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TestimonialsQueryDto>>> {
		let query = ListQueryBuilder::new(&ResourceEnum::Testimonials.to_string())
			.with_select_fields(vec!["*", "user.* as user"]) // Select user details
			.with_pagination(meta.page, Some(10))
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.build();
		let res: Vec<TestimonialsQueryDto> =
			self.state.surrealdb_ws.query(query).await?.take(0)?;
		let data = ResponseListSuccessDto {
			data: res,
			meta: None,
		};
		Ok(data)
	}

	pub async fn query_testimonial_by_id(
		&self,
		id: String,
	) -> Result<TestimonialsQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Testimonials.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*", "user.* as user"]); // Select user details
		let sql = builder.build();
		let result: Option<TestimonialsQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;

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

	pub async fn query_create_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<TestimonialsSchema> = db
			.create(ResourceEnum::Testimonials.to_string())
			.content(data)
			.await?;

		match record {
			Some(_) => Ok("Success create testimonial".into()),
			None => bail!("Failed to create testimonial"),
		}
	}

	pub async fn query_update_testimonial(
		&self,
		data: TestimonialsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;

		let existing = self.query_testimonial_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Testimonial already deleted");
		}

		let merged = TestimonialsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			user: existing.user.id, // Preserve user ID
			..data
		};

		let record_key = get_id(&merged.id)?;
		let record: Option<TestimonialsSchema> =
			db.update(record_key).merge(merged).await?;

		match record {
			Some(_) => Ok("Success update testimonial".into()),
			None => bail!("Failed to update testimonial"),
		}
	}

	pub async fn query_delete_testimonial(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let testimonial = self.query_testimonial_by_id(id).await?;
		if testimonial.is_deleted {
			bail!("Testimonial not found");
		}

		let record_key = get_id(&testimonial.id)?;
		let record: Option<TestimonialsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;

		match record {
			Some(_) => Ok("Success delete testimonial".into()),
			None => bail!("Failed to delete testimonial"),
		}
	}
}
