use crate::v1::gacha_rolls::gacha_rolls_dto::GachaRollQueryDto;
use crate::v1::gacha_rolls::gacha_rolls_schema::GachaRollSchema;
use crate::AppState;
use imphnen_libs::ResourceEnum;
use imphnen_utils::DetailQueryBuilder;
use crate::{get_id, make_thing};
use anyhow::{Result, bail};

use rand::prelude::*;

use imphnen_utils::get_iso_date;
use rand_distr::weighted::WeightedIndex;
use serde_json::{Map, Value};
use std::time::Instant;
use tracing::instrument;
use tracing::info;

pub struct GachaRollRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaRollRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_gacha_roll_by_id(
		&self,
		id: String,
	) -> Result<GachaRollQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::GachaRolls.to_string())
			.with_id(id.clone())
			.with_condition("is_deleted = false")
			.with_select_fields(vec!["*"])
			.with_fetch("item");
		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query");
		let result: Option<GachaRollQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_roll_by_id' took: {elapsed:.2?}");
		}
		match result {
			Some(roll) if !roll.is_deleted => Ok(roll),
			_ => bail!("Gacha Roll not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_roll(
		&self,
		data: GachaRollSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		info!(query = "CREATE", "Executing SurrealDB create operation for GachaRolls");
		let record: Option<GachaRollSchema> = db
			.create(ResourceEnum::GachaRolls.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_gacha_roll' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success create Gacha Roll".into()),
			None => bail!("Failed to create Gacha Roll"),
		}
	}

	#[instrument(skip(self), err)]
	pub async fn query_all_active_rolls(&self) -> Result<Vec<GachaRollQueryDto>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let table_name = ResourceEnum::GachaRolls.to_string();
		let sql =
			format!("SELECT * FROM {table_name} WHERE is_deleted = false FETCH item");
		info!(query = %sql, "Executing SurrealDB query");
		let result: Vec<GachaRollQueryDto> = db.query(sql).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_all_active_rolls' took: {elapsed:.2?}");
		}
		Ok(result)
	}

	#[instrument]
	pub fn roll_once(rolls: &[GachaRollQueryDto]) -> Option<GachaRollQueryDto> {
		let filtered: Vec<_> = rolls
			.iter()
			.filter(|r| !r.is_deleted && r.quantity > 0)
			.collect();
		let weights: Vec<f32> = filtered
			.iter()
			.map(|r| r.weight * r.quantity as f32)
			.collect();
		if weights.iter().all(|&w| w <= 0.0) {
			return None;
		}
		let dist = WeightedIndex::new(&weights).ok()?;
		let mut rng = rand::rngs::ThreadRng::default();
		let index = dist.sample(&mut rng);
		Some(filtered[index].clone())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_soft_delete_gacha_roll(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let roll_id_thing = make_thing(&ResourceEnum::GachaRolls.to_string(), &id);
		let roll = self.query_gacha_roll_by_id(id.clone()).await?;
		if roll.is_deleted {
			bail!("Gacha Roll already deleted");
		}
		let record_key = get_id(&roll_id_thing)?;

		let mut patch = Map::new();
		patch.insert("is_deleted".to_string(), Value::Bool(true));
		patch.insert("updated_at".to_string(), Value::String(get_iso_date()));

		info!(query = "UPDATE", record_key = ?record_key, "Executing SurrealDB update operation for GachaRolls");
		let record: Option<GachaRollSchema> = db.update(record_key).merge(patch).await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_soft_delete_gacha_roll' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success soft delete Gacha Roll".into()),
			None => bail!("Failed to soft delete Gacha Roll"),
		}
	}
}
