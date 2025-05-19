use super::{
	RolesDetailItemDto, RolesDetailQueryDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto, RolesSchema,
};
use crate::{
	AppState, MetaRequestDto, PermissionsItemDto, ResourceEnum,
	ResponseListSuccessDto, extract_id, get_id, make_thing, query_list_with_meta,
};
use anyhow::{Result, bail};
use imphnen_utils::DetailQueryBuilder;
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
		let mut conditions = vec!["is_deleted = false".into()];
		if let Some(_search) = meta.search.as_deref().filter(|s| !s.is_empty()) {
			conditions.push("string::contains(name ?? '', $search)".into());
		}
		if let (Some(filter_by), Some(filter_val)) =
			(meta.filter_by.as_ref(), meta.filter.as_ref())
		{
			if !filter_val.is_empty() {
				conditions.push(format!("{} = $filter", filter_by));
			}
		}
		let raw_result: ResponseListSuccessDto<Vec<RolesSchema>> = query_list_with_meta(
			&self.state.surrealdb_ws,
			&ResourceEnum::Roles.to_string(),
			&meta,
			conditions,
			None,
			"name",
			None,
			None,
		)
		.await?;
		let data = raw_result
			.data
			.into_iter()
			.map(|role| RolesSchema::list(&role))
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: raw_result.meta,
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
			.with_select_fields(vec![
				"id",
				"name",
				"permissions",
				"created_at",
				"updated_at",
				"is_deleted",
			])
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
		let permissions = role
			.permissions
			.into_iter()
			.map(|perm| PermissionsItemDto {
				id: extract_id(&perm.id),
				name: perm.name,
				created_at: perm.created_at,
				updated_at: perm.updated_at,
			})
			.collect();
		Ok(RolesDetailItemDto {
			id: extract_id(&role.id),
			name: role.name,
			is_deleted: role.is_deleted,
			permissions,
			created_at: role.created_at,
			updated_at: role.updated_at,
		})
	}

	pub async fn query_role_by_id(&self, id: String) -> Result<RolesDetailItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Roles.to_string())
			.with_id(&id)
			.with_select_fields(vec![
				"id",
				"name",
				"is_deleted",
				"permissions",
				"created_at",
				"updated_at",
			])
			.with_fetch("permissions");
		let sql = builder.build();
		let result: Option<RolesDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let role = match result {
			Some(r) if !r.is_deleted => r,
			_ => bail!("Role not found"),
		};
		let permissions = role
			.permissions
			.into_iter()
			.map(|perm| PermissionsItemDto {
				id: extract_id(&perm.id),
				name: perm.name,
				created_at: perm.created_at,
				updated_at: perm.updated_at,
			})
			.collect();
		Ok(RolesDetailItemDto {
			id: extract_id(&role.id),
			name: role.name,
			is_deleted: role.is_deleted,
			permissions,
			created_at: role.created_at,
			updated_at: role.updated_at,
		})
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
