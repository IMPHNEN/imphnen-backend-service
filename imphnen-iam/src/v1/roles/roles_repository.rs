use super::{
	RolesDetailItemDto, RolesDetailQueryDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto, RolesSchema,
};
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id, make_thing,
};
use anyhow::{Result, bail};
use imphnen_utils::{DetailQueryBuilder, QueryListBuilder};
use serde_json;
use std::time::Instant;
use surrealdb::Uuid;
use surrealdb::sql::Thing;
use tracing::instrument;

pub struct RolesRepository<'a> {
	state: &'a AppState,
}

impl<'a> RolesRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_role_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<RolesListItemDto>>> {
		let now = Instant::now();
		let result: ResponseListSuccessDto<Vec<RolesSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Roles.to_string(),
			&meta,
		)
		.search_field("name")
		.build()
		.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_list' took: {elapsed:.2?}");
		}
		let data = result
			.data
			.into_iter()
			.map(|role| RolesSchema::list(&role))
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	#[instrument(skip(self, name), err)]
	pub async fn query_role_by_name(
		&self,
		name: String,
	) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Roles.to_string())
			.with_where("name", Some(name.clone()))
			.with_select_fields(vec!["*"])
			.with_fetch("permissions");
		let sql = builder.build();
		let mut response = builder.apply_bindings(db.query(sql)).await?;

		let result_vec: Vec<RolesDetailQueryDto> = response.take(0).map_err(|e| {
			anyhow::anyhow!("Failed to take result from response: {:?}", e)
		})?;

		let role = result_vec
			.into_iter()
			.next()
			.ok_or_else(|| anyhow::anyhow!("Role not found"))?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_by_name' took: {elapsed:.2?}");
		}
		Ok(RolesDetailItemDto::from(&role))
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_role_by_id(&self, id: String) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let thing_id = make_thing(&ResourceEnum::Roles.to_string(), &id);
		let builder = DetailQueryBuilder::new(ResourceEnum::Roles.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"])
			.with_fetch("permissions");

		let sql = builder.build();
		let sql_debug = sql.to_string(); // Move this line here
		let result: Option<RolesDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_role_by_id' took: {elapsed:.2?}");
		}
		let role = match result {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Role not found sql: {} id: {}", sql_debug, thing_id),
		};
		Ok(RolesDetailItemDto::from(&role))
	}

	#[instrument(skip(self, payload), err)]
	pub async fn query_create_role(
		&self,
		payload: RolesRequestCreateDto,
	) -> Result<RolesDetailItemDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let role_id = Uuid::new_v4().to_string();
		let permission_things: Vec<Thing> = payload
			.permissions
			.iter()
			.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), id))
			.collect();
		let role = RolesSchema {
			id: make_thing(&ResourceEnum::Roles.to_string(), &role_id),
			name: payload.name,
			is_deleted: false,
			permissions: permission_things,
			created_at: Some(crate::get_iso_date()),
			updated_at: Some(crate::get_iso_date()),
		};
		let _: Option<RolesSchema> = db
			.create((&ResourceEnum::Roles.to_string(), role_id.clone()))
			.content(role)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_role' took: {elapsed:.2?}");
		}
		// After successful creation, fetch the created role
		self.query_role_by_id(role_id).await
	}

	#[instrument(skip(self, id, data), err)]
	pub async fn query_update_role(
		&self,
		id: String,
		data: RolesRequestUpdateDto,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let existing = self.query_role_by_id(id.clone()).await?;
		if existing.is_deleted {
			bail!("Role already deleted");
		}
		let merged = RolesSchema::update(data, id.clone(), existing);
		let record: Option<RolesSchema> =
			db.update(get_id(&merged.id)?).content(merged).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_role' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success update role".into()),
			None => bail!("Failed to update role"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_role(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let role_id = make_thing(&ResourceEnum::Roles.to_string(), &id);
		let role = self.query_role_by_id(role_id.id.to_raw()).await?;
		if role.is_deleted {
			bail!("Role already deleted");
		}
		let record_key = get_id(&role_id)?;
		let record: Option<RolesSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_role' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success delete role".into()),
			None => bail!("Failed to delete role"),
		}
	}
}
