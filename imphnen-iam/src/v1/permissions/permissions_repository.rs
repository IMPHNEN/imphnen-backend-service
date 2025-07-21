use super::{PermissionsItemDto, PermissionsSchema};
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id, make_thing,
};
use anyhow::{Result, bail};
use imphnen_utils::{DetailQueryBuilder, QueryListBuilder, extract_id};
use serde_json;
use std::time::Instant;
use tracing::instrument;

pub struct PermissionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> PermissionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_permission_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<PermissionsItemDto>>> {
		let now = Instant::now();
		let raw_result: ResponseListSuccessDto<Vec<PermissionsSchema>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::Permissions.to_string(),
				&meta,
			)
			.with_condition("is_deleted = false")
			.search_field("name")
			.select_fields(vec!["*"])
			.build()
			.await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_permission_list' took: {elapsed:.2?}");
		}

		let transformed_data = raw_result
			.data
			.into_iter()
			.map(|permission| PermissionsSchema::list(&permission))
			.collect();

		Ok(ResponseListSuccessDto {
			data: transformed_data,
			meta: raw_result.meta,
		})
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_permission_by_id(
		&self,
		id: String,
	) -> Result<PermissionsSchema> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let result: Option<PermissionsSchema> = db
			.select((ResourceEnum::Permissions.to_string(), id.clone()))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_permission_by_id' took: {elapsed:.2?}");
		}
		match result {
			Some(permission) if !permission.is_deleted => Ok(permission),
			_ => bail!("Permission not found"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn transformed_query_permission_by_id(
		&self,
		id: String,
	) -> Result<PermissionsItemDto> {
		let now = Instant::now();
		let raw_result = self.query_permission_by_id(id.clone()).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'transformed_query_permission_by_id' took: {elapsed:.2?}");
		}
		let transformed_data = PermissionsItemDto {
			id: extract_id(&raw_result.id),
			name: raw_result.name,
			created_at: raw_result.created_at,
			updated_at: raw_result.updated_at,
		};
		Ok(transformed_data)
	}

	#[instrument(skip(self, name), err)]
	pub async fn query_permission_by_name(
		&self,
		name: String,
	) -> Result<PermissionsSchema> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Permissions.to_string())
			.with_where("name", Some(name.clone()))
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		let result: Option<PermissionsSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_permission_by_name' took: {elapsed:.2?}");
		}
		match result {
			Some(permission) => Ok(permission),
			None => bail!("Permission not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record: Option<PermissionsSchema> = db
			.create(ResourceEnum::Permissions.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_permission' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success create permission".into()),
			None => bail!("Failed to create permission"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_permission(
		&self,
		data: PermissionsSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_permission_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Permission already deleted");
		}
		let merged = PermissionsSchema {
			created_at: existing.created_at,
			..data.clone()
		};
		let record: Option<PermissionsSchema> =
			db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_permission' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success update permission".into()),
			None => bail!("Failed to update permission"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_permission(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let permission_id = make_thing(&ResourceEnum::Permissions.to_string(), &id);
		let permission = self
			.query_permission_by_id(permission_id.id.to_raw())
			.await?;
		if permission.is_deleted {
			bail!("Permission already deleted");
		}
		let record_key = get_id(&permission.id)?;
		let record: Option<PermissionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_permission' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success delete permission".into()),
			None => bail!("Failed to delete permission"),
		}
	}
}
