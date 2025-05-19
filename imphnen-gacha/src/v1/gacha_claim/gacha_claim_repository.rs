use super::GachaClaimSchema;
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};

pub struct GachaClaimRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaClaimRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_claim_by_id(
		&self,
		id: String,
	) -> Result<GachaClaimSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<GachaClaimSchema> = db
			.select((ResourceEnum::GachaClaims.to_string(), id.clone()))
			.await?;
		match result {
			Some(claim) if !claim.is_deleted => Ok(claim),
			_ => bail!("Gacha Claim not found"),
		}
	}

	pub async fn query_create_gacha_claim(
		&self,
		data: GachaClaimSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<GachaClaimSchema> = db
			.create(ResourceEnum::GachaClaims.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create Gacha Claim".into()),
			None => bail!("Failed to create Gacha Claim"),
		}
	}
}
