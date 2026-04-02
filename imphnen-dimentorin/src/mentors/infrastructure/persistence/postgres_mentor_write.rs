use crate::mentors::domain::mentor::MentorEntity;
use imphnen_entities::seaorm::auth::mentors::ActiveModel as MentorActiveModel;
use imphnen_utils::AppError;
use sea_orm::ActiveValue;

pub fn apply_entity_to_model(
	entity: &MentorEntity,
	active_model: &mut MentorActiveModel,
) -> Result<(), AppError> {
	if !entity.industries.is_empty() {
		active_model.industries = ActiveValue::Set(Some(
			serde_json::to_value(&entity.industries)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.expertise.is_empty() {
		active_model.expertise = ActiveValue::Set(Some(
			serde_json::to_value(&entity.expertise)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.languages.is_empty() {
		active_model.languages = ActiveValue::Set(Some(
			serde_json::to_value(&entity.languages)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.current_company.is_empty() {
		active_model.current_company =
			ActiveValue::Set(Some(entity.current_company.clone()));
	}
	if !entity.current_role.is_empty() {
		active_model.current_role = ActiveValue::Set(Some(entity.current_role.clone()));
	}
	active_model.years_of_experience =
		ActiveValue::Set(Some(entity.years_of_experience));
	if !entity.topics_of_interest.is_empty() {
		active_model.topics_of_interest = ActiveValue::Set(Some(
			serde_json::to_value(&entity.topics_of_interest)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.preferred_mentee_level.is_empty() {
		active_model.preferred_mentee_level = ActiveValue::Set(Some(
			serde_json::to_string(&entity.preferred_mentee_level)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.preferred_mentoring_formats.is_empty() {
		active_model.preferred_mentoring_formats = ActiveValue::Set(Some(
			serde_json::to_value(&entity.preferred_mentoring_formats)
				.map_err(|e| AppError::InternalServerError(e.to_string()))?,
		));
	}
	if !entity.availability_commitment.is_empty() {
		active_model.availability_commitment =
			ActiveValue::Set(Some(entity.availability_commitment.clone()));
	}
	active_model.mentoring_rate = ActiveValue::Set(Some(entity.mentoring_rate));
	if !entity.status.is_empty() {
		active_model.status = ActiveValue::Set(Some(entity.status.clone()));
	}
	active_model.updated_at = ActiveValue::Set(chrono::Utc::now());
	Ok(())
}
