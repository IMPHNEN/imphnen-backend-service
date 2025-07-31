use super::{GachaClaimQueryDto, GachaClaimSchema};
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};
use imphnen_iam::DetailQueryBuilder;
use std::time::Instant;
use tracing::{instrument, info};

pub struct GachaClaimRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaClaimRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_gacha_claim_by_id(
		&self,
		id: String,
	) -> Result<GachaClaimQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::GachaClaims.to_string())
			.with_id(id.clone())
			.with_select_fields(vec!["*"])
			.with_fetch("item")
			.with_fetch("user");
		let sql = builder.build();
		info!(query = %sql, "Executing SurrealDB query");
		let result: Option<GachaClaimQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_gacha_claim_by_id' took: {elapsed:.2?}");
		}
		match result {
			Some(claim) if !claim.is_deleted => Ok(claim),
			_ => bail!("Gacha Claim not found"),
		}
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_claim(
		&self,
		data: GachaClaimSchema,
	) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		info!(
			resource = %ResourceEnum::GachaClaims.to_string(),
			content = ?data,
			"Executing SurrealDB create query"
		);
		let record: Option<GachaClaimSchema> = db
			.create(ResourceEnum::GachaClaims.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_gacha_claim' took: {elapsed:.2?}");
		}
		match record {
			Some(_) => Ok("Success create Gacha Claim".into()),
			None => bail!("Failed to create Gacha Claim"),
		}
	}
}
