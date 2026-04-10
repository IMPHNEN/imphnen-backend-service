use crate::roadmap::domain::{roadmap::RoadmapEntity, repository::RoadmapRepository};
use async_trait::async_trait;
use imphnen_entities::seaorm::common::roadmap_items::{
	ActiveModel as RoadmapActiveModel, Column as RoadmapColumn, Entity as RoadmapEntity_,
	Model as RoadmapModel,
};
use imphnen_utils::AppError;
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, PaginatorTrait, QueryOrder};
use std::sync::Arc;
use uuid::Uuid;

fn to_entity(model: RoadmapModel) -> RoadmapEntity {
	RoadmapEntity {
		id: model.id,
		title: model.title,
		description: model.description,
		status: model.status,
		votes: model.votes,
		is_deleted: model.is_deleted,
		created_at: model.created_at,
		updated_at: model.updated_at,
	}
}

pub struct PostgresRoadmapRepository {
	db: Arc<DatabaseConnection>,
}

impl PostgresRoadmapRepository {
	pub fn new(db: DatabaseConnection) -> Self {
		Self { db: Arc::new(db) }
	}
}

#[async_trait]
impl RoadmapRepository for PostgresRoadmapRepository {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoadmapEntity>, AppError> {
		let page = params.page.max(1);
		let per_page = params.per_page.clamp(1, 100);

		let mut query = RoadmapEntity_::find().filter(RoadmapColumn::IsDeleted.eq(false));

		if let Some(ref search) = params.search {
			query = query.filter(RoadmapColumn::Title.contains(&search.query));
		}

		query = match params.sort_by.as_deref() {
			Some("title") => match params.sort_direction {
				Some(SortDirection::Desc) => query.order_by(RoadmapColumn::Title, Order::Desc),
				_ => query.order_by(RoadmapColumn::Title, Order::Asc),
			},
			Some("votes") => match params.sort_direction {
				Some(SortDirection::Asc) => query.order_by(RoadmapColumn::Votes, Order::Asc),
				_ => query.order_by(RoadmapColumn::Votes, Order::Desc),
			},
			_ => match params.sort_direction {
				Some(SortDirection::Asc) => {
					query.order_by(RoadmapColumn::CreatedAt, Order::Asc)
				}
				_ => query.order_by(RoadmapColumn::CreatedAt, Order::Desc),
			},
		};

		let paginator = query.paginate(self.db.as_ref(), per_page as u64);
		let total = paginator
			.num_items()
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		let items = paginator
			.fetch_page((page - 1) as u64)
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let data = items.into_iter().map(to_entity).collect();
		let meta = PaginatorResponseMeta::new(page, per_page, total as u32);
		Ok(PaginatorResponse { data, meta })
	}

	async fn find_by_id(&self, id: Uuid) -> Result<RoadmapEntity, AppError> {
		let item = RoadmapEntity_::find_by_id(id)
			.filter(RoadmapColumn::IsDeleted.eq(false))
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Roadmap item not found".to_string()))?;

		Ok(to_entity(item))
	}

	async fn create(&self, entity: RoadmapEntity) -> Result<(), AppError> {
		let active_model = RoadmapActiveModel {
			id: ActiveValue::Set(entity.id),
			title: ActiveValue::Set(entity.title),
			description: ActiveValue::Set(entity.description),
			status: ActiveValue::Set(entity.status),
			votes: ActiveValue::Set(0),
			is_deleted: ActiveValue::Set(false),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
		};

		RoadmapEntity_::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}

	async fn update(&self, entity: RoadmapEntity) -> Result<(), AppError> {
		let mut active_model: RoadmapActiveModel = RoadmapEntity_::find_by_id(entity.id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Roadmap item not found".to_string()))?
			.into();

		active_model.title = ActiveValue::Set(entity.title);
		active_model.description = ActiveValue::Set(entity.description);
		active_model.status = ActiveValue::Set(entity.status);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		let mut active_model: RoadmapActiveModel = RoadmapEntity_::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Roadmap item not found".to_string()))?
			.into();

		active_model.is_deleted = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}

	async fn increment_votes(&self, id: Uuid) -> Result<(), AppError> {
		let item = RoadmapEntity_::find_by_id(id)
			.filter(RoadmapColumn::IsDeleted.eq(false))
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Roadmap item not found".to_string()))?;

		let new_votes = item.votes + 1;
		let mut active_model: RoadmapActiveModel = item.into();
		active_model.votes = ActiveValue::Set(new_votes);
		active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
		active_model
			.update(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;
		Ok(())
	}
}
