use crate::v1::gacha_rolls::gacha_rolls_dto::GachaRollQueryDto;
use crate::v1::gacha_rolls::gacha_rolls_schema::GachaRollSchema;
use crate::AppState;
use imphnen_entities::seaorm::gacha::gacha_rolls::{Entity as GachaRollsEntity, ActiveModel as GachaRollActiveModel, Column as GachaRollColumn};
use anyhow::{Result, bail};
use chrono::Utc;
use rand::prelude::*;
use sea_orm::{EntityTrait, QueryFilter, Set, ColumnTrait, ActiveModelTrait};
use imphnen_libs::postgres::AppStatePostgresExt;
use std::time::Instant;
use tracing::instrument;
use uuid::Uuid;

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
		id: Uuid,
	) -> Result<GachaRollQueryDto> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		
		let result = GachaRollsEntity::find()
			.filter(GachaRollColumn::Id.eq(id))
			.filter(GachaRollColumn::IsDeleted.eq(false))
			.one(db)
			.await?;
			
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_roll_by_id' took: {elapsed:.2?}");
		}
		
		match result {
			Some(r) => Ok(GachaRollQueryDto {
				id: r.id.to_string(),
				item: None,
				weight: r.weight,
				quantity: r.quantity,
				is_deleted: r.is_deleted,
				created_at: r.created_at.map(|d| d.to_string()),
				updated_at: r.updated_at.map(|d| d.to_string()),
			}),
			None => bail!("Gacha Roll not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_roll(
		&self,
		data: GachaRollSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		
		let active_model = GachaRollActiveModel {
			id: Set(Uuid::parse_str(&data.id)?),
			user_id: Set(Uuid::parse_str(&data.user_id)?),
			gacha_id: Set(data.gacha_id), // gacha_id is String
			item_id: Set(Uuid::parse_str(&data.item_id)?),
			quantity: Set(data.quantity),
			weight: Set(data.weight),
			is_deleted: Set(false),
			created_at: Set(Some(Utc::now().naive_utc())),
			updated_at: Set(Some(Utc::now().naive_utc())),
		};
		
		let _ = GachaRollsEntity::insert(active_model)
			.exec(db)
			.await?;
			
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_gacha_roll' took: {elapsed:.2?}");
		}
		
		Ok("Success create Gacha Roll".into())
	}

	#[instrument(skip(self), err)]
	pub async fn query_all_active_rolls(&self) -> Result<Vec<GachaRollQueryDto>> {
		let now = Instant::now();
		let db = self.state.postgres_db();		
		let results = GachaRollsEntity::find()
			.filter(GachaRollColumn::IsDeleted.eq(false))
			.filter(GachaRollColumn::Quantity.gt(0))
			.all(db)
			.await?;
			
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_all_active_rolls' took: {elapsed:.2?}");
		}
		
		Ok(results.into_iter().map(|r| GachaRollQueryDto {
			id: r.id.to_string(),
			item: None,
			weight: r.weight,
			quantity: r.quantity,
			is_deleted: r.is_deleted,
			created_at: r.created_at.map(|d| d.to_string()),
			updated_at: r.updated_at.map(|d| d.to_string()),
		}).collect())
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
		let total_weight: f64 = filtered.iter()
			.map(|r| f64::from(r.weight) * f64::from(r.quantity))
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
			cumulative_weight += f64::from(roll.weight) * f64::from(roll.quantity);
			if random_value <= cumulative_weight {
				return Some(roll.clone());
			}
		}

		// This should rarely happen but provides a fallback
		Some(filtered[0].clone())
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_soft_delete_gacha_roll(&self, id: Uuid) -> Result<String> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		
		let roll = self.query_gacha_roll_by_id(id).await?;
		if roll.is_deleted {
			bail!("Gacha Roll already deleted");
		}
		
		let mut active_model: GachaRollActiveModel = GachaRollsEntity::find_by_id(id)
			.one(db)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Gacha Roll not found"))?
			.into();
			
		active_model.is_deleted = Set(true);
		active_model.updated_at = Set(Some(Utc::now().naive_utc()));
		
		let _result = GachaRollActiveModel::update(active_model, db).await?;
			
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_soft_delete_gacha_roll' took: {elapsed:.2?}");
		}
		
		Ok("Success soft delete Gacha Roll".into())
	}
}
