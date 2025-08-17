use anyhow::{Result, bail};
use imphnen_iam::{get_id, make_thing};
use surrealdb::sql::Thing;

use crate::v1::mentors::{MentorDetailWithUserDto, MentorInsertDto, MentorSchema};
use imphnen_libs::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use imphnen_utils::{DetailQueryBuilder, QueryListBuilder, get_iso_date};
use serde_json::{Map, Value};
use std::time::Instant;
use tracing::instrument;
use tracing::info;

pub struct MentorsRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> MentorsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_mentor_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<MentorDetailWithUserDto>>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let mentors_table = ResourceEnum::Mentors.to_string();
		let builder = QueryListBuilder::new(db, &mentors_table, &meta)
			.search_field("user_id.legal_name") // Search in user data instead
			.select_fields(vec![
				"id",
				"user_id",
				// Personal data comes from user relation, not mentor table
				"industries",
				"expertise",
				"languages",
				"current_company",
				"current_role",
				"years_of_experience",
				"topics_of_interest",
				"preferred_mentee_level",
				"preferred_mentoring_formats",
				"availability_commitment",
				"mentoring_rate",
				"status",
				"is_deleted",
				"created_at",
				"updated_at",
			]);
		let result = builder.build().await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_mentor_list' took: {elapsed:.2?}");
		}
		let data = result.data.into_iter().collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	#[instrument(skip(self, email, include_deleted), err)]
	pub async fn query_mentor_by_email(
		&self,
		email: String,
		include_deleted: bool,
	) -> Result<MentorDetailWithUserDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let mut builder = DetailQueryBuilder::new(ResourceEnum::Mentors.to_string())
			.with_where("user_id.email", Some(email.clone())) // Search in user table
			.with_select_fields(vec![
				"id",
				"user_id",
				// Personal data comes from user relation, not mentor table
				"industries",
				"expertise",
				"languages",
				"current_company",
				"current_role",
				"years_of_experience",
				"topics_of_interest",
				"preferred_mentee_level",
				"preferred_mentoring_formats",
				"availability_commitment",
				"mentoring_rate",
				"status",
				"is_deleted",
				"created_at",
				"updated_at",
			]);

		if !include_deleted {
			builder = builder.with_condition("is_deleted = false");
		}

		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query in query_mentor_by_email");
		let mentor_opt: Option<MentorDetailWithUserDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_mentor_by_email' took: {elapsed:.2?}");
		}
		let Some(mentor) = mentor_opt else {
			bail!("Mentor not found");
		};
		Ok(mentor)
	}

	#[instrument(skip(self, id, include_deleted), err)]
	pub async fn query_mentor_by_id(
		&self,
		id: &Thing,
		include_deleted: bool,
	) -> Result<MentorDetailWithUserDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let mut builder = DetailQueryBuilder::new(ResourceEnum::Mentors.to_string())
			.with_id(get_id(id)?.1)
			.with_select_fields(vec![
				"id",
				"user_id",
				// Personal data comes from user relation, not mentor table
				"industries",
				"expertise",
				"languages",
				"current_company",
				"current_role",
				"years_of_experience",
				"topics_of_interest",
				"preferred_mentee_level",
				"preferred_mentoring_formats",
				"availability_commitment",
				"mentoring_rate",
				"status",
				"is_deleted",
				"created_at",
				"updated_at",
			]);

		if !include_deleted {
			builder = builder.with_condition("is_deleted = false");
		}

		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query in query_mentor_by_id");
		let mentor_opt: Option<MentorDetailWithUserDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_mentor_by_id' took: {elapsed:.2?}");
		}
		let Some(mentor) = mentor_opt else {
			bail!("Mentor not found in database");
		};
		if mentor.is_deleted && !include_deleted {
			bail!("Mentor has been deleted");
		}
		Ok(mentor)
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_mentor(&self, data: MentorSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let dto: MentorInsertDto = data.into();
		let resource = ResourceEnum::Mentors.to_string();
		info!(query = %resource, "Executing SurrealDB create in query_create_mentor");
		let record: Option<MentorSchema> = db
			.create(resource)
			.content(dto.clone())
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_mentor' took: {elapsed:.2?}");
		}
		match record {
			Some(mentor) => {
				let id_str = mentor.id.id.to_raw();
				let _user = format!("{:?}", mentor.user_id);
				Ok(id_str)
			}
			None => {
				bail!("Failed to create mentor")
			}
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_mentor(&self, data: MentorSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let id_ref = &data.id;
		let record_key = get_id(id_ref)?;
		let _existing = self.query_mentor_by_id(id_ref, false).await?;

		let mut merged_data_json: Map<String, Value> =
			serde_json::to_value(data.clone())
				.map_err(|e| anyhow::anyhow!("Failed to serialize MentorSchema: {}", e))?
				.as_object()
				.cloned()
				.unwrap_or_default();

		merged_data_json.remove("id");
		merged_data_json.remove("user_id");
		merged_data_json.remove("created_at");

		merged_data_json.insert("updated_at".to_string(), Value::String(get_iso_date()));

		info!(query = ?record_key, "Executing SurrealDB update in query_update_mentor");
		let record: Option<MentorSchema> =
			db.update(record_key).merge(merged_data_json).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_mentor' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success update mentor".into()),
			None => {
				bail!("Failed to update mentor")
			}
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_mentor(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let thing = make_thing(ResourceEnum::Mentors.to_string().as_str(), &id);
		let record_key = get_id(&thing)?;

		let mentor_to_delete_res = self.query_mentor_by_id(&thing, true).await;

		let _mentor_to_delete = match mentor_to_delete_res {
			Ok(mentor) => {
				if mentor.is_deleted {
					bail!("Mentor is already soft deleted");
				}
				mentor
			}
			Err(e) => {
				if e.to_string().contains("Mentor has been deleted") {
					bail!("Mentor is already soft deleted");
				} else {
					return Err(e);
				}
			}
		};

		let mut patch = Map::new();
		patch.insert("is_deleted".to_string(), Value::Bool(true));
		patch.insert("updated_at".to_string(), Value::String(get_iso_date()));

		info!(query = ?record_key, "Executing SurrealDB soft delete in query_delete_mentor");
		let record: Option<MentorSchema> = db.update(record_key).merge(patch).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_mentor' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success soft delete mentor".into()),
			None => {
				bail!("Failed to soft delete mentor")
			}
		}
	}
}
