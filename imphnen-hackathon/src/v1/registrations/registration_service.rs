use axum::response::Response;
use axum::http::StatusCode;
use imphnen_entities::ResponseSuccessDto;
use imphnen_libs::AppState;
use imphnen_utils::{
    common_response, extract_id, make_thing_from_enum, success_response, validate_request,
};
use surrealdb::sql::Thing;

use super::{
    CheckInResponseDto, RegistrationListItemDto, RegistrationListResponseDto,
    RegistrationRequestDto, RegistrationResponseDto, RegistrationSchema, 
    RegistrationStatsDto, RegistrationStatus,
    RegistrationsRepository, UpdateRegistrationStatusRequestDto,
    UpdateRegistrationStatusResponseDto, UserHackathonDto, UserHackathonsResponseDto,
};
use crate::v1::hackathon::HackathonRepository;
use imphnen_libs::ResourceEnum;

pub struct RegistrationsService<'a> {
    state: &'a AppState,
}

impl<'a> RegistrationsService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    // ============================================
    // Register for Hackathon
    // ============================================
    pub async fn register_hackathon(
        &self,
        hackathon_id: &Thing,
        user_email: &str,
        data: RegistrationRequestDto,
    ) -> Response {
        // Validate request
        if let Err((status, message)) = validate_request(&data) {
            return common_response(status, &message);
        }

        let repository = RegistrationsRepository::new(self.state);

        // Get user ID from email
        let user_id = make_thing_from_enum(ResourceEnum::Users, user_email);

        // Check if hackathon exists
        let hackathon_repo = HackathonRepository::new(self.state);
        match hackathon_repo.get_by_id(hackathon_id).await {
            Ok(None) => {
                return common_response(StatusCode::NOT_FOUND, "Hackathon not found");
            }
            Err(e) => {
                return common_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to verify hackathon: {}", e));
            }
            Ok(Some(_)) => {} // Hackathon exists, continue
        }

        // Check if user already registered
        match repository
            .check_existing_registration(hackathon_id, &user_id)
            .await
        {
            Ok(Some(_)) => {
                return common_response(StatusCode::CONFLICT, "You have already registered for this hackathon")
            }
            Ok(None) => {}
            Err(e) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }

        // Create registration
        let registration = match RegistrationSchema::from_request(hackathon_id, &user_id, data) {
            Ok(reg) => reg,
            Err(e) => return common_response(StatusCode::BAD_REQUEST, &e),
        };

        match repository.create_registration(registration).await {
            Ok(created) => {
                let response = RegistrationResponseDto {
                    id: extract_id(&created.id),
                    hackathon_id: extract_id(&created.hackathon_id),
                    user_id: extract_id(&created.user_id),
                    team_id: created.team_id.as_ref().map(|t| extract_id(t)),
                    status: created.status,
                    role: created.role,
                    registration_date: created.registration_date,
                    checked_in: created.checked_in,
                    message: "Registration submitted successfully. You will be notified once approved."
                        .to_string(),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }

    // ============================================
    // List Registrations for Hackathon
    // ============================================
    pub async fn get_hackathon_registrations(
        &self,
        hackathon_id: &Thing,
        status_filter: Option<String>,
    ) -> Response {
        let repository = RegistrationsRepository::new(self.state);

        // Parse status filter if provided
        let status_enum = if let Some(status_str) = &status_filter {
            match status_str.to_lowercase().as_str() {
                "pending" => Some(RegistrationStatus::Pending),
                "approved" => Some(RegistrationStatus::Approved),
                "rejected" => Some(RegistrationStatus::Rejected),
                "waitlisted" => Some(RegistrationStatus::Waitlisted),
                "cancelled" => Some(RegistrationStatus::Cancelled),
                _ => return common_response(StatusCode::BAD_REQUEST, "Invalid status filter"),
            }
        } else {
            None
        };

        match repository
            .query_hackathon_registrations(hackathon_id, status_enum)
            .await
        {
            Ok(results) => {
                let registrations = results
                    .into_iter()
                    .map(|r| RegistrationListItemDto {
                        id: r.id,
                        hackathon_id: r.hackathon_id,
                        hackathon_name: r.hackathon_name,
                        user_id: r.user_id,
                        user_fullname: r.user_fullname,
                        user_email: r.user_email,
                        team_id: r.team_id,
                        team_name: r.team_name,
                        status: r.status,
                        role: r.role,
                        registration_date: r.registration_date,
                        checked_in: r.checked_in,
                        check_in_time: r.check_in_time,
                        experience_level: r.experience_level,
                        skills: r.skills,
                    })
                    .collect::<Vec<_>>();

                let total = registrations.len();
                let response = RegistrationListResponseDto {
                    registrations,
                    total,
                    status_filter,
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }

    // ============================================
    // Get Current User's Hackathon Registrations
    // ============================================
    pub async fn get_my_hackathons(&self, user_email: &str) -> Response {
        let repository = RegistrationsRepository::new(self.state);

        // Get user ID from email
        let user_id = make_thing_from_enum(ResourceEnum::Users, user_email);

        match repository.query_user_hackathons(&user_id).await {
            Ok(results) => {
                let hackathons = results
                    .into_iter()
                    .map(|h| UserHackathonDto {
                        registration_id: h.registration_id,
                        hackathon_id: h.hackathon_id,
                        hackathon_name: h.hackathon_name,
                        hackathon_description: h.hackathon_description,
                        start_date: h.start_date,
                        end_date: h.end_date,
                        status: h.status,
                        role: h.role,
                        registration_date: h.registration_date,
                        checked_in: h.checked_in,
                        team_id: h.team_id,
                        team_name: h.team_name,
                    })
                    .collect::<Vec<_>>();

                let total = hackathons.len();
                let response = UserHackathonsResponseDto { hackathons, total };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }

    // ============================================
    // Update Registration Status
    // ============================================
    pub async fn update_registration_status(
        &self,
        registration_id: &Thing,
        data: UpdateRegistrationStatusRequestDto,
    ) -> Response {
        // Validate request
        if let Err((status, message)) = validate_request(&data) {
            return common_response(status, &message);
        }

        let repository = RegistrationsRepository::new(self.state);

        // Get existing registration
        let mut registration = match repository.query_registration_by_id(registration_id).await {
            Ok(Some(reg)) => reg,
            Ok(None) => return common_response(StatusCode::NOT_FOUND, "Registration not found"),
            Err(e) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        };

        // Update status
        registration.update_status(data.status.clone(), data.reason);

        // Save updated registration
        match repository.update_registration(registration_id, registration.clone()).await {
            Ok(updated) => {
                let status_clone = updated.status.clone();
                let response = UpdateRegistrationStatusResponseDto {
                    id: extract_id(&updated.id),
                    status: updated.status,
                    updated_at: updated.updated_at,
                    message: format!("Registration status updated to {:?}", status_clone),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }

    // ============================================
    // Check-in Participant
    // ============================================
    pub async fn check_in_participant(&self, registration_id: &Thing) -> Response {
        let repository = RegistrationsRepository::new(self.state);

        // Get existing registration
        let mut registration = match repository.query_registration_by_id(registration_id).await {
            Ok(Some(reg)) => reg,
            Ok(None) => return common_response(StatusCode::NOT_FOUND, "Registration not found"),
            Err(e) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        };

        // Perform check-in
        if let Err(e) = registration.check_in() {
            return common_response(StatusCode::BAD_REQUEST, &e);
        }

        // Save updated registration
        match repository.update_registration(registration_id, registration.clone()).await {
            Ok(updated) => {
                let response = CheckInResponseDto {
                    id: extract_id(&updated.id),
                    user_fullname: None, // Would need to query user info
                    checked_in: updated.checked_in,
                    check_in_time: updated.check_in_time.unwrap_or_default(),
                    message: "Participant checked in successfully".to_string(),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }

    // ============================================
    // Get Registration Statistics
    // ============================================
    pub async fn get_registration_stats(&self, hackathon_id: &Thing) -> Response {
        let repository = RegistrationsRepository::new(self.state);

        match repository.query_registration_stats(hackathon_id).await {
            Ok(stats) => {
                let response = RegistrationStatsDto {
                    hackathon_id: stats.hackathon_id,
                    hackathon_name: stats.hackathon_name,
                    total_registrations: stats.total_registrations,
                    pending: stats.pending,
                    approved: stats.approved,
                    rejected: stats.rejected,
                    waitlisted: stats.waitlisted,
                    cancelled: stats.cancelled,
                    checked_in: stats.checked_in,
                    team_registrations: stats.team_registrations,
                    individual_registrations: stats.individual_registrations,
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e),
        }
    }
}
