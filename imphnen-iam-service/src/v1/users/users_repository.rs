use super::{
	UsersActiveInactiveSchema, UsersItemDto, UsersItemDtoRaw, UsersListItemDto,
	UsersListItemDtoRaw, UsersSchema, UsersSetNewPasswordSchema,
};
use crate::{
	extract_id, get_id, make_thing, query_list_with_meta, AppState, MetaRequestDto,
	PermissionsItemDto, PermissionsItemDtoRaw, ResourceEnum, ResponseListSuccessDto,
	RolesItemDto, RolesItemDtoRaw,
};
use anyhow::{anyhow, bail, Result};

pub struct UsersRepository<'a> {
	state: &'a AppState,
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
		let mut conditions = vec!["is_deleted = false".to_string()];
		if let Some(search) = meta.search.as_deref() {
			if !search.is_empty() {
				conditions.push("string::contains(fullname ?? '', $search)".to_string());
			}
		}
		if let (Some(filter_by), Some(_filter)) =
			(meta.filter_by.as_ref(), meta.filter.as_ref())
		{
			conditions.push(format!("{} = $filter", filter_by));
		}
		let where_clause = if !conditions.is_empty() {
			format!("WHERE {}", conditions.join(" AND "))
		} else {
			String::new()
		};
		let limit = meta.per_page.unwrap_or(10);
		let page = meta.page.unwrap_or(1);
		if page < 1 || limit < 1 {
			return Err(anyhow!("Invalid pagination parameters"));
		}
		let start = (page - 1) * limit;
		let select_query = format!(
			"
			SELECT
				id,
				role.name AS role,
				fullname,
				email,
				avatar,
				phone_number,
				is_active,
				gender,
				birthdate,
				created_at,
				updated_at
			FROM {}
			{}
			LIMIT {} START {}
			FETCH role 
			",
			ResourceEnum::Users.to_string(),
			where_clause,
			limit,
			start
		);
		let raw_result = query_list_with_meta::<UsersListItemDtoRaw>(
			db,
			&ResourceEnum::Users.to_string(),
			&meta,
			vec![],
			Some(select_query),
		)
		.await?;
		let transformed_data = raw_result
			.data
			.into_iter()
			.map(|user| UsersListItemDto {
				id: extract_id(&user.id),
				fullname: user.fullname,
				email: user.email,
				avatar: user.avatar,
				phone_number: user.phone_number,
				is_active: user.is_active,
				role: user.role.unwrap_or_else(|| "-".into()),
				created_at: user.created_at,
				updated_at: user.updated_at,
			})
			.collect::<Vec<_>>();
		Ok(ResponseListSuccessDto {
			data: transformed_data,
			meta: raw_result.meta,
		})
	}

	pub async fn query_user_by_email(&self, email: String) -> Result<UsersItemDtoRaw> {
		let db = &self.state.surrealdb_ws;
		let sql = format!(
			"SELECT *, role AS role FROM {} WHERE email = $email AND is_deleted = false LIMIT 1 FETCH role, role.permissions",
			ResourceEnum::Users.to_string()
		);

		let response: Option<UsersItemDtoRaw> = db
			.query(sql)
			.bind(("email", email.clone()))
			.await?
			.take(0)?;

		match response {
			Some(user) if !user.role.is_deleted => {
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
					.collect::<Vec<_>>();
				Ok(UsersItemDtoRaw {
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
					role: RolesItemDtoRaw {
						id: user.role.id,
						name: user.role.name,
						permissions,
						created_at: user.role.created_at,
						updated_at: user.role.updated_at,
						is_deleted: user.role.is_deleted,
					},
				})
			}
			_ => bail!("User not found"),
		}
	}

	pub async fn query_user_by_id(&self, id: String) -> Result<UsersItemDto> {
		let db = &self.state.surrealdb_ws;

		let query = format!(
			"SELECT *, role AS role FROM app_users:⟨{}⟩ FETCH role, role.permissions",
			id
		);

		let mut result = db.query(query).await?;

		let response: Option<UsersItemDtoRaw> = result.take(0)?;

		match response {
			Some(user) if !user.role.is_deleted => {
				let permissions = user
					.role
					.permissions
					.into_iter()
					.map(|perm| PermissionsItemDto {
						id: extract_id(&perm.id),
						name: perm.name,
						created_at: perm.created_at,
						updated_at: perm.updated_at,
					})
					.collect::<Vec<_>>();

				Ok(UsersItemDto {
					id: extract_id(&user.id),
					fullname: user.fullname,
					email: user.email,
					avatar: user.avatar,
					phone_number: user.phone_number,
					is_active: user.is_active,
					is_deleted: user.is_deleted,
					gender: user.gender,
					birthdate: user.birthdate,
					password: user.password,
					role: RolesItemDto {
						id: extract_id(&user.role.id),
						name: user.role.name,
						permissions,
						created_at: user.role.created_at,
						updated_at: user.role.updated_at,
					},
					created_at: user.created_at,
					updated_at: user.updated_at,
				})
			}
			_ => bail!("User not found"),
		}
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
