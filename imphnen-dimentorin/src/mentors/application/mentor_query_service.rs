use crate::mentors::domain::{
	MentorDetail, MentorEntity, MentorListItem, MentorListPage, MentorRepository,
};
use imphnen_entities::UsersDetailQueryDto;
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use std::sync::Arc;
use uuid::Uuid;

pub fn build_detail(
	entity: &MentorEntity,
	user: Option<&UsersDetailQueryDto>,
) -> MentorDetail {
	MentorDetail {
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

pub struct MentorQueryService {
	pub repo: Arc<dyn MentorRepository>,
	pub state: Arc<AppState>,
}

impl MentorQueryService {
	pub async fn list(
		&self,
		params: PaginationParams,
	) -> Result<MentorListPage, AppError> {
		let result = self.repo.find_all(params).await?;

		let mut items: Vec<MentorListItem> = Vec::with_capacity(result.data.len());
		for entity in &result.data {
			let mut item = MentorListItem {
				id: entity.id.to_string(),
				user_id: entity.user_id.to_string(),
				fullname: None,
				email: None,
				status: entity.status.clone(),
				created_at: entity.created_at.to_rfc3339(),
				updated_at: entity.updated_at.to_rfc3339(),
			};
			if let Ok(info) = self
				.state
				.user_lookup_service
				.get_user_by_id(entity.user_id, self.state.as_ref())
				.await
			{
				item.fullname = Some(info.basic_info.fullname);
				item.email = Some(info.basic_info.email);
			}
			items.push(item);
		}

		Ok(paginator_utils::PaginatorResponse {
			data: items,
			meta: result.meta,
		})
	}

	pub async fn get_by_id(&self, id: Uuid) -> Result<MentorDetail, AppError> {
		let entity = self.repo.find_by_id(id, false).await?;
		let user = self
			.state
			.user_lookup_service
			.get_user_by_id(entity.user_id, self.state.as_ref())
			.await
			.ok()
			.map(|i| i.basic_info);
		Ok(build_detail(&entity, user.as_ref()))
	}

	pub async fn get_by_email(&self, email: &str) -> Result<MentorDetail, AppError> {
		let user_dto = self
			.state
			.user_lookup_service
			.get_user_by_email(email, self.state.as_ref())
			.await
			.map(|i| i.basic_info)
			.map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

		let user_id = Uuid::parse_str(&user_dto.id)
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let entity = self.repo.find_by_user_id(user_id, false).await?;
		Ok(build_detail(&entity, Some(&user_dto)))
	}

	pub async fn get_status(&self, email: &str) -> Result<String, AppError> {
		let user_dto = self
			.state
			.user_lookup_service
			.get_user_by_email(email, self.state.as_ref())
			.await
			.map(|i| i.basic_info)
			.map_err(|_| {
				AppError::NotFoundError(
					"No mentor application found for current user".to_string(),
				)
			})?;

		let user_id = Uuid::parse_str(&user_dto.id)
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let entity = self
			.repo
			.find_by_user_id(user_id, false)
			.await
			.map_err(|_| {
				AppError::NotFoundError(
					"No mentor application found for current user".to_string(),
				)
			})?;

		Ok(entity.status)
	}

	pub async fn get_entity_by_id(
		&self,
		id: Uuid,
		include_deleted: bool,
	) -> Result<MentorEntity, AppError> {
		self.repo.find_by_id(id, include_deleted).await
	}
}
