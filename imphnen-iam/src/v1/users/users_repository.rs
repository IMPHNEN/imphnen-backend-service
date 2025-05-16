use super::{UsersDetailQueryDto, UsersListItemDto, UsersListQueryDto, UsersSchema};
use crate::{
	AppState, MetaRequestDto, PermissionsItemDto, PermissionsItemDtoRaw, ResourceEnum,
	ResponseListSuccessDto, RolesDetailQueryDto, extract_id, get_id, make_thing,
	query_list_with_meta,
};
use anyhow::{Result, bail};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

pub fn build_user_by_field_query(field: &str) -> String {
	format!(
		r#"
		SELECT *, role AS role 
		FROM {} 
		WHERE {} = $value AND is_deleted = false 
		LIMIT 1 
		FETCH role, role.permissions
		"#,
		ResourceEnum::Users.to_string(),
		field
	)
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
				schema.list_from(role)
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

		let sql = build_user_by_field_query("email");

		let user_opt: Option<UsersDetailQueryDto> = db
			.query(sql)
			.bind(("email", email.clone()))
			.await?
			.take(0)?;

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
			.map(|perm| PermissionsItemDtoRaw {
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

		let sql = build_user_by_field_query(&make_thing("app_users", &id).to_raw());

		let user_opt: Option<UsersDetailQueryDto> = db
			.query(sql)
			.bind(("email", make_thing("app_users", &id).to_raw()))
			.await?
			.take(0)?;

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
			.map(|perm| PermissionsItemDtoRaw {
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
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			role: make_thing("app_roles", &existing.role.id),
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_active_inactive_user(
		&self,
		email: String,
		data: UsersActiveInactiveSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_email(email.clone()).await?;
		if user.is_deleted {
			bail!("User already deleted");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(UsersActiveInactiveSchema {
				is_active: data.is_active,
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_active_inactive_user_by_id(
		&self,
		id: String,
		data: UsersActiveInactiveSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.update((ResourceEnum::Users.to_string(), id))
			.merge(UsersActiveInactiveSchema {
				is_active: data.is_active,
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_update_password_user(
		&self,
		email: String,
		data: UsersSetNewPasswordSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_email(email).await?;
		let record: Option<UsersSetNewPasswordSchema> = db
			.update((ResourceEnum::Users.to_string(), user.id.id.to_raw()))
			.merge(UsersSetNewPasswordSchema {
				password: data.password.clone(),
			})
			.await?;
		dbg!(record.clone());
		match record {
			Some(_) => Ok("Success update password user".into()),
			None => bail!("Failed to update password user"),
		}
	}

	pub async fn query_delete_user(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user_id = make_thing(&ResourceEnum::Users.to_string(), &id);
		let user = self.query_user_by_id(user_id.id.to_raw()).await?;
		if user.is_deleted {
			bail!("User already deleted");
		}
		let id = make_thing(&ResourceEnum::Users.to_string(), &user.id);
		let record_key = get_id(&id)?;
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
