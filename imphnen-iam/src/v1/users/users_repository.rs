use super::{UsersDetailQueryDto, UsersListItemDto, UsersListQueryDto, UsersSchema};
use crate::{
	AppState, MetaRequestDto, PermissionsQueryDto, ResourceEnum,
	ResponseListSuccessDto, RolesDetailQueryDto, get_id, make_thing,
	query_list_with_meta,
};
use anyhow::{Result, bail};
use imphnen_utils::DetailQueryBuilder;
use surrealdb::{Surreal, engine::remote::ws::Client};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

pub async fn update_partial_schema(
	db: &Surreal<Client>,
	table: &str,
	id: &str,
	patch: UsersSchema,
) -> Result<String> {
	let thing = make_thing(table, id);
	let record_key = get_id(&thing)?;
	let result: Option<UsersSchema> = db.update(record_key).merge(patch).await?;
	match result {
		Some(_) => Ok("Success update".into()),
		None => bail!("Failed to update"),
	}
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<UsersListItemDto>>> {
		let db = &self.state.surrealdb_ws;

		let raw_result = query_list_with_meta::<UsersListQueryDto>(
			db,
			&ResourceEnum::Users.to_string(),
			&meta,
			vec!["is_deleted = false".into()],
			None,
			"fullname",
			Some(vec![
				"email",
				"fullname",
				"id",
				"is_active",
				"is_deleted",
				"role",
				"role.permissions",
			]),
		)
		.await?;

		let data = raw_result
			.data
			.into_iter()
			.map(|schema| {
				let role = schema.clone().role.name;
				schema.from(role)
			})
			.collect::<Vec<_>>();

		Ok(ResponseListSuccessDto {
			data,
			meta: raw_result.meta,
		})
	}

	pub async fn query_user_by_email(
		&self,
		email: String,
	) -> Result<UsersDetailQueryDto> {
		let db = &self.state.surrealdb_ws;

		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_where("email")
			.where_value(email.clone())
			.with_select_fields(vec![
				"id",
				"fullname",
				"email",
				"avatar",
				"phone_number",
				"is_active",
				"is_deleted",
				"gender",
				"birthdate",
				"password",
				"created_at",
				"updated_at",
				"role",
			])
			.with_fetch("role")
			.with_fetch("role.permissions");

		let sql = builder.build();

		let user_opt: Option<UsersDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;

		let Some(user) = user_opt else {
			bail!("User not found");
		};

		if user.role.is_deleted {
			bail!("User not found");
		}

		let permissions = user
			.role
			.permissions
			.into_iter()
			.map(|perm| PermissionsQueryDto {
				id: perm.id,
				name: perm.name,
				created_at: perm.created_at,
				updated_at: perm.updated_at,
			})
			.collect();

		Ok(UsersDetailQueryDto {
			id: user.id,
			fullname: user.fullname,
			email: user.email,
			avatar: user.avatar,
			phone_number: user.phone_number,
			is_active: user.is_active,
			is_deleted: user.is_deleted,
			gender: user.gender,
			birthdate: user.birthdate,
			password: user.password,
			created_at: user.created_at,
			updated_at: user.updated_at,
			role: RolesDetailQueryDto {
				id: user.role.id,
				name: user.role.name,
				created_at: user.role.created_at,
				updated_at: user.role.updated_at,
				is_deleted: user.role.is_deleted,
				permissions,
			},
		})
	}

	pub async fn query_user_by_id(&self, id: String) -> Result<UsersDetailQueryDto> {
		let db = &self.state.surrealdb_ws;

		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_id(&id)
			.with_select_fields(vec![
				"id",
				"fullname",
				"email",
				"avatar",
				"phone_number",
				"is_active",
				"is_deleted",
				"gender",
				"birthdate",
				"password",
				"created_at",
				"updated_at",
				"role",
			])
			.with_fetch("role")
			.with_fetch("role.permissions");

		let sql = builder.build();

		let result: Option<UsersDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;

		let Some(user) = result else {
			bail!("User not found");
		};

		if user.role.is_deleted {
			bail!("User not found");
		}

		let permissions = user
			.role
			.permissions
			.into_iter()
			.map(|perm| PermissionsQueryDto {
				id: perm.id,
				name: perm.name,
				created_at: perm.created_at,
				updated_at: perm.updated_at,
			})
			.collect();

		Ok(UsersDetailQueryDto {
			id: user.id,
			fullname: user.fullname,
			email: user.email,
			avatar: user.avatar,
			phone_number: user.phone_number,
			is_active: user.is_active,
			is_deleted: user.is_deleted,
			gender: user.gender,
			birthdate: user.birthdate,
			password: user.password,
			created_at: user.created_at,
			updated_at: user.updated_at,
			role: RolesDetailQueryDto {
				id: user.role.id,
				name: user.role.name,
				created_at: user.role.created_at,
				updated_at: user.role.updated_at,
				is_deleted: user.role.is_deleted,
				permissions,
			},
		})
	}

	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.create(ResourceEnum::Users.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	pub async fn query_update_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_user_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("User already deleted");
		}
		let role_thing = if data.role == existing.role.id {
			existing.role.id
		} else {
			data.clone().role
		};
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			role: role_thing,
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_delete_user(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_id(id).await?;
		if user.is_deleted {
			bail!("User already deleted");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete user".into()),
			None => bail!("Failed to delete user"),
		}
	}
}
