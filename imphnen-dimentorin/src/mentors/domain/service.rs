use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::mentor::MentorEntity;
use crate::mentors::infrastructure::http::dto::{
    MentorDetailResponseDto, MentorListResponseDto, MentorRegisterResponseDto,
    MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
};

#[async_trait]
pub trait MentorService: Send + Sync {
    async fn list(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatorResponse<MentorListResponseDto>, AppError>;

    async fn get_by_id(&self, id: Uuid) -> Result<MentorDetailResponseDto, AppError>;

    async fn get_by_email(&self, email: &str) -> Result<MentorDetailResponseDto, AppError>;

    async fn register(
        &self,
        dto: MentorUserRegisterRequestDto,
    ) -> Result<MentorRegisterResponseDto, AppError>;

    async fn update(
        &self,
        id: Uuid,
        dto: MentorUpdateRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError>;

    async fn update_me(
        &self,
        email: &str,
        dto: MentorUpdateRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError>;

    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    async fn verify(
        &self,
        id: Uuid,
        dto: MentorVerifyRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError>;

    async fn get_status(&self, email: &str) -> Result<String, AppError>;

    async fn get_entity_by_id(
        &self,
        id: Uuid,
        include_deleted: bool,
    ) -> Result<MentorEntity, AppError>;
}
