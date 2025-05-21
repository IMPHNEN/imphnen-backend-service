use super::{GachaClaimQueryDto, GachaClaimSchema};
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};
use imphnen_iam::DetailQueryBuilder;

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
	) -> Result<GachaClaimQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::GachaClaims.to_string())
			.with_id(id.clone())
			.with_select_fields(vec!["*"])
			.with_fetch("item")
			.with_fetch("user");
		let sql = builder.build();
		let result: Option<GachaClaimQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
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
