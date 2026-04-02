use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::users::domain::{
    entity::{UpdateUserInput, UserEntity},
    repository::UserRepository,
};

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
        sqlx::query_as::<_, UserEntity>(
            "SELECT id, email, name, role, provider, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<UserEntity>, AppError> {
        sqlx::query_as::<_, UserEntity>(
            "SELECT id, email, name, role, provider, created_at, updated_at FROM users ORDER BY created_at DESC",
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn update(&self, id: Uuid, input: UpdateUserInput) -> Result<UserEntity, AppError> {
        sqlx::query_as::<_, UserEntity>(
            "UPDATE users SET name = COALESCE($1, name), email = COALESCE($2, email), updated_at = NOW() WHERE id = $3 RETURNING id, email, name, role, provider, created_at, updated_at",
        )
        .bind(input.name)
        .bind(input.email)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn update_role(&self, id: Uuid, role: String) -> Result<UserEntity, AppError> {
        sqlx::query_as::<_, UserEntity>(
            "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2 RETURNING id, email, name, role, provider, created_at, updated_at",
        )
        .bind(role)
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        Ok(())
    }
}
