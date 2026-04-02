use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use sqlx::FromRow;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::users::domain::{
	entity::{UpdateUserInput, UserEntity},
	repository::UserRepository,
};

#[derive(FromRow)]
struct UserRow {
	pub id: Uuid,
	pub email: String,
	pub name: String,
	pub role: String,
	pub provider: String,
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
	pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<UserRow> for UserEntity {
	fn from(row: UserRow) -> Self {
		UserEntity {
			id: row.id,
			email: row.email,
			name: row.name,
			role: row.role,
			provider: row.provider,
			created_at: row.created_at,
			updated_at: row.updated_at,
		}
	}
}

pub struct PostgresUserRepository {
	pool: Arc<PgPool>,
}

impl PostgresUserRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
	async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, AppError> {
		sqlx::query_as::<_, UserRow>(
            "SELECT id, email, name, role, provider, created_at, updated_at FROM qr_users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
        .map(|opt| opt.map(Into::into))
	}

	async fn find_all(&self) -> Result<Vec<UserEntity>, AppError> {
		sqlx::query_as::<_, UserRow>(
            "SELECT id, email, name, role, provider, created_at, updated_at FROM qr_users ORDER BY created_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
        .map(|rows| rows.into_iter().map(Into::into).collect())
	}

	async fn update(
		&self,
		id: Uuid,
		input: UpdateUserInput,
	) -> Result<UserEntity, AppError> {
		sqlx::query_as::<_, UserRow>(
            "UPDATE qr_users SET name = COALESCE($1, name), email = COALESCE($2, email), updated_at = NOW() WHERE id = $3 RETURNING id, email, name, role, provider, created_at, updated_at",
        )
        .bind(input.name)
        .bind(input.email)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
        .map(Into::into)
	}

	async fn update_role(
		&self,
		id: Uuid,
		role: String,
	) -> Result<UserEntity, AppError> {
		sqlx::query_as::<_, UserRow>(
            "UPDATE qr_users SET role = $1, updated_at = NOW() WHERE id = $2 RETURNING id, email, name, role, provider, created_at, updated_at",
        )
        .bind(role)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
        .map(Into::into)
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		sqlx::query("DELETE FROM qr_users WHERE id = $1")
			.bind(id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
