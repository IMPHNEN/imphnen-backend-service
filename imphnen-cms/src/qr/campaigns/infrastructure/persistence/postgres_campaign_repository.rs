use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use sqlx::FromRow;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::campaigns::domain::{
	entity::{CampaignEntity, CreateCampaignInput},
	repository::CampaignRepository,
};

#[derive(FromRow)]
struct CampaignRow {
	pub id: Uuid,
	pub name: String,
	pub url: String,
	pub is_active: bool,
	pub created_by: Uuid,
	pub expires_at: chrono::DateTime<chrono::Utc>,
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
	pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<CampaignRow> for CampaignEntity {
	fn from(row: CampaignRow) -> Self {
		CampaignEntity {
			id: row.id,
			name: row.name,
			url: row.url,
			is_active: row.is_active,
			created_by: row.created_by,
			expires_at: row.expires_at,
			created_at: row.created_at,
			updated_at: row.updated_at,
		}
	}
}

pub struct PostgresCampaignRepository {
	pool: Arc<PgPool>,
}

impl PostgresCampaignRepository {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl CampaignRepository for PostgresCampaignRepository {
	async fn create(
		&self,
		input: CreateCampaignInput,
	) -> Result<CampaignEntity, AppError> {
		let mut tx = self
			.pool
			.begin()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		sqlx::query("UPDATE qr_campaigns SET is_active = false, updated_at = NOW()")
			.execute(&mut *tx)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let id = Uuid::new_v4();
		let campaign = sqlx::query_as::<_, CampaignRow>(
            "INSERT INTO qr_campaigns (id, name, url, qr_code_data, is_active, created_by, expires_at) \
             VALUES ($1, $2, $3, $4, true, $5, NOW() + INTERVAL '30 days') \
             RETURNING id, name, url, is_active, created_by, expires_at, created_at, updated_at",
        )
        .bind(id)
        .bind(&input.name)
        .bind(&input.url)
        .bind(&input.qr_code_data)
        .bind(input.created_by)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

		tx.commit()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(campaign.into())
	}

	async fn find_all(&self) -> Result<Vec<CampaignEntity>, AppError> {
		sqlx::query_as::<_, CampaignRow>(
			"SELECT id, name, url, is_active, created_by, expires_at, created_at, updated_at \
             FROM qr_campaigns ORDER BY created_at DESC",
		)
		.fetch_all(self.pool.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))
		.map(|rows| rows.into_iter().map(Into::into).collect())
	}

	async fn find_active_qr_data(&self) -> Result<Option<Vec<u8>>, AppError> {
		let row = sqlx::query_as::<_, (Vec<u8>,)>(
			"SELECT qr_code_data FROM qr_campaigns WHERE is_active = true LIMIT 1",
		)
		.fetch_optional(self.pool.as_ref())
		.await
		.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(row.map(|r| r.0))
	}

	async fn set_active(&self, id: Uuid) -> Result<CampaignEntity, AppError> {
		let mut tx = self
			.pool
			.begin()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		sqlx::query("UPDATE qr_campaigns SET is_active = false, updated_at = NOW()")
			.execute(&mut *tx)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let campaign = sqlx::query_as::<_, CampaignRow>(
            "UPDATE qr_campaigns SET is_active = true, updated_at = NOW() WHERE id = $1 \
             RETURNING id, name, url, is_active, created_by, expires_at, created_at, updated_at",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

		tx.commit()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(campaign.into())
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		sqlx::query("DELETE FROM qr_campaigns WHERE id = $1")
			.bind(id)
			.execute(self.pool.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
