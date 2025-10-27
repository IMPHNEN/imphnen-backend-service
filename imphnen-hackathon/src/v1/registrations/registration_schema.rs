use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

use imphnen_libs::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing, make_thing_from_enum};

use super::RegistrationRequestDto;

/// Registration status enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    Pending,
    Approved,
    Rejected,
    Waitlisted,
    Cancelled,
}

/// Participant role in hackathon
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantRole {
    Individual,
    TeamLeader,
    TeamMember,
}

/// Hackathon registration schema
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationSchema {
    pub id: Thing,
    pub hackathon_id: Thing,
    pub user_id: Thing,
    pub team_id: Option<Thing>,
    pub status: RegistrationStatus,
    pub role: ParticipantRole,
    pub registration_date: String,
    pub approved_at: Option<String>,
    pub rejected_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub checked_in: bool,
    pub check_in_time: Option<String>,
    pub notes: Option<String>,
    pub skills: Option<Vec<String>>,
    pub experience_level: Option<String>, // beginner, intermediate, advanced
    pub github_username: Option<String>,
    pub portfolio_url: Option<String>,
    pub motivation: Option<String>,
    pub dietary_requirements: Option<String>,
    pub tshirt_size: Option<String>, // XS, S, M, L, XL, XXL
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl RegistrationSchema {
    /// Create a new registration from request DTO
    pub fn from_request(
        hackathon_id: &Thing,
        user_id: &Thing,
        data: RegistrationRequestDto,
    ) -> Result<Self, String> {
        let now = get_iso_date();
        
        // Convert team_id from String to Thing if provided
        let team_id_thing = data.team_id
            .as_ref()
            .map(|id| make_thing_from_enum(ResourceEnum::Teams, id));
        
        Ok(Self {
            id: make_thing(ResourceEnum::HackathonRegistrations.as_str(), &uuid::Uuid::new_v4().to_string()),
            hackathon_id: hackathon_id.clone(),
            user_id: user_id.clone(),
            team_id: team_id_thing,
            status: RegistrationStatus::Pending,
            role: data.role.unwrap_or(ParticipantRole::Individual),
            registration_date: now.clone(),
            approved_at: None,
            rejected_at: None,
            rejection_reason: None,
            checked_in: false,
            check_in_time: None,
            notes: None,
            skills: data.skills,
            experience_level: data.experience_level,
            github_username: data.github_username,
            portfolio_url: data.portfolio_url,
            motivation: data.motivation,
            dietary_requirements: data.dietary_requirements,
            tshirt_size: data.tshirt_size,
            emergency_contact_name: data.emergency_contact_name,
            emergency_contact_phone: data.emergency_contact_phone,
            is_deleted: false,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// Update registration status
    pub fn update_status(&mut self, status: RegistrationStatus, reason: Option<String>) {
        let now = get_iso_date();
        self.status = status.clone();
        self.updated_at = now.clone();

        match status {
            RegistrationStatus::Approved => {
                self.approved_at = Some(now);
                self.rejected_at = None;
                self.rejection_reason = None;
            }
            RegistrationStatus::Rejected => {
                self.rejected_at = Some(now);
                self.rejection_reason = reason;
                self.approved_at = None;
            }
            _ => {}
        }
    }

    /// Check-in participant
    pub fn check_in(&mut self) -> Result<(), String> {
        if self.status != RegistrationStatus::Approved {
            return Err("Only approved registrations can be checked in".to_string());
        }
        
        if self.checked_in {
            return Err("Already checked in".to_string());
        }

        let now = get_iso_date();
        self.checked_in = true;
        self.check_in_time = Some(now.clone());
        self.updated_at = now;

        Ok(())
    }
}
