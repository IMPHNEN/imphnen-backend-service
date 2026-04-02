use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::mentor::MentorEntity;


#[async_trait]
pub trait MentorRepository: Send + Sync {
    async fn find_all(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatorResponse<MentorEntity>, AppError>;

    async fn find_by_id(
        &self,
        id: Uuid,
        include_deleted: bool,
    ) -> Result<MentorEntity, AppError>;

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        include_deleted: bool,
    ) -> Result<MentorEntity, AppError>;

    async fn create(&self, entity: MentorEntity) -> Result<Uuid, AppError>;

    async fn update(&self, entity: MentorEntity) -> Result<(), AppError>;

    async fn soft_delete(&self, id: Uuid) -> Result<(), AppError>;
}
