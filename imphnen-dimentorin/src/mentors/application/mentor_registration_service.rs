use super::mentor_query_service::build_detail;
use crate::mentors::domain::{
	MentorDetail, MentorEntity, MentorRegisterCommand, MentorRegistered,
	MentorRepository, MentorVerifyCommand,
};
use imphnen_entities::{RolesDetailQueryDto, users::UserProfileExtensionDto};
use imphnen_iam::roles::domain::RoleRepository;
use imphnen_iam::users::domain::{UserEntity, UserRepository};
use imphnen_libs::{AppState, hash_password};
use imphnen_utils::AppError;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

pub struct MentorRegistrationService {
	pub repo: Arc<dyn MentorRepository>,
	pub state: Arc<AppState>,
	pub user_repo: Arc<dyn UserRepository>,
	pub role_repo: Arc<dyn RoleRepository>,
}

impl MentorRegistrationService {
	pub async fn register(
		&self,
		cmd: MentorRegisterCommand,
	) -> Result<MentorRegistered, AppError> {
		let user_email = cmd.email.clone();

		let user_id: Uuid = match self.user_repo.find_by_email(user_email.clone()).await
		{
			Ok(mut entity) => {
				let existing_user_id = Uuid::parse_str(&entity.id)
					.map_err(|e| AppError::InternalServerError(e.to_string()))?;

				if self
					.repo
					.find_by_user_id(existing_user_id, false)
					.await
					.is_ok()
				{
					return Err(AppError::ConflictError(
						"Mentor profile already exists for this user".to_string(),
					));
				}

				let mentor_role = self
					.role_repo
					.find_by_name("Mentor".to_string())
					.await
					.map_err(|_| {
						AppError::BadRequestError("Mentor Role Not Found".to_string())
					})?;

				entity.fullname = cmd.fullname.clone();
				entity.is_active = false;
				entity.role = RolesDetailQueryDto {
					id: mentor_role.id.to_string(),
					name: mentor_role.name.clone(),
					..Default::default()
				};
				entity.password = hash_password(&cmd.password).map_err(|e| {
					error!("Failed to hash password for {}: {}", user_email, e);
					AppError::InternalServerError("Failed to hash password".to_string())
				})?;

				let mut profile_ext = entity.profile_extension.clone().unwrap_or_default();
				profile_ext.phone_number = cmd.phone_number.clone();
				profile_ext.phone_for_verification = cmd.phone_for_verification.clone();
				profile_ext.gender = cmd.gender.clone();
				profile_ext.domicile = cmd.domicile.clone();
				profile_ext.bio = Some(cmd.bio.clone());
				profile_ext.last_education = cmd.last_education.clone();
				profile_ext.linkedin_url = cmd.linkedin_url.clone();
				profile_ext.github_url = cmd.github_url.clone();
				profile_ext.cv_url = cmd.cv_url.clone();
				profile_ext.portfolio_url = cmd.portfolio_url.clone();
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
				let mentor_role = self
					.role_repo
					.find_by_name("Mentor".to_string())
					.await
					.map_err(|_| {
						AppError::BadRequestError("Mentor Role Not Found".to_string())
					})?;

				let hashed_password = hash_password(&cmd.password).map_err(|e| {
					error!("Failed to hash password for new user {}: {}", user_email, e);
					AppError::InternalServerError("Failed to hash password".to_string())
				})?;

				let new_user_id = Uuid::new_v4();
				let profile_ext = UserProfileExtensionDto {
					phone_number: cmd.phone_number.clone(),
					phone_for_verification: cmd.phone_for_verification.clone(),
					gender: cmd.gender.clone(),
					domicile: cmd.domicile.clone(),
					bio: Some(cmd.bio.clone()),
					last_education: cmd.last_education.clone(),
					linkedin_url: cmd.linkedin_url.clone(),
					github_url: cmd.github_url.clone(),
					cv_url: cmd.cv_url.clone(),
					portfolio_url: cmd.portfolio_url.clone(),
					..Default::default()
				};
				let new_entity = UserEntity {
					id: new_user_id.to_string(),
					email: cmd.email.clone(),
					fullname: cmd.fullname.clone(),
					legal_name: Some(cmd.legal_name.clone()),
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
			industries: cmd.industries.clone(),
			expertise: cmd.expertise.clone(),
			languages: cmd.languages.clone(),
			current_company: cmd.current_company.clone(),
			current_role: cmd.current_role.clone(),
			years_of_experience: cmd.years_of_experience,
			topics_of_interest: cmd.topics_of_interest.clone(),
			preferred_mentee_level: cmd.preferred_mentee_level.clone(),
			preferred_mentoring_formats: cmd.preferred_mentoring_formats.clone(),
			availability_commitment: cmd.availability_commitment.clone(),
			mentoring_rate: cmd.mentoring_rate_amount as f64,
			status: "pending".to_string(),
			is_deleted: false,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
		};

		let mentor_id = self.repo.create(new_entity.clone()).await.map_err(|e| {
			error!("Failed to create mentor profile for {}: {}", user_email, e);
			e
		})?;

		Ok(MentorRegistered {
			id: mentor_id.to_string(),
			user_id: user_id.to_string(),
			email: Some(user_email),
			status: "pending".to_string(),
			created_at: new_entity.created_at.to_rfc3339(),
			updated_at: new_entity.updated_at.to_rfc3339(),
		})
	}

	pub async fn verify(
		&self,
		id: Uuid,
		cmd: MentorVerifyCommand,
	) -> Result<MentorDetail, AppError> {
		let mut entity = self.repo.find_by_id(id, false).await?;
		entity.status = cmd.status;
		entity.updated_at = chrono::Utc::now();

		self.repo.update(entity).await?;

		let updated = self.repo.find_by_id(id, false).await?;
		let user = self
			.state
			.user_lookup_service
			.get_user_by_id(updated.user_id, self.state.as_ref())
			.await
			.ok()
			.map(|i| i.basic_info);
		Ok(build_detail(&updated, user.as_ref()))
	}

	pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.soft_delete(id).await
	}
}
