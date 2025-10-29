use imphnen_entities::UsersDetailQueryDto;
use super::{UsersListItemDto, UsersListQueryDto, UsersSchema};
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id, make_thing,
};
use surrealdb::sql::Thing;
use anyhow::{Result, bail};
use imphnen_utils::{DetailQueryBuilder, QueryListBuilder, make_thing_from_enum};
use serde_json;
use std::time::Instant;
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
		let now = Instant::now();
		let result: ResponseListSuccessDto<Vec<UsersListQueryDto>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::Users.to_string(),
				&meta,
			)
			.with_condition("is_deleted = false")
			.search_field("fullname")
			.select_fields(vec!["*"])
			.fetch_fields(vec!["role", "role.permissions"])
			.build()
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_user_list' took: {elapsed:.2?}");
		}

		let data = result
			.data
			.into_iter()
			.map(UsersListQueryDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	
	pub async fn query_user_by_email(
		&self,
		email: String,
	) -> Result<UsersDetailQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_where("email", Some(email.clone()))
			.with_select_fields(vec!["*"])
			.with_fetch("role")
			.with_fetch("role.permissions");
		let sql = builder.build();
		// Some SurrealDB queries may return multiple rows (e.g., duplicates).
		// Safely take all rows and pick the first valid user (not deleted and with a valid role).
		let rows: Vec<UsersDetailQueryDto> = builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let user_opt: Option<UsersDetailQueryDto> = rows
			.into_iter()
			.find(|u| !u.is_deleted && !u.role.is_deleted && u.role.updated_at.is_some());
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_user_by_email' took: {elapsed:.2?}");
		}

		let Some(user) = user_opt else {
			bail!("User not found");
		};
		if user.is_deleted {
			bail!("User not found");
		}
		if user.role.updated_at.is_none() || user.role.is_deleted {
			bail!("User not found");
		}
		Ok(UsersDetailQueryDto::from(user))
	}

	


	pub async fn query_user_by_id(&self, id: &Thing) -> Result<UsersDetailQueryDto> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_id(id.id.to_raw())
			.with_select_fields(vec!["*"])
			.with_fetch("role")
			.with_fetch("role.permissions");
		let sql = builder.build();
		let result: Option<UsersDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_user_by_id' took: {elapsed:.2?}");
		}

		let Some(user) = result else {
			bail!("User not found in database");
		};
		if user.is_deleted {
			bail!("User not found");
		}
		if user.role.is_deleted {
			bail!("User's role has been deleted");
		}
		Ok(UsersDetailQueryDto::from(user))
	}

	
	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.create(ResourceEnum::Users.to_string())
			.content(data)
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_create_user' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	
	pub async fn query_update_user(&self, data: UsersSchema) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_user_by_id(&data.id).await?;
		if existing.is_deleted {
			bail!("User already deleted");
		}
		let role_thing = if data.role == existing.role.id {
			existing.role.id.clone()
		} else {
			data.role.clone()
		};
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			role: role_thing,
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_update_user' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	
	pub async fn query_delete_user(&self, id: String) -> Result<String> {
		let now = Instant::now();
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_id(&make_thing_from_enum(ResourceEnum::Users, &id)).await?;
		if user.is_deleted {
			bail!("User not found");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		let elapsed = now.elapsed();

		if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
			== "development"
		{
			println!("Query 'query_delete_user' took: {elapsed:.2?}");
		}

		match record {
			Some(_) => Ok("Success delete user".into()),
			None => bail!("Failed to delete user"),
		}
	}
}

