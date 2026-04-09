#![allow(clippy::field_reassign_with_default)]
use crate::users::domain::UserListItem;
use imphnen_entities::{
	PermissionsEnum, PermissionsQueryDto, RolesDetailQueryDto, UsersDetailQueryDto,
	seaorm::auth::roles::Entity as RolesEntity,
	seaorm::auth::users::{Column as UserColumn, Entity as UsersEntity},
};
use strum::IntoEnumIterator;
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;

fn resolve_permission_name(id: &str) -> String {
	for perm in PermissionsEnum::iter() {
		if perm.id() == id {
			return perm.to_string();
		}
	}
	id.to_string()
}

pub fn build_role_dto(
	role: Option<imphnen_entities::seaorm::auth::roles::Model>,
) -> RolesDetailQueryDto {
	role.map_or_else(RolesDetailQueryDto::default, |r| RolesDetailQueryDto {
		id: r.id.to_string(),
		name: r.name,
		permissions: r.permissions.clone().and_then(|json| {
			serde_json::from_value::<Vec<String>>(json)
				.ok()
				.map(|list| {
					list
						.into_iter()
						.map(|p| {
							let name = resolve_permission_name(&p);
							Some(PermissionsQueryDto {
								id: Some(p),
								name: Some(name),
								created_at: None,
								updated_at: None,
							})
						})
						.collect()
				})
		}),
		is_deleted: r.deleted_at.is_some(),
		created_at: Some(r.created_at.to_rfc3339()),
		updated_at: Some(r.updated_at.to_rfc3339()),
	})
}

pub fn build_user_dto(
	user: imphnen_entities::seaorm::auth::users::Model,
	role_dto: RolesDetailQueryDto,
) -> UsersDetailQueryDto {
	let mut dto = UsersDetailQueryDto::default();
	dto.id = user.id.to_string();
	dto.fullname = format!(
		"{} {}",
		user.first_name.as_deref().unwrap_or(""),
		user.last_name.as_deref().unwrap_or("")
	)
	.trim()
	.to_string();
	dto.legal_name = None;
	dto.email = user.email;
	dto.avatar = user.avatar_url;
	dto.is_active = user.is_active;
	dto.is_deleted = user.deleted_at.is_some();
	dto.profile_extension = user.metadata.and_then(|m| serde_json::from_value(m).ok());
	dto.password = user.password_hash;
	dto.role = role_dto;
	dto.created_at = user.created_at.to_rfc3339();
	dto.updated_at = user.updated_at.to_rfc3339();
	dto.mentor_id = None;
	dto
}

pub async fn query_user_list(
	db: &Arc<DatabaseConnection>,
	params: PaginationParams,
) -> Result<PaginatorResponse<UserListItem>, AppError> {
	let page = params.page.max(1);
	let per_page = params.per_page.clamp(1, 100);

	let mut query = UsersEntity::find()
		.filter(UserColumn::DeletedAt.is_null())
		.filter(UserColumn::IsActive.eq(true));

	if let Some(ref search) = params.search {
		query = query.filter(
			UserColumn::Email
				.contains(&search.query)
				.or(UserColumn::FirstName.contains(&search.query))
				.or(UserColumn::LastName.contains(&search.query)),
		);
	}

	let order = match params.sort_direction {
		Some(SortDirection::Desc) => Order::Desc,
		_ => Order::Asc,
	};
	query = match params.sort_by.as_deref() {
		Some("email") => query.order_by(UserColumn::Email, order),
		_ => query.order_by(UserColumn::CreatedAt, order),
	};

	let paginator = query.paginate(db.as_ref(), per_page as u64);
	let users = paginator
		.fetch_page((page - 1) as u64)
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

	let role_ids: Vec<Uuid> = users.iter().filter_map(|u| u.role_id).collect();
	let roles = if !role_ids.is_empty() {
		RolesEntity::find()
			.filter(imphnen_entities::seaorm::auth::roles::Column::Id.is_in(role_ids))
			.all(db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.into_iter()
			.map(|r| (r.id, r.name))
			.collect::<std::collections::HashMap<_, _>>()
	} else {
		std::collections::HashMap::new()
	};

	let data: Vec<UserListItem> = users
		.into_iter()
		.map(|user| {
			let role_name = user
				.role_id
				.and_then(|rid| roles.get(&rid).cloned())
				.unwrap_or_default();
			UserListItem {
				id: user.id.to_string(),
				role: role_name,
				fullname: format!(
					"{} {}",
					user.first_name.as_deref().unwrap_or(""),
					user.last_name.as_deref().unwrap_or("")
				)
				.trim()
				.to_string(),
				email: user.email,
				avatar: user.avatar_url,
				is_active: user.is_active,
				created_at: user.created_at.to_rfc3339(),
				updated_at: user.updated_at.to_rfc3339(),
			}
		})
		.collect();

	let total = paginator
		.num_items()
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;
	let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
	Ok(PaginatorResponse { data, meta })
}
