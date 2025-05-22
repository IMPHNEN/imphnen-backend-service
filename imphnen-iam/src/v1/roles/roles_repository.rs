use super::{
	RolesDetailItemDto, RolesDetailQueryDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto, RolesSchema,
};
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id, make_thing,
};
use anyhow::{Result, bail};
use imphnen_utils::{DetailQueryBuilder, QueryListBuilder};
use surrealdb::Uuid;
use surrealdb::sql::Thing;

pub struct RolesRepository<'a> {
	state: &'a AppState,
}

impl<'a> RolesRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_role_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<RolesListItemDto>>> {
		let result: ResponseListSuccessDto<Vec<RolesSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Roles.to_string(),
			&meta,
		)
		.search_field("name")
		.build()
		.await?;
		let data = result
			.data
			.into_iter()
			.map(|role| RolesSchema::list(&role))
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_role_by_name(
		&self,
		name: String,
	) -> Result<RolesDetailItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Roles.to_string())
			.with_where("name")
			.where_value(name.clone())
			.with_select_fields(vec!["*"])
			.with_fetch("permissions");
		let sql = builder.build();
		let result: Option<RolesDetailQueryDto> = builder
			.apply_bindings(db.query(sql).bind(("name", name)))
			.await?
			.take(0)?;
		let role = match result {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Role not found"),
		};
		Ok(RolesDetailItemDto::from(&role))
	}

	pub async fn query_role_by_id(&self, id: String) -> Result<RolesDetailItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Roles.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"])
			.with_fetch("permissions");
		let sql = builder.build();
		let result: Option<RolesDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let role = match result {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Role not found"),
		};
		Ok(RolesDetailItemDto::from(&role))
	}

	pub async fn query_create_role(
		&self,
		payload: RolesRequestCreateDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let role_id = Uuid::new_v4().to_string();
		let permission_things: Vec<Thing> = payload
			.permissions
			.iter()
			.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), id))
			.collect();
		let role = RolesSchema {
			id: make_thing(&ResourceEnum::Roles.to_string(), &role_id),
			name: payload.name,
			is_deleted: false,
			permissions: permission_things,
			created_at: Some(crate::get_iso_date()),
			updated_at: Some(crate::get_iso_date()),
		};
		let _: Option<RolesSchema> = db
			.create((&ResourceEnum::Roles.to_string(), role_id))
			.content(role)
			.await?;
		Ok("Role with permissions created successfully".into())
	}

	pub async fn query_update_role(
		&self,
		id: String,
		data: RolesRequestUpdateDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let existing = self.query_role_by_id(id.clone()).await?;
		if existing.is_deleted {
			bail!("Role already deleted");
		}
		let merged = RolesSchema::update(data, id.clone(), existing);
		let record: Option<RolesSchema> =
			db.update(get_id(&merged.id)?).content(merged).await?;
		match record {
			Some(_) => Ok("Success update role".into()),
			None => bail!("Failed to update role"),
		}
	}

	pub async fn query_delete_role(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let role_id = make_thing(&ResourceEnum::Roles.to_string(), &id);
		let role = self.query_role_by_id(role_id.id.to_raw()).await?;
		if role.is_deleted {
			bail!("Role already deleted");
		}
		let record_key = get_id(&role_id)?;
		let record: Option<RolesSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete role".into()),
			None => bail!("Failed to delete role"),
		}
	}
}
