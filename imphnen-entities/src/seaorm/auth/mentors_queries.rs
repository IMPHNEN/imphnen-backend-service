use super::mentors::{ActiveModel, MentorBuilder};
use sea_orm::ActiveValue::Set;

impl MentorBuilder {
	pub fn build(self) -> Result<ActiveModel, String> {
		let mut active_model = <ActiveModel as std::default::Default>::default();

		if let Some(user_id) = self.user_id {
			active_model.user_id = Set(user_id);
		} else {
			return Err("User ID is required".to_string());
		}

		if let Some(industries) = self.industries {
			active_model.industries =
				Set(Some(serde_json::to_value(industries).map_err(|e| {
					format!("Failed to serialize industries: {}", e)
				})?));
		}

		if let Some(expertise) = self.expertise {
			active_model.expertise =
				Set(Some(serde_json::to_value(expertise).map_err(|e| {
					format!("Failed to serialize expertise: {}", e)
				})?));
		}

		if let Some(languages) = self.languages {
			active_model.languages =
				Set(Some(serde_json::to_value(languages).map_err(|e| {
					format!("Failed to serialize languages: {}", e)
				})?));
		}

		if let Some(current_company) = self.current_company {
			active_model.current_company = Set(Some(current_company));
		}

		if let Some(current_role) = self.current_role {
			active_model.current_role = Set(Some(current_role));
		}

		if let Some(years_of_experience) = self.years_of_experience {
			active_model.years_of_experience = Set(Some(years_of_experience));
		}

		if let Some(topics_of_interest) = self.topics_of_interest {
			active_model.topics_of_interest = Set(Some(
				serde_json::to_value(topics_of_interest)
					.map_err(|e| format!("Failed to serialize topics_of_interest: {}", e))?,
			));
		}

		if let Some(preferred_mentee_level) = self.preferred_mentee_level {
			active_model.preferred_mentee_level = Set(Some(preferred_mentee_level));
		}

		if let Some(preferred_mentoring_formats) = self.preferred_mentoring_formats {
			active_model.preferred_mentoring_formats = Set(Some(
				serde_json::to_value(preferred_mentoring_formats).map_err(|e| {
					format!("Failed to serialize preferred_mentoring_formats: {}", e)
				})?,
			));
		}

		if let Some(availability_commitment) = self.availability_commitment {
			active_model.availability_commitment = Set(Some(availability_commitment));
		}

		if let Some(mentoring_rate) = self.mentoring_rate {
			active_model.mentoring_rate = Set(Some(mentoring_rate));
		}

		if let Some(status) = self.status {
			active_model.status = Set(Some(status));
		}

		if let Some(is_deleted) = self.is_deleted {
			active_model.is_deleted = Set(is_deleted);
		}

		Ok(active_model)
	}
}
