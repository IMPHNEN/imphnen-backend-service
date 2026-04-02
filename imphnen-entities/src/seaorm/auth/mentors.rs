use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
	Clone,
	Debug,
	PartialEq,
	DeriveEntityModel,
	Serialize,
	Deserialize,
	imphnen_macros::Builder,
)]
#[sea_orm(table_name = "app_mentors")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(unique, not_null)]
	pub user_id: Uuid,

	#[sea_orm(type = "jsonb", nullable)]
	pub industries: Option<serde_json::Value>,

	#[sea_orm(type = "jsonb", nullable)]
	pub expertise: Option<serde_json::Value>,

	#[sea_orm(type = "jsonb", nullable)]
	pub languages: Option<serde_json::Value>,

	#[sea_orm(nullable)]
	pub current_company: Option<String>,

	#[sea_orm(nullable)]
	pub current_role: Option<String>,

	#[sea_orm(nullable)]
	pub years_of_experience: Option<i32>,

	#[sea_orm(type = "jsonb", nullable)]
	pub topics_of_interest: Option<serde_json::Value>,

	#[sea_orm(nullable)]
	pub preferred_mentee_level: Option<String>,

	#[sea_orm(type = "jsonb", nullable)]
	pub preferred_mentoring_formats: Option<serde_json::Value>,

	#[sea_orm(nullable)]
	pub availability_commitment: Option<String>,

	#[sea_orm(nullable)]
	pub mentoring_rate: Option<f64>,

	#[sea_orm(nullable)]
	pub status: Option<String>,

	#[sea_orm(default = "false")]
	pub is_deleted: bool,

	#[sea_orm(not_null, default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::users::Entity",
		from = "Column::UserId",
		to = "super::users::Column::Id"
	)]
	User,
}

impl Related<super::users::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Default, Serialize, Deserialize)]
pub struct MentorBuilder {
	pub user_id: Option<Uuid>,
	pub industries: Option<Vec<String>>,
	pub expertise: Option<Vec<String>>,
	pub languages: Option<Vec<String>>,
	pub current_company: Option<String>,
	pub current_role: Option<String>,
	pub years_of_experience: Option<i32>,
	pub topics_of_interest: Option<Vec<String>>,
	pub preferred_mentee_level: Option<String>,
	pub preferred_mentoring_formats: Option<Vec<String>>,
	pub availability_commitment: Option<String>,
	pub mentoring_rate: Option<f64>,
	pub status: Option<String>,
	pub is_deleted: Option<bool>,
}

impl MentorBuilder {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	#[must_use]
	pub fn user_id(mut self, user_id: Uuid) -> Self {
		self.user_id = Some(user_id);
		self
	}

	#[must_use]
	pub fn industries(mut self, industries: Vec<String>) -> Self {
		self.industries = Some(industries);
		self
	}

	#[must_use]
	pub fn expertise(mut self, expertise: Vec<String>) -> Self {
		self.expertise = Some(expertise);
		self
	}

	#[must_use]
	pub fn languages(mut self, languages: Vec<String>) -> Self {
		self.languages = Some(languages);
		self
	}

	#[must_use]
	pub fn current_company(mut self, current_company: String) -> Self {
		self.current_company = Some(current_company);
		self
	}

	#[must_use]
	pub fn current_role(mut self, current_role: String) -> Self {
		self.current_role = Some(current_role);
		self
	}

	#[must_use]
	pub fn years_of_experience(mut self, years_of_experience: i32) -> Self {
		self.years_of_experience = Some(years_of_experience);
		self
	}

	#[must_use]
	pub fn topics_of_interest(mut self, topics_of_interest: Vec<String>) -> Self {
		self.topics_of_interest = Some(topics_of_interest);
		self
	}

	#[must_use]
	pub fn preferred_mentee_level(mut self, preferred_mentee_level: String) -> Self {
		self.preferred_mentee_level = Some(preferred_mentee_level);
		self
	}

	#[must_use]
	pub fn preferred_mentoring_formats(
		mut self,
		preferred_mentoring_formats: Vec<String>,
	) -> Self {
		self.preferred_mentoring_formats = Some(preferred_mentoring_formats);
		self
	}

	#[must_use]
	pub fn availability_commitment(mut self, availability_commitment: String) -> Self {
		self.availability_commitment = Some(availability_commitment);
		self
	}

	#[must_use]
	pub fn mentoring_rate(mut self, mentoring_rate: f64) -> Self {
		self.mentoring_rate = Some(mentoring_rate);
		self
	}

	#[must_use]
	pub fn status(mut self, status: String) -> Self {
		self.status = Some(status);
		self
	}

	#[must_use]
	pub fn is_deleted(mut self, is_deleted: bool) -> Self {
		self.is_deleted = Some(is_deleted);
		self
	}
}
