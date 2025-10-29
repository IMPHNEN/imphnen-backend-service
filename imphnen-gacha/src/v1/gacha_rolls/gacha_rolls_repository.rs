use crate::v1::gacha_rolls::gacha_rolls_dto::GachaRollQueryDto;
use crate::v1::gacha_rolls::gacha_rolls_schema::GachaRollSchema;
use crate::AppState;
use imphnen_libs::ResourceEnum;
use imphnen_utils::DetailQueryBuilder;
use crate::{get_id, make_thing};
use anyhow::{Result, bail};

use rand::prelude::*;

use imphnen_utils::get_iso_date;
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
		
		// Use DetailQueryBuilder to properly fetch related item data
		let builder = DetailQueryBuilder::new(table_name)
			.with_condition("is_deleted = false AND quantity > 0")
			.with_select_fields(vec!["*"])
			.with_fetch("item");
		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query for active rolls");
		
		let mut result = builder.apply_bindings(db.query(sql)).await?;
		let results = match result.take(0) {
			Ok(v) => v,
			Err(_) => return Ok(Vec::new()),
		};

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_all_active_rolls' took: {elapsed:.2?}");
		}
		Ok(results)
	}

	#[instrument]
	pub fn roll_once(rolls: &[GachaRollQueryDto]) -> Option<GachaRollQueryDto> {
		let filtered: Vec<GachaRollQueryDto> = rolls
			.iter()
			.filter(|r| !r.is_deleted && r.quantity > 0)
			.cloned()
			.collect();
		
		if filtered.is_empty() {
			return None;
		}

		// Simple random selection based on quantity weights
		let total_weight: f32 = filtered.iter()
			.map(|r| r.weight * r.quantity as f32)
			.sum();

		if total_weight <= 0.0 {
			// Fallback to equal probability if weights are invalid
			let mut rng = rand::rngs::ThreadRng::default();
			let index = rng.random_range(0..filtered.len());
			return Some(filtered[index].clone());
		}

		// Weighted random selection
		let mut rng = rand::rngs::ThreadRng::default();
		let random_value = rng.random_range(0.0..total_weight);
		
		let mut cumulative_weight = 0.0;
		for roll in &filtered {
			cumulative_weight += roll.weight * roll.quantity as f32;
			if random_value <= cumulative_weight {
				return Some(roll.clone());
			}
		}

		// This should rarely happen but provides a fallback
		Some(filtered[0].clone())
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
