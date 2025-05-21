use super::GachaRollQueryDto;
use super::GachaRollSchema;
use crate::{AppState, DetailQueryBuilder, ResourceEnum};
use anyhow::{Result, bail};
use imphnen_iam::ListQueryBuilder;
use rand::prelude::*;
use rand::rng;
use rand_distr::weighted::WeightedIndex;

pub struct GachaRollRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaRollRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_roll_by_id(
		&self,
		id: String,
	) -> Result<GachaRollQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::GachaRolls.to_string())
			.with_id(id.clone())
			.with_select_fields(vec!["*"])
			.with_fetch("item");
		let sql = builder.build();
		let result: Option<GachaRollQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		match result {
			Some(roll) if !roll.is_deleted => Ok(roll),
			_ => bail!("Gacha Roll not found"),
		}
	}

	pub async fn query_create_gacha_roll(
		&self,
		data: GachaRollSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<GachaRollSchema> = db
			.create(ResourceEnum::GachaRolls.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create Gacha Roll".into()),
			None => bail!("Failed to create Gacha Roll"),
		}
	}

	pub async fn query_all_active_rolls(&self) -> Result<Vec<GachaRollQueryDto>> {
		let db = &self.state.surrealdb_ws;
		let builder = ListQueryBuilder::new(ResourceEnum::GachaRolls.to_string())
			.with_select_fields(vec!["*"])
			.with_fetch(Some(vec!["item"]));
		let sql = builder.build();
		let result: Vec<GachaRollQueryDto> = db.query(sql).await?.take(0)?;
		Ok(result)
	}

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
		let mut rng = rng();
		let index = dist.sample(&mut rng);
		Some(filtered[index].clone())
	}
}
