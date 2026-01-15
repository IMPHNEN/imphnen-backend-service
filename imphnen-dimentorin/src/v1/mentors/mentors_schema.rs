use super::{
	MentorDetailQueryDto, MentorUpdateRequestDto,
	MentoringLogistics, ProfessionalProfile,
};
use crate::v1::sessions::sessions_schema::Thing;
use imphnen_entities::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentorSchema {
	pub id: Thing,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<Thing>,
	// Personal data has been moved to UsersSchema - use user_id to reference
	// phone_for_verification, bio, last_education, linkedin_url, github_url, 
	// cv_url, portfolio_url
	pub industries: Vec<String>,
	pub expertise: Vec<String>,
	pub languages: Vec<String>,
	pub current_company: String,
	pub current_role: String,
	pub years_of_experience: i32,
	pub topics_of_interest: Vec<String>,
	pub preferred_mentee_level: Vec<String>,
	pub preferred_mentoring_formats: Vec<String>,
	pub availability_commitment: String,
	pub mentoring_rate: f64,
	pub status: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for MentorSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				ResourceEnum::Mentors.to_string().as_str(),
				&Uuid::new_v4().to_string(),
			),
			user_id: Some(make_thing(
				ResourceEnum::Users.to_string().as_str(),
				&Uuid::new_v4().to_string(),
			)),
			industries: Vec::new(),
			expertise: Vec::new(),
			languages: Vec::new(),
			current_company: String::new(),
			current_role: String::new(),
			years_of_experience: 0,
			topics_of_interest: Vec::new(),
			preferred_mentee_level: Vec::new(),
			preferred_mentoring_formats: Vec::new(),
			availability_commitment: String::new(),
			mentoring_rate: 0.0,
			status: "pending".to_string(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl MentorSchema {
	pub fn create(
		professional_profile: ProfessionalProfile,
		mentoring_logistics: MentoringLogistics,
		user_id_raw: String,
	) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Mentors.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user_id: Some(make_thing(&ResourceEnum::Users.to_string(), &user_id_raw)),
			// Personal data now stored in UsersSchema, not here
			industries: professional_profile.industries,
			expertise: professional_profile.expertise,
			languages: professional_profile.languages,
			current_company: professional_profile.current_company,
			current_role: professional_profile.current_role,
			years_of_experience: professional_profile.years_of_experience,
			topics_of_interest: mentoring_logistics.topics_of_interest,
			preferred_mentee_level: mentoring_logistics.preferred_mentee_level,
			preferred_mentoring_formats: mentoring_logistics.preferred_mentoring_formats,
			availability_commitment: mentoring_logistics.availability_commitment,
			mentoring_rate: mentoring_logistics.mentoring_rate_amount as f64,
			status: "pending".to_string(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn from(dto: MentorDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			user_id: Some(dto.user_id),
			// Personal data now comes from UsersSchema via user_id
			industries: dto.industries,
			expertise: dto.expertise,
			languages: dto.languages,
			current_company: dto.current_company,
			current_role: dto.current_role,
			years_of_experience: dto.years_of_experience,
			topics_of_interest: dto.topics_of_interest,
			preferred_mentee_level: dto.preferred_mentee_level,
			preferred_mentoring_formats: dto.preferred_mentoring_formats,
			availability_commitment: dto.availability_commitment,
			mentoring_rate: dto.mentoring_rate,
			status: dto.status,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}

	pub fn update(mut self, dto: MentorUpdateRequestDto) -> Self {
		// phone_for_verification, bio, last_education, linkedin_url, github_url, 
		// cv_url, portfolio_url) are now updated in UsersSchema, not here
		
		// Only update professional fields that are still in MentorSchema
		if let Some(val) = dto.industries {
			self.industries = val;
		}
		if let Some(val) = dto.expertise {
			self.expertise = val;
		}
		if let Some(val) = dto.languages {
			self.languages = val;
		}
		if let Some(val) = dto.current_company {
			self.current_company = val;
		}
		if let Some(val) = dto.current_role {
			self.current_role = val;
		}
		if let Some(val) = dto.years_of_experience {
			self.years_of_experience = val;
		}
		if let Some(val) = dto.topics_of_interest {
			self.topics_of_interest = val;
		}
		if let Some(val) = dto.preferred_mentee_level {
			self.preferred_mentee_level = val;
		}
		if let Some(val) = dto.preferred_mentoring_formats {
			self.preferred_mentoring_formats = val;
		}
		if let Some(val) = dto.availability_commitment {
			self.availability_commitment = val;
		}
		if let Some(val) = dto.mentoring_rate_amount {
			self.mentoring_rate = val as f64;
		}

		self.updated_at = get_iso_date();
		self
	}

	pub fn update_status(mut self, status: String) -> Self {
		self.status = status;
		self.updated_at = get_iso_date();
		self
	}
}
