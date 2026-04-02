use std::sync::Arc;
use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_libs::{AppState, hash_password};
use imphnen_entities::{RolesDetailQueryDto, users::UserProfileExtensionDto};
use imphnen_iam::users::domain::{UserRepository, UserEntity};
use imphnen_iam::roles::domain::RoleRepository;
use tracing::error;
use crate::mentors::domain::{MentorEntity, MentorRepository, MentorService};
use crate::mentors::infrastructure::http::dto::{
    MentorDetailResponseDto, MentorListResponseDto, MentorRegisterResponseDto,
    MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
};

pub struct MentorServiceImpl {
    repo: Arc<dyn MentorRepository>,
    state: Arc<AppState>,
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
}

impl MentorServiceImpl {
    pub fn new(
        repo: Arc<dyn MentorRepository>,
        state: Arc<AppState>,
        user_repo: Arc<dyn UserRepository>,
        role_repo: Arc<dyn RoleRepository>,
    ) -> Self {
        Self { repo, state, user_repo, role_repo }
    }

    fn build_detail_response(
        entity: &MentorEntity,
        user: Option<&imphnen_entities::UsersDetailQueryDto>,
    ) -> MentorDetailResponseDto {
        MentorDetailResponseDto {
            id: entity.id.to_string(),
            user_id: entity.user_id.to_string(),
            fullname: user.map(|u| u.fullname.clone()),
            email: user.map(|u| u.email.clone()),
            legal_name: user.and_then(|u| u.legal_name.clone()),
            gender: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.gender.clone()),
            domicile: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.domicile.clone()),
            phone_for_verification: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.phone_for_verification.clone()),
            bio: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.bio.clone()),
            last_education: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.last_education.clone()),
            linkedin_url: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.linkedin_url.clone()),
            github_url: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.github_url.clone()),
            cv_url: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.cv_url.clone()),
            portfolio_url: user
                .and_then(|u| u.profile_extension.as_ref())
                .and_then(|ext| ext.portfolio_url.clone()),
            industries: entity.industries.clone(),
            expertise: entity.expertise.clone(),
            languages: entity.languages.clone(),
            current_company: entity.current_company.clone(),
            current_role: entity.current_role.clone(),
            years_of_experience: entity.years_of_experience,
            topics_of_interest: entity.topics_of_interest.clone(),
            preferred_mentee_level: entity.preferred_mentee_level.clone(),
            preferred_mentoring_formats: entity.preferred_mentoring_formats.clone(),
            availability_commitment: entity.availability_commitment.clone(),
            mentoring_rate: entity.mentoring_rate,
            status: entity.status.clone(),
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
        }
    }
}

#[async_trait]
impl MentorService for MentorServiceImpl {
    async fn list(
        &self,
        params: PaginationParams,
    ) -> Result<PaginatorResponse<MentorListResponseDto>, AppError> {
        let result = self.repo.find_all(params).await?;

        let mut items: Vec<MentorListResponseDto> = Vec::with_capacity(result.data.len());
        for entity in &result.data {
            let mut item = MentorListResponseDto {
                id: entity.id.to_string(),
                user_id: entity.user_id.to_string(),
                fullname: None,
                email: None,
                status: entity.status.clone(),
                created_at: entity.created_at.to_rfc3339(),
                updated_at: entity.updated_at.to_rfc3339(),
            };
            if let Ok(info) = self.state.user_lookup_service
                .get_user_by_id(entity.user_id, self.state.as_ref())
                .await
            {
                item.fullname = Some(info.basic_info.fullname);
                item.email = Some(info.basic_info.email);
            }
            items.push(item);
        }

        Ok(PaginatorResponse { data: items, meta: result.meta })
    }

    async fn get_by_id(&self, id: Uuid) -> Result<MentorDetailResponseDto, AppError> {
        let entity = self.repo.find_by_id(id, false).await?;
        let user = self.state.user_lookup_service
            .get_user_by_id(entity.user_id, self.state.as_ref())
            .await
            .ok()
            .map(|i| i.basic_info);
        Ok(Self::build_detail_response(&entity, user.as_ref()))
    }

    async fn get_by_email(&self, email: &str) -> Result<MentorDetailResponseDto, AppError> {
        let user_dto = self.state.user_lookup_service
            .get_user_by_email(email, self.state.as_ref())
            .await
            .map(|i| i.basic_info)
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

        let user_id = Uuid::parse_str(&user_dto.id)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let entity = self.repo.find_by_user_id(user_id, false).await?;
        Ok(Self::build_detail_response(&entity, Some(&user_dto)))
    }

