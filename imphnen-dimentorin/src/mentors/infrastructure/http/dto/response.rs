use crate::mentors::domain::{MentorDetail, MentorListItem, MentorRegistered};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorListResponseDto {
	pub id: String,
	pub user_id: String,
	pub fullname: Option<String>,
	pub email: Option<String>,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
}

impl From<MentorListItem> for MentorListResponseDto {
	fn from(item: MentorListItem) -> Self {
		Self {
			id: item.id,
			user_id: item.user_id,
			fullname: item.fullname,
			email: item.email,
			status: item.status,
			created_at: item.created_at,
			updated_at: item.updated_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorDetailResponseDto {
	pub id: String,
	pub user_id: String,
	pub fullname: Option<String>,
	pub email: Option<String>,
	pub legal_name: Option<String>,
	pub gender: Option<String>,
	pub domicile: Option<String>,
	pub phone_for_verification: Option<String>,
	pub bio: Option<String>,
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
	pub mentoring_rate: f64,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
}

impl From<MentorDetail> for MentorDetailResponseDto {
	fn from(d: MentorDetail) -> Self {
		Self {
			id: d.id,
			user_id: d.user_id,
			fullname: d.fullname,
			email: d.email,
			legal_name: d.legal_name,
			gender: d.gender,
			domicile: d.domicile,
			phone_for_verification: d.phone_for_verification,
			bio: d.bio,
			last_education: d.last_education,
			linkedin_url: d.linkedin_url,
			github_url: d.github_url,
			cv_url: d.cv_url,
			portfolio_url: d.portfolio_url,
			industries: d.industries,
			expertise: d.expertise,
			languages: d.languages,
			current_company: d.current_company,
			current_role: d.current_role,
			years_of_experience: d.years_of_experience,
			topics_of_interest: d.topics_of_interest,
			preferred_mentee_level: d.preferred_mentee_level,
			preferred_mentoring_formats: d.preferred_mentoring_formats,
			availability_commitment: d.availability_commitment,
			mentoring_rate: d.mentoring_rate,
			status: d.status,
			created_at: d.created_at,
			updated_at: d.updated_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorRegisterResponseDto {
	pub id: String,
	pub user_id: String,
	pub email: Option<String>,
	pub status: String,
	pub created_at: String,
	pub updated_at: String,
}

impl From<MentorRegistered> for MentorRegisterResponseDto {
	fn from(r: MentorRegistered) -> Self {
		Self {
			id: r.id,
			user_id: r.user_id,
			email: r.email,
			status: r.status,
			created_at: r.created_at,
			updated_at: r.updated_at,
		}
	}
}
