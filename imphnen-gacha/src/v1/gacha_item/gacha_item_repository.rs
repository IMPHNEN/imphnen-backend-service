use super::GachaItemSchema;
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id,
	make_thing, query_list_with_meta,
};
use anyhow::{Result, bail};

pub struct GachaItemRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaItemRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_gacha_item_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<GachaItemSchema>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions.push("string::contains(name, $search)".into());
		}
		query_list_with_meta(
			&self.state.surrealdb_ws,
			&ResourceEnum::GachaItems.to_string(),
			&meta,
			conditions,
			None,
			"name",
			None,
		)
		.await
	}

	pub async fn query_gacha_item_by_id(&self, id: String) -> Result<GachaItemSchema> {
		let db = &self.state.surrealdb_ws;
		let result: Option<GachaItemSchema> = db
			.select((ResourceEnum::GachaItems.to_string(), id.clone()))
			.await?;
		match result {
			Some(item) if !item.is_deleted => Ok(item),
			_ => bail!("Gacha Item not found"),
		}
	}

	pub async fn query_create_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<GachaItemSchema> = db
			.create(ResourceEnum::GachaItems.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create Gacha Item".into()),
			None => bail!("Failed to create Gacha Item"),
		}
	}

	pub async fn query_update_gacha_item(
		&self,
		data: GachaItemSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_gacha_item_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Gacha Item already deleted");
		}
		let merged = GachaItemSchema {
			created_at: existing.created_at,
			..data.clone()
		};
		let record: Option<GachaItemSchema> =
			db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update Gacha Item".into()),
			None => bail!("Failed to update Gacha Item"),
		}
	}

	pub async fn query_delete_gacha_item(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let item_id = make_thing(&ResourceEnum::GachaItems.to_string(), &id);
		let item = self.query_gacha_item_by_id(item_id.id.to_raw()).await?;
		if item.is_deleted {
			bail!("Gacha Item already deleted");
		}
		let record_key = get_id(&item.id)?;
		let record: Option<GachaItemSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete Gacha Item".into()),
			None => bail!("Failed to delete Gacha Item"),
		}
	}
}
