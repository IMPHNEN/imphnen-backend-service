use super::GachaItemSchema;
use crate::{
	AppState, GachaItemDto, MetaRequestDto, ResourceEnum, ResponseListSuccessDto,
	get_id, make_thing,
};
use anyhow::{Result, bail};
use imphnen_iam::QueryListBuilder;
use imphnen_utils::get_iso_date;
use serde_json::{Map, Value};
use std::time::Instant;
use tracing::instrument;
use tracing::info;

pub struct GachaItemRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaItemRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, meta), err)]
	pub async fn query_gacha_item_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<GachaItemDto>>> {
		let now = Instant::now();
		let surreal_query = format!(
			"SELECT * FROM {} WHERE is_deleted = false AND name LIKE ?",
			ResourceEnum::GachaItems
		);
		info!(query = %surreal_query, "Executing SurrealDB query");
		let raw_result: ResponseListSuccessDto<Vec<GachaItemSchema>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::GachaItems.to_string(),
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
			println!("Query 'query_gacha_item_list' took: {elapsed:.2?}");
		}
		let data = raw_result
			.data
			.into_iter()
			.map(GachaItemDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: raw_result.meta,
		})
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_gacha_item_by_id(&self, id: String) -> Result<GachaItemSchema> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let surreal_query = format!("SELECT * FROM {} WHERE id = '{}'", ResourceEnum::GachaItems, id);
		info!(query = %surreal_query, "Executing SurrealDB query");
		let result: Option<GachaItemSchema> = db
			.select((ResourceEnum::GachaItems.to_string(), id.clone()))
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_item_by_id' took: {elapsed:.2?}");
		}
		match result {
			Some(item) if !item.is_deleted => Ok(item),
			_ => bail!("Gacha Item not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let surreal_query = format!("CREATE {} CONTENT ...", ResourceEnum::GachaItems);
		info!(query = %surreal_query, "Executing SurrealDB query");
		let record: Option<GachaItemSchema> = db
			.create(ResourceEnum::GachaItems.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_gacha_item' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success create Gacha Item".into()),
			None => bail!("Failed to create Gacha Item"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_update_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_gacha_item_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Gacha Item already deleted");
		}
		let merged = GachaItemSchema {
			created_at: existing.created_at,
			..data.clone()
		};
		let surreal_query = format!("UPDATE {:?} MERGE ...", record_key);
		info!(query = %surreal_query, "Executing SurrealDB query");
		let record: Option<GachaItemSchema> =
			db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_gacha_item' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success update Gacha Item".into()),
			None => bail!("Failed to update Gacha Item"),
		}
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_delete_gacha_item(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let item_id = make_thing(&ResourceEnum::GachaItems.to_string(), &id);
		let item = self.query_gacha_item_by_id(item_id.id.to_raw()).await?;
		if item.is_deleted {
			bail!("Gacha Item already deleted");
		}
		let record_key = get_id(&item.id)?;
		let mut patch = Map::new();
		patch.insert("is_deleted".to_string(), Value::Bool(true));
		patch.insert("updated_at".to_string(), Value::String(get_iso_date()));

		let surreal_query = format!("UPDATE {:?} MERGE ...", record_key);
		info!(query = %surreal_query, "Executing SurrealDB query");
		let record: Option<GachaItemSchema> = db.update(record_key).merge(patch).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_gacha_item' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success soft delete Gacha Item".into()),
			None => bail!("Failed to soft delete Gacha Item"),
		}
	}
}
