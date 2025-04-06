use super::GachaRollSchema;
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};

pub struct GachaRollRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaRollRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_roll_by_id(&self, id: String) -> Result<GachaRollSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<GachaRollSchema> = db
			.select((ResourceEnum::GachaRolls.to_string(), id.clone()))
			.await?;
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
}
