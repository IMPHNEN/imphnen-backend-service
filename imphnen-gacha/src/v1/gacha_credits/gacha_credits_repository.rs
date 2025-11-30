use crate::v1::gacha_credits::gacha_credits_dto::GachaCreditRequestDto;
use crate::AppState;
use imphnen_libs::AppStatePostgresExt;
use imphnen_entities::seaorm::gacha::gacha_credits::{self, Entity as GachaCreditsEntity, Column as GachaCreditsColumn};
use anyhow::{Result, bail};
use sea_orm::{QueryFilter, ActiveValue, EntityTrait, ColumnTrait};
use std::time::Instant;
use tracing::instrument;
use uuid::Uuid;

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
		user_id: Uuid,
	) -> Result<Option<gacha_credits::Model>> {
		let now = Instant::now();
		let db = self.state.postgres_db();

		let result = GachaCreditsEntity::find()
			.filter(GachaCreditsColumn::UserId.eq(user_id))
			.filter(GachaCreditsColumn::IsDeleted.eq(false))
			.one(db)
			.await?;

		let elapsed = now.elapsed();
		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_by_user_id' took: {elapsed:.2?}");
		}
		Ok(result)
	}

	#[instrument(skip(self, user_id), err)]
	pub async fn query_consume_credit(&self, user_id: Uuid) -> Result<()> {
		let now = Instant::now();
		let db = self.state.postgres_db();
		let credit_opt = self.query_by_user_id(user_id).await?;
		let Some(credit) = credit_opt else {
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

		let mut active_model: gacha_credits::ActiveModel = credit.clone().into();
		active_model.available_rolls = ActiveValue::Set(credit.available_rolls - 1);

		GachaCreditsEntity::update(active_model).exec(db).await?;

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
		let db = self.state.postgres_db();
		if let Some(credit) = self.query_by_user_id(Uuid::parse_str(&payload.user_id)?).await? {
			let mut active_model: gacha_credits::ActiveModel = credit.clone().into();
			active_model.available_rolls = ActiveValue::Set(credit.available_rolls + payload.amount);

			GachaCreditsEntity::update(active_model).exec(db).await?;
		} else {
			let active_model = gacha_credits::ActiveModel {
				id: ActiveValue::Set(uuid::Uuid::new_v4()),
				user_id: ActiveValue::Set(Uuid::parse_str(&payload.user_id)?),
				available_rolls: ActiveValue::Set(payload.amount),
				is_deleted: ActiveValue::Set(false),
				created_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
				updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
			};

			GachaCreditsEntity::insert(active_model).exec(db).await?;
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
