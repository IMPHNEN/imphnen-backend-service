use super::{
	RolesItemByIdDto, RolesItemByIdDtoRaw, RolesRequestCreateDto,
	RolesRequestUpdateDto, RolesSchema,
};
use crate::{
	extract_id, get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	PermissionsItemDto, ResourceEnum, ResponseListSuccessDto,
};
use anyhow::{bail, Result};
use surrealdb::sql::Thing;
use surrealdb::Uuid;

pub struct RolesRepository<'a> {
	state: &'a AppState,
}

impl<'a> RolesRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_role_by_id(&self, id: &str) -> Result<RolesSchema> {
		let db = &self.state.surrealdb_ws;
		let role: Option<RolesSchema> =
			db.select((ResourceEnum::Roles.to_string(), id)).await?;
		match role {
			Some(r) if !r.is_deleted => Ok(r),
			_ => bail!("Role not found"),
		}
	}

	pub async fn query_role_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<RolesSchema>>> {
		let mut conditions = vec!["is_deleted = false".into()];
		if meta.search.is_some() {
			conditions.push("string::contains(name, $search)".into());
		}
		if meta.filter_by.is_some() && meta.filter.is_some() {
			let filter_by = meta.filter_by.as_ref().unwrap();
			conditions.push(format!("{} = $filter", filter_by));
		}
		query_list_with_meta(
			&self.state.surrealdb_ws,
			&ResourceEnum::Roles.to_string(),
			&meta,
			conditions,
			None,
		)
		.await
	}

	pub async fn query_role_by_name(&self, name: String) -> Result<RolesItemByIdDto> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT *, permissions FROM {} WHERE name = $name AND is_deleted = false LIMIT 1 FETCH permissions",
			ResourceEnum::Roles.to_string()
		);
		let mut result = db.query(sql).bind(("name", name.clone())).await?;
		let role: Option<RolesItemByIdDtoRaw> = result.take(0)?;
		let role = match role {
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
			.collect::<Vec<_>>();
		Ok(RolesItemByIdDto {
			id: extract_id(&role.id),
			name: role.name,
			is_deleted: role.is_deleted,
			permissions,
			created_at: role.created_at,
			updated_at: role.updated_at,
		})
	}

	pub async fn query_role_by_id(&self, id: String) -> Result<RolesItemByIdDto> {
		let db = &self.state.surrealdb_ws;
		let query = format!(
			"SELECT *, permissions.* AS permissions
			FROM app_roles:⟨{}⟩ WHERE is_deleted = false FETCH permissions",
			id
		);
		let mut result = db.query(query).await?;
		let role: Option<RolesItemByIdDtoRaw> = result.take(0)?;
		let role = match role {
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
			.collect::<Vec<_>>();
		Ok(RolesItemByIdDto {
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
		let thing_id = make_thing(&ResourceEnum::Roles.to_string(), &id);
		let existing = self.query_raw_role_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Role already deleted");
		}
		let permissions: Vec<Thing> = if let Some(permission_ids) = &data.permissions {
			permission_ids
				.iter()
				.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), id))
				.collect()
		} else {
			existing
				.permissions
				.iter()
				.map(|p| make_thing(&ResourceEnum::Permissions.to_string(), &p.id.to_raw()))
				.collect()
		};
		let merged = RolesSchema {
			id: thing_id,
			name: data.name.unwrap_or(existing.name),
			permissions,
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			updated_at: Some(crate::get_iso_date()),
		};
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
