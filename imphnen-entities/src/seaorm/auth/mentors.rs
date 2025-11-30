//! SeaORM entity for Mentors table
//! Corresponding to ResourceEnum::Mentors

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, imphnen_macros::Builder)]
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
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::UserId", to = "super::users::Column::Id")]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // Default implementation - SeaORM will handle timestamps automatically
}

// Builder pattern for Mentor creation
#[derive(Default, Serialize, Deserialize)]
pub struct MentorBuilder {
    user_id: Option<Uuid>,
    industries: Option<Vec<String>>,
    expertise: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    current_company: Option<String>,
    current_role: Option<String>,
    years_of_experience: Option<i32>,
    topics_of_interest: Option<Vec<String>>,
    preferred_mentee_level: Option<String>,
    preferred_mentoring_formats: Option<Vec<String>>,
    availability_commitment: Option<String>,
    mentoring_rate: Option<f64>,
    status: Option<String>,
    is_deleted: Option<bool>,
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
    pub fn preferred_mentoring_formats(mut self, preferred_mentoring_formats: Vec<String>) -> Self {
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

    pub fn build(self) -> Result<ActiveModel, String> {
        let mut active_model = <ActiveModel as std::default::Default>::default();

        if let Some(user_id) = self.user_id {
            active_model.user_id = Set(user_id);
        } else {
            return Err("User ID is required".to_string());
        }

        if let Some(industries) = self.industries {
            active_model.industries = Set(Some(serde_json::to_value(industries).map_err(|e| format!("Failed to serialize industries: {}", e))?));
        }

        if let Some(expertise) = self.expertise {
            active_model.expertise = Set(Some(serde_json::to_value(expertise).map_err(|e| format!("Failed to serialize expertise: {}", e))?));
        }

        if let Some(languages) = self.languages {
            active_model.languages = Set(Some(serde_json::to_value(languages).map_err(|e| format!("Failed to serialize languages: {}", e))?));
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
            active_model.topics_of_interest = Set(Some(serde_json::to_value(topics_of_interest).map_err(|e| format!("Failed to serialize topics_of_interest: {}", e))?));
        }

        if let Some(preferred_mentee_level) = self.preferred_mentee_level {
            active_model.preferred_mentee_level = Set(Some(preferred_mentee_level));
        }

        if let Some(preferred_mentoring_formats) = self.preferred_mentoring_formats {
            active_model.preferred_mentoring_formats = Set(Some(serde_json::to_value(preferred_mentoring_formats).map_err(|e| format!("Failed to serialize preferred_mentoring_formats: {}", e))?));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seaorm::common::utils::generate_uuid;
    use serde_json::json;

    #[test]
    fn test_mentor_model_creation() {
        let uid = generate_uuid();
        let mentor = MentorBuilder::new()
            .user_id(uid)
            .industries(vec!["Technology".to_string(), "Finance".to_string()])
            .expertise(vec!["Blockchain".to_string(), "AI".to_string()])
            .languages(vec!["English".to_string(), "Spanish".to_string()])
            .current_company("Tech Corp".to_string())
            .current_role("Senior Engineer".to_string())
            .years_of_experience(10)
            .topics_of_interest(vec!["Web3".to_string(), "Machine Learning".to_string()])
            .preferred_mentee_level("Intermediate".to_string())
            .preferred_mentoring_formats(vec!["1:1".to_string(), "Group".to_string()])
            .availability_commitment("Weekly".to_string())
            .mentoring_rate(150.0)
            .status("active".to_string())
            .build();

        assert!(mentor.is_ok());
        let mentor_model = mentor.unwrap();
        assert_eq!(mentor_model.user_id, Set(uid));
        assert_eq!(mentor_model.industries, Set(Some(json!(["Technology", "Finance"]))));
        assert_eq!(mentor_model.expertise, Set(Some(json!(["Blockchain", "AI"]))));
    }

    #[test]
    fn test_mentor_model_missing_required_fields() {
        let mentor = MentorBuilder::new()
            // Missing user_id
            .industries(vec!["Technology".to_string()])
            .build();

        assert!(mentor.is_err());
        assert_eq!(mentor.unwrap_err(), "User ID is required");
    }
}