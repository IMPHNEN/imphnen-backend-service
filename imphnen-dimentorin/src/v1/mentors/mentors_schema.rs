use super::{
	IdentityAndVerification, MentorDetailQueryDto, MentorUpdateRequestDto,
	MentoringLogistics, MentoringRate, ProfessionalProfile,
};
use imphnen_libs::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentorSchema {
	pub id: Thing,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<Thing>,
	pub email: Option<String>,
	pub legal_name: String,
	pub gender: Option<String>,
	pub domicile: Option<String>,
	pub identity_document_url: String,
	pub phone_for_verification: String,
	pub bio: String,
	pub last_education: Option<String>,
	pub linkedin_url: Option<String>,
	pub github_url: Option<String>,
	pub cv_url: Option<String>,
	pub portfolio_url: Option<String>,
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
	pub mentoring_rate: MentoringRate,
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
			email: None,
			legal_name: String::new(),
			gender: None,
			domicile: None,
			identity_document_url: String::new(),
			phone_for_verification: String::new(),
			bio: String::new(),
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
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
			mentoring_rate: MentoringRate {
				amount: 0,
				currency: "IDR".to_string(),
				per_duration: "hour".to_string(),
			},
			status: "pending".to_string(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl MentorSchema {
	pub fn create(
		identity_and_verification: IdentityAndVerification,
		professional_profile: ProfessionalProfile,
		mentoring_logistics: MentoringLogistics,
		user_id_raw: String,
		email_str: String,
	) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Mentors.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user_id: Some(make_thing(&ResourceEnum::Users.to_string(), &user_id_raw)),
			email: Some(email_str),
			legal_name: identity_and_verification.legal_name,
			gender: identity_and_verification.gender,
			domicile: identity_and_verification.domicile,
			identity_document_url: identity_and_verification.identity_document_url,
			phone_for_verification: identity_and_verification.phone_for_verification,
			bio: professional_profile.bio,
			last_education: professional_profile.last_education,
			linkedin_url: professional_profile.linkedin_url,
			github_url: professional_profile.github_url,
			cv_url: professional_profile.cv_url,
			portfolio_url: professional_profile.portfolio_url,
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
			mentoring_rate: MentoringRate {
				amount: mentoring_logistics.mentoring_rate_amount,
				currency: "IDR".to_string(),
				per_duration: "hour".to_string(),
			},
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
			email: dto.email,
			legal_name: dto.legal_name,
			gender: dto.gender,
			domicile: dto.domicile,
			identity_document_url: dto.identity_document_url,
			phone_for_verification: dto.phone_for_verification,
			bio: dto.bio,
			last_education: dto.last_education,
			linkedin_url: dto.linkedin_url,
			github_url: dto.github_url,
			cv_url: dto.cv_url,
			portfolio_url: dto.portfolio_url,
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
		// Update fields only if they are Some(value), otherwise preserve current value
		if let Some(val) = dto.legal_name {
			self.legal_name = val;
		}
		if let Some(val) = dto.gender {
			self.gender = Some(val);
		}
		if let Some(val) = dto.domicile {
			self.domicile = Some(val);
		}
		if let Some(val) = dto.identity_document_url {
			self.identity_document_url = val;
		}
		if let Some(val) = dto.phone_for_verification {
			self.phone_for_verification = val;
		}
		if let Some(val) = dto.bio {
			self.bio = val;
		}
		if let Some(val) = dto.last_education {
			self.last_education = Some(val);
		}
		if let Some(val) = dto.linkedin_url {
			self.linkedin_url = Some(val);
		}
		if let Some(val) = dto.github_url {
			self.github_url = Some(val);
		}
		if let Some(val) = dto.cv_url {
			self.cv_url = Some(val);
		}
		if let Some(val) = dto.portfolio_url {
			self.portfolio_url = Some(val);
		}
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
			self.mentoring_rate.amount = val;
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