    async fn register(
        &self,
        dto: MentorUserRegisterRequestDto,
    ) -> Result<MentorRegisterResponseDto, AppError> {
        let user_email = dto.email.clone();

        let user_id: Uuid = match self.user_repo.find_by_email(user_email.clone()).await {
            Ok(mut entity) => {
                let existing_user_id = Uuid::parse_str(&entity.id)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

                if self.repo.find_by_user_id(existing_user_id, false).await.is_ok() {
                    return Err(AppError::ConflictError(
                        "Mentor profile already exists for this user".to_string(),
                    ));
                }

                let mentor_role = self.role_repo
                    .find_by_name("Mentor".to_string())
                    .await
                    .map_err(|_| AppError::BadRequestError("Mentor Role Not Found".to_string()))?;

                entity.fullname = dto.fullname.clone();
                entity.is_active = false;
                entity.role = RolesDetailQueryDto {
                    id: mentor_role.id.to_string(),
                    name: mentor_role.name.clone(),
                    ..Default::default()
                };
                entity.password = hash_password(&dto.password).map_err(|e| {
                    error!("Failed to hash password for {}: {}", user_email, e);
                    AppError::InternalServerError("Failed to hash password".to_string())
                })?;

                let mut profile_ext = entity.profile_extension.clone().unwrap_or_default();
                profile_ext.phone_number = dto.phone_number.clone();
                profile_ext.phone_for_verification = dto.identity_and_verification.phone_for_verification.clone();
                profile_ext.gender = dto.identity_and_verification.gender.clone();
                profile_ext.domicile = dto.identity_and_verification.domicile.clone();
                profile_ext.bio = Some(dto.professional_profile.bio.clone());
                profile_ext.last_education = dto.professional_profile.last_education.clone();
                profile_ext.linkedin_url = dto.professional_profile.linkedin_url.clone();
                profile_ext.github_url = dto.professional_profile.github_url.clone();
                profile_ext.cv_url = dto.professional_profile.cv_url.clone();
                profile_ext.portfolio_url = dto.professional_profile.portfolio_url.clone();
                entity.profile_extension = Some(profile_ext);

                let uid_str = entity.id.clone();
                self.user_repo.update(entity).await.map_err(|e| {
                    error!("Failed to update user {} to mentor role: {}", user_email, e);
                    AppError::InternalServerError(e.to_string())
                })?;

                Uuid::parse_str(&uid_str)
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?
            }
            Err(_) => {
                let mentor_role = self.role_repo
                    .find_by_name("Mentor".to_string())
                    .await
                    .map_err(|_| AppError::BadRequestError("Mentor Role Not Found".to_string()))?;

                let hashed_password = hash_password(&dto.password).map_err(|e| {
                    error!("Failed to hash password for new user {}: {}", user_email, e);
                    AppError::InternalServerError("Failed to hash password".to_string())
                })?;

                let new_user_id = Uuid::new_v4();
                let profile_ext = UserProfileExtensionDto {
                    phone_number: dto.phone_number.clone(),
                    phone_for_verification: dto.identity_and_verification.phone_for_verification.clone(),
                    gender: dto.identity_and_verification.gender.clone(),
                    domicile: dto.identity_and_verification.domicile.clone(),
                    bio: Some(dto.professional_profile.bio.clone()),
                    last_education: dto.professional_profile.last_education.clone(),
                    linkedin_url: dto.professional_profile.linkedin_url.clone(),
                    github_url: dto.professional_profile.github_url.clone(),
                    cv_url: dto.professional_profile.cv_url.clone(),
                    portfolio_url: dto.professional_profile.portfolio_url.clone(),
                    ..Default::default()
                };
                let new_entity = UserEntity {
                    id: new_user_id.to_string(),
                    email: dto.email.clone(),
                    fullname: dto.fullname.clone(),
                    legal_name: Some(dto.identity_and_verification.legal_name.clone()),
                    password: hashed_password,
                    is_active: false,
                    role: RolesDetailQueryDto {
                        id: mentor_role.id.to_string(),
                        name: mentor_role.name.clone(),
                        ..Default::default()
                    },
                    profile_extension: Some(profile_ext),
                    created_at: imphnen_utils::get_iso_date(),
                    updated_at: imphnen_utils::get_iso_date(),
                    ..Default::default()
                };

                self.user_repo.create(new_entity).await.map_err(|e| {
                    error!("Failed to create new user {}: {}", user_email, e);
                    AppError::InternalServerError(e.to_string())
                })?;

                new_user_id
            }
        };

        let new_entity = MentorEntity {
            id: Uuid::new_v4(),
            user_id,
            industries: dto.professional_profile.industries.clone(),
            expertise: dto.professional_profile.expertise.clone(),
            languages: dto.professional_profile.languages.clone(),
            current_company: dto.professional_profile.current_company.clone(),
            current_role: dto.professional_profile.current_role.clone(),
            years_of_experience: dto.professional_profile.years_of_experience,
            topics_of_interest: dto.mentoring_logistics.topics_of_interest.clone(),
            preferred_mentee_level: dto.mentoring_logistics.preferred_mentee_level.clone(),
            preferred_mentoring_formats: dto.mentoring_logistics.preferred_mentoring_formats.clone(),
            availability_commitment: dto.mentoring_logistics.availability_commitment.clone(),
            mentoring_rate: dto.mentoring_logistics.mentoring_rate_amount as f64,
            status: "pending".to_string(),
            is_deleted: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let mentor_id = self.repo.create(new_entity.clone()).await.map_err(|e| {
            error!("Failed to create mentor profile for {}: {}", user_email, e);
            e
        })?;

        Ok(MentorRegisterResponseDto {
            id: mentor_id.to_string(),
            user_id: user_id.to_string(),
            email: Some(user_email),
            status: "pending".to_string(),
            created_at: new_entity.created_at.to_rfc3339(),
            updated_at: new_entity.updated_at.to_rfc3339(),
        })
    }

    async fn update(
        &self,
        id: Uuid,
        dto: MentorUpdateRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError> {
        let mut entity = self.repo.find_by_id(id, false).await?;

        if let Some(val) = dto.industries { entity.industries = val; }
        if let Some(val) = dto.expertise { entity.expertise = val; }
        if let Some(val) = dto.languages { entity.languages = val; }
        if let Some(val) = dto.current_company { entity.current_company = val; }
        if let Some(val) = dto.current_role { entity.current_role = val; }
        if let Some(val) = dto.years_of_experience { entity.years_of_experience = val; }
        if let Some(val) = dto.topics_of_interest { entity.topics_of_interest = val; }
        if let Some(val) = dto.preferred_mentee_level { entity.preferred_mentee_level = val; }
        if let Some(val) = dto.preferred_mentoring_formats { entity.preferred_mentoring_formats = val; }
        if let Some(val) = dto.availability_commitment { entity.availability_commitment = val; }
        if let Some(val) = dto.mentoring_rate_amount { entity.mentoring_rate = val as f64; }
        entity.updated_at = chrono::Utc::now();

        self.repo.update(entity).await?;

        let updated = self.repo.find_by_id(id, false).await?;
        let user = self.state.user_lookup_service
            .get_user_by_id(updated.user_id, self.state.as_ref())
            .await
            .ok()
            .map(|i| i.basic_info);
        Ok(Self::build_detail_response(&updated, user.as_ref()))
    }

    async fn update_me(
        &self,
        email: &str,
        dto: MentorUpdateRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError> {
        let user_dto = self.state.user_lookup_service
            .get_user_by_email(email, self.state.as_ref())
            .await
            .map(|i| i.basic_info)
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

        let user_id = Uuid::parse_str(&user_dto.id)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let mut entity = self.repo.find_by_user_id(user_id, false).await?;

        if let Some(val) = dto.industries { entity.industries = val; }
        if let Some(val) = dto.expertise { entity.expertise = val; }
        if let Some(val) = dto.languages { entity.languages = val; }
        if let Some(val) = dto.current_company { entity.current_company = val; }
        if let Some(val) = dto.current_role { entity.current_role = val; }
        if let Some(val) = dto.years_of_experience { entity.years_of_experience = val; }
        if let Some(val) = dto.topics_of_interest { entity.topics_of_interest = val; }
        if let Some(val) = dto.preferred_mentee_level { entity.preferred_mentee_level = val; }
        if let Some(val) = dto.preferred_mentoring_formats { entity.preferred_mentoring_formats = val; }
        if let Some(val) = dto.availability_commitment { entity.availability_commitment = val; }
        if let Some(val) = dto.mentoring_rate_amount { entity.mentoring_rate = val as f64; }
        entity.updated_at = chrono::Utc::now();

        let entity_id = entity.id;
        self.repo.update(entity).await?;

        let updated = self.repo.find_by_id(entity_id, false).await?;
        let refreshed_user = self.state.user_lookup_service
            .get_user_by_id(updated.user_id, self.state.as_ref())
            .await
            .ok()
            .map(|i| i.basic_info);
        Ok(Self::build_detail_response(&updated, refreshed_user.as_ref()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.soft_delete(id).await
    }

    async fn verify(
        &self,
        id: Uuid,
        dto: MentorVerifyRequestDto,
    ) -> Result<MentorDetailResponseDto, AppError> {
        let mut entity = self.repo.find_by_id(id, false).await?;
        entity.status = dto.status;
        entity.updated_at = chrono::Utc::now();

        self.repo.update(entity).await?;

        let updated = self.repo.find_by_id(id, false).await?;
        let user = self.state.user_lookup_service
            .get_user_by_id(updated.user_id, self.state.as_ref())
            .await
            .ok()
            .map(|i| i.basic_info);
        Ok(Self::build_detail_response(&updated, user.as_ref()))
    }

    async fn get_status(&self, email: &str) -> Result<String, AppError> {
        let user_dto = self.state.user_lookup_service
            .get_user_by_email(email, self.state.as_ref())
            .await
            .map(|i| i.basic_info)
            .map_err(|_| {
                AppError::NotFoundError("No mentor application found for current user".to_string())
            })?;

        let user_id = Uuid::parse_str(&user_dto.id)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let entity = self.repo.find_by_user_id(user_id, false).await.map_err(|_| {
            AppError::NotFoundError("No mentor application found for current user".to_string())
        })?;

        Ok(entity.status)
    }

    async fn get_entity_by_id(
        &self,
        id: Uuid,
        include_deleted: bool,
    ) -> Result<MentorEntity, AppError> {
        self.repo.find_by_id(id, include_deleted).await
    }
}
