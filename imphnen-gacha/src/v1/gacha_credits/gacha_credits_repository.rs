use super::{GachaCreditRequestDto, GachaCreditSchema};
use crate::{AppState, ResourceEnum};
use anyhow::{Result, bail};
use imphnen_iam::make_thing;
use surrealdb::Uuid;

pub struct GachaCreditRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaCreditRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_by_user_id(
		&self,
		user_id: String,
	) -> Result<Option<GachaCreditSchema>> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT * FROM {} WHERE user = {}:⟨$user_id⟩ AND is_deleted = false LIMIT 1",
			ResourceEnum::GachaCredits.to_string(),
			ResourceEnum::Users.to_string()
		);
		let result: Vec<GachaCreditSchema> =
			db.query(sql).bind(("user_id", user_id)).await?.take(0)?;
		Ok(result.into_iter().next())
	}

	pub async fn query_consume_credit(&self, user_id: String) -> Result<()> {
		let db = &self.state.surrealdb_ws;
		let credit_opt = self.query_by_user_id(user_id).await?;
		let Some(mut credit) = credit_opt else {
			return Ok(());
		};
		if credit.available_rolls <= 0 {
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
		Ok(())
	}

	pub async fn query_add_credit(
		&self,
		payload: GachaCreditRequestDto,
	) -> Result<()> {
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
			let data = GachaCreditSchema::from(&GachaCreditSchema {
				id: make_thing(
					&ResourceEnum::GachaCredits.to_string(),
					&Uuid::new_v4().to_string(),
				),
				user: make_thing(&ResourceEnum::Users.to_string(), &payload.user_id),
				available_rolls: payload.amount,
				..Default::default()
			});
			let _: Option<GachaCreditSchema> = db
				.create(&ResourceEnum::GachaCredits.to_string())
				.content(data)
				.await?;
		}
		Ok(())
	}
}
