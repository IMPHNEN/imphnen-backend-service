use super::mentor_query_service::build_detail;
use crate::mentors::domain::{MentorDetail, MentorRepository, MentorUpdateCommand};
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct MentorUpdateService {
	pub repo: Arc<dyn MentorRepository>,
	pub state: Arc<AppState>,
}

impl MentorUpdateService {
	pub async fn update(
		&self,
		id: Uuid,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError> {
		let mut entity = self.repo.find_by_id(id, false).await?;

		if let Some(val) = cmd.industries {
			entity.industries = val;
		}
		if let Some(val) = cmd.expertise {
			entity.expertise = val;
		}
		if let Some(val) = cmd.languages {
			entity.languages = val;
		}
		if let Some(val) = cmd.current_company {
			entity.current_company = val;
		}
		if let Some(val) = cmd.current_role {
			entity.current_role = val;
		}
		if let Some(val) = cmd.years_of_experience {
			entity.years_of_experience = val;
		}
		if let Some(val) = cmd.topics_of_interest {
			entity.topics_of_interest = val;
		}
		if let Some(val) = cmd.preferred_mentee_level {
			entity.preferred_mentee_level = val;
		}
		if let Some(val) = cmd.preferred_mentoring_formats {
			entity.preferred_mentoring_formats = val;
		}
		if let Some(val) = cmd.availability_commitment {
			entity.availability_commitment = val;
		}
		if let Some(val) = cmd.mentoring_rate_amount {
			entity.mentoring_rate = val as f64;
		}
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

	pub async fn update_me(
		&self,
		email: &str,
		cmd: MentorUpdateCommand,
	) -> Result<MentorDetail, AppError> {
		let user_dto = self
			.state
			.user_lookup_service
			.get_user_by_email(email, self.state.as_ref())
			.await
			.map(|i| i.basic_info)
			.map_err(|_| AppError::NotFoundError("User not found".to_string()))?;

		let user_id = Uuid::parse_str(&user_dto.id)
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		let mut entity = self.repo.find_by_user_id(user_id, false).await?;

		if let Some(val) = cmd.industries {
			entity.industries = val;
		}
		if let Some(val) = cmd.expertise {
			entity.expertise = val;
		}
		if let Some(val) = cmd.languages {
			entity.languages = val;
		}
		if let Some(val) = cmd.current_company {
			entity.current_company = val;
		}
		if let Some(val) = cmd.current_role {
			entity.current_role = val;
		}
		if let Some(val) = cmd.years_of_experience {
			entity.years_of_experience = val;
		}
		if let Some(val) = cmd.topics_of_interest {
			entity.topics_of_interest = val;
		}
		if let Some(val) = cmd.preferred_mentee_level {
			entity.preferred_mentee_level = val;
		}
		if let Some(val) = cmd.preferred_mentoring_formats {
			entity.preferred_mentoring_formats = val;
		}
		if let Some(val) = cmd.availability_commitment {
			entity.availability_commitment = val;
		}
		if let Some(val) = cmd.mentoring_rate_amount {
			entity.mentoring_rate = val as f64;
		}
		entity.updated_at = chrono::Utc::now();

		let entity_id = entity.id;
		self.repo.update(entity).await?;

		let updated = self.repo.find_by_id(entity_id, false).await?;
		let refreshed_user = self
			.state
			.user_lookup_service
			.get_user_by_id(updated.user_id, self.state.as_ref())
			.await
			.ok()
			.map(|i| i.basic_info);
		Ok(build_detail(&updated, refreshed_user.as_ref()))
	}
}
