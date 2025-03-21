use super::{
	GachaClaimResponseDto, GachaClaimSchema, GachaCreateClaimRequestDto,
	GachaCreateItemRequestDto, GachaCreateRollRequestDto, GachaItemResponseDto,
	GachaItemSchema, GachaRollSchema,
};
use crate::{v1::AuthRepository, AppState, ResourceEnum};
use anyhow::{bail, Result};
use surrealdb::sql::{Id, Thing};

pub struct GachaRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> GachaRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_claim_by_transaction_number(
		&self,
		transaction_number: String,
	) -> Result<GachaClaimResponseDto> {
		let db = &self.state.surrealdb;

		let result = db
			.select((ResourceEnum::GachaClaims.to_string(), transaction_number))
			.await?;

		match result {
			Some(response) => Ok(response),
			None => bail!("Gacha claim not found"),
		}
	}

	pub async fn query_gacha_item_by_name(
		&self,
		name: String,
	) -> Result<GachaItemResponseDto> {
		let db = &self.state.surrealdb;

		let result = db
			.select((ResourceEnum::GachaItems.to_string(), name))
			.await?;

		match result {
			Some(response) => Ok(response),
			None => bail!("Gacha item not found"),
		}
	}

	pub async fn query_gacha_claim_by_name(
		&self,
		name: String,
	) -> Result<GachaItemResponseDto> {
		let db = &self.state.surrealdb;

		let result = db
			.select((ResourceEnum::GachaClaims.to_string(), name))
			.await?;

		match result {
			Some(response) => Ok(response),
			None => bail!("Gacha item not found"),
		}
	}

	pub async fn query_create_gacha_claim(
		&self,
		data: GachaCreateClaimRequestDto,
		email: String,
	) -> Result<String> {
		let auth_repository = AuthRepository::new(self.state);
		let db = &self.state.surrealdb;

		let user = auth_repository.query_user_by_email(email).await?;

		let user_thing =
			Thing::from((ResourceEnum::Users.to_string(), Id::String(user.email)));

		let item_thing = Thing::from((
			ResourceEnum::GachaItems.to_string(),
			Id::String(data.item_name.clone()),
		));

		let record: Option<GachaClaimSchema> = db
			.create((
				ResourceEnum::GachaClaims.to_string(),
				&data.transaction_number,
			))
			.content(GachaClaimSchema {
				transaction_number: data.transaction_number.clone(),
				item: item_thing,
				user: user_thing,
			})
			.await?;

		match record {
			Some(_) => Ok("Gacha claims successfully created".to_string()),
			None => bail!("Failed to create gacha claims"),
		}
	}

	pub async fn query_create_gacha_item(
		&self,
		data: GachaCreateItemRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb;

		let record: Option<GachaItemSchema> = db
			.create((ResourceEnum::GachaItems.to_string(), data.item_name.clone()))
			.content(GachaItemSchema {
				item_name: data.item_name.clone(),
				item_image: data.item_image.clone(),
			})
			.await?;

		match record {
			Some(_) => Ok("Gacha item successfully created".to_string()),
			None => bail!("Failed to create gacha item"),
		}
	}

	pub async fn query_create_gacha_roll(
		&self,
		data: GachaCreateRollRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb;
		let item_thing = Thing::from((
			ResourceEnum::GachaItems.to_string(),
			Id::String(data.item_name.clone()),
		));

		let record: Option<GachaRollSchema> = db
			.create((ResourceEnum::GachaRolls.to_string(), data.item_name.clone()))
			.content(GachaRollSchema {
				weight: data.weight.clone(),
				item: item_thing,
			})
			.await?;

		match record {
			Some(_) => Ok("Gacha roll successfully created".to_string()),
			None => bail!("Failed to create gacha roll"),
		}
	}
}
