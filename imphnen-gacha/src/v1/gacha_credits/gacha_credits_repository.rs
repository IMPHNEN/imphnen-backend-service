use super::{GachaCreditRequestDto, GachaCreditSchema};
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};
use imphnen_iam::make_thing;
use std::time::Instant;
use surrealdb::Uuid;
use tracing::instrument;

pub struct GachaCreditRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaCreditRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, user_id), err)]
	pub async fn query_by_user_id(
		&self,
		user_id: String,
	) -> Result<Option<GachaCreditSchema>> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE user = {}:⟨$user_id⟩ AND is_deleted = false LIMIT 1",
			ResourceEnum::GachaCredits,
			ResourceEnum::Users
		);
		let result: Vec<GachaCreditSchema> =
			db.query(sql).bind(("user_id", user_id)).await?.take(0)?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_by_user_id' took: {elapsed:.2?}");
		}
		Ok(result.into_iter().next())
	}

	#[instrument(skip(self, user_id), err)]
	pub async fn query_consume_credit(&self, user_id: String) -> Result<()> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let credit_opt = self.query_by_user_id(user_id).await?;
		let Some(mut credit) = credit_opt else {
			let elapsed = now.elapsed();
			if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
				== "development"
			{
				println!(
					"Query 'query_consume_credit' took: {elapsed:.2?} (no credit to consume)"
				);
			}
			return Ok(());
		};
		if credit.available_rolls <= 0 {
			let elapsed = now.elapsed();
			if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
				== "development"
			{
				println!(
					"Query 'query_consume_credit' took: {elapsed:.2?} (no rolls remaining)"
				);
			}
			bail!("No extra roll credits remaining");
		}
		credit.available_rolls -= 1;
		let _: Option<GachaCreditSchema> = db
			.update((
				&ResourceEnum::GachaCredits.to_string(),
				credit.id.id.to_raw(),
			))
			.merge(credit)
			.await?;
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_consume_credit' took: {elapsed:.2?}");
		}
		Ok(())
	}

	#[instrument(skip(self, payload), err)]
	pub async fn query_add_credit(
		&self,
		payload: GachaCreditRequestDto,
	) -> Result<()> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		if let Some(mut credit) = self.query_by_user_id(payload.user_id.clone()).await? {
			credit.available_rolls += payload.amount;
			let _: Option<GachaCreditSchema> = db
				.update((
					&ResourceEnum::GachaCredits.to_string(),
					credit.id.id.to_raw(),
				))
				.merge(credit)
				.await?;
		} else {
			let data = GachaCreditSchema {
				id: make_thing(
					&ResourceEnum::GachaCredits.to_string(),
					&Uuid::new_v4().to_string(),
				),
				user: make_thing(&ResourceEnum::Users.to_string(), &payload.user_id),
				available_rolls: payload.amount,
				..Default::default()
			};
			let _: Option<GachaCreditSchema> = db
				.create(ResourceEnum::GachaCredits.to_string())
				.content(data)
				.await?;
		}
		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_add_credit' took: {elapsed:.2?}");
		}
		Ok(())
	}
}
