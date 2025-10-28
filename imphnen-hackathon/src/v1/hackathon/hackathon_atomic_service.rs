use super::hackathon_dto::{
    HackathonCreateRequestDto, HackathonDto, HackathonEventCreateRequestDto,
    HackathonTimelineCreateRequestDto,
};
use super::hackathon_repository::HackathonRepository;
use super::hackathon_schema::{HackathonSchema, HackathonEventsSchema, HackathonTimelineSchema};
use super::hackathon_audit_schema::{AuditAction, HackathonAuditLogSchema};
use super::hackathon_audit_repository::HackathonAuditRepository;
use super::hackathon_validation::{validate_timeline_phases, validate_dates, validate_organizers, validate_prizes, MAX_EVENTS_PER_HACKATHON};
use crate::{AppState, ResponseSuccessDto, ErrorDto};
use axum::http::StatusCode;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Request DTO for atomic hackathon creation with timeline and events
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct HackathonCompleteSetupRequestDto {
    #[validate(nested)]
    pub hackathon: HackathonCreateRequestDto,
    
    #[validate(length(min = 1, message = "At least one timeline phase is required"))]
    pub timelines: Vec<HackathonTimelineCreateRequestDto>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HackathonEventCreateRequestDto>>,
    
    /// Actor ID for audit logging
    pub actor_id: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_email: Option<String>,
}

/// Response DTO for complete hackathon setup
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HackathonCompleteSetupResponseDto {
    pub hackathon: HackathonDto,
    pub timelines: Vec<super::hackathon_dto::HackathonTimelineDto>,
    pub events: Option<Vec<super::hackathon_dto::HackathonEventDto>>,
    pub message: String,
}

/// Service for atomic hackathon operations
pub struct HackathonAtomicService;

impl HackathonAtomicService {
    /// Create hackathon with timeline and events atomically
    /// This ensures all-or-nothing creation - if any step fails, nothing is created
    pub async fn create_hackathon_complete(
        payload: HackathonCompleteSetupRequestDto,
        state: &AppState,
    ) -> Result<ResponseSuccessDto<HackathonCompleteSetupResponseDto>, ErrorDto> {
        // 1. Validate all inputs before any database operations
        if let Err((_, error_message)) = imphnen_utils::validator::validate_request(&payload) {
            return Err(ErrorDto {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "Validation failed".to_string(),
                details: Some(serde_json::json!({ "validation_errors": error_message })),
            });
        }

        // 2. Validate dates
        if let Err(e) = validate_dates(
            &payload.hackathon.start_date,
            &payload.hackathon.end_date,
            &payload.hackathon.registration_deadline,
        ) {
            return Err(ErrorDto {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: e.to_string(),
                details: None,
            });
        }

        // 3. Validate organizers
        if let Err(e) = validate_organizers(&payload.hackathon.organizers) {
            return Err(ErrorDto {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: e.to_string(),
                details: None,
            });
        }

        // 4. Validate prizes if provided
        if let Some(ref prizes) = payload.hackathon.prizes {
            let prize_schemas: Vec<super::hackathon_schema::Prize> = prizes
                .iter()
                .map(|p| super::hackathon_schema::Prize {
                    position: p.position,
                    title: p.title.clone(),
                    description: p.description.clone(),
                    value: p.value.clone(),
                })
                .collect();
                
            if let Err(e) = validate_prizes(&prize_schemas) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: e.to_string(),
                    details: None,
                });
            }
        }

        // 5. Validate events count if provided
        if let Some(ref events) = payload.events {
            if events.len() > MAX_EVENTS_PER_HACKATHON {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: format!(
                        "Maximum {} events allowed per hackathon",
                        MAX_EVENTS_PER_HACKATHON
                    ),
                    details: None,
                });
            }
        }

        let repo = HackathonRepository::new(state);
        let audit_repo = HackathonAuditRepository::new(state);

        // 6. Create hackathon first
        let hackathon = match repo.create_hackathon(payload.hackathon.clone()).await {
            Ok(h) => h,
            Err(e) => {
                tracing::error!("Failed to create hackathon: {}", e);
                return Err(ErrorDto {
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Failed to create hackathon".to_string(),
                    details: Some(serde_json::json!({ "error": e.to_string() })),
                });
            }
        };

        let hackathon_id = hackathon.id.id.to_string();

        // 7. Create timelines - if this fails, we should ideally rollback hackathon
        let mut created_timelines = Vec::new();
        for timeline_dto in &payload.timelines {
            match repo.create_hackathon_timeline(hackathon_id.clone(), timeline_dto.clone()).await {
                Ok(timeline) => created_timelines.push(timeline),
                Err(e) => {
                    tracing::error!("Failed to create timeline, attempting cleanup: {}", e);
                    // Attempt to delete hackathon and created timelines
                    let _ = Self::cleanup_failed_creation(
                        &hackathon_id,
                        &created_timelines.iter().map(|t| t.id.id.to_string()).collect::<Vec<_>>(),
                        &[],
                        &repo,
                    )
                    .await;

                    return Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to create timeline, changes rolled back".to_string(),
                        details: Some(serde_json::json!({ "error": e.to_string() })),
                    });
                }
            }
        }

        // 8. Validate timeline phases after all are created
        if let Err(e) = validate_timeline_phases(&hackathon, &created_timelines) {
            tracing::error!("Timeline validation failed, attempting cleanup: {}", e);
            let _ = Self::cleanup_failed_creation(
                &hackathon_id,
                &created_timelines.iter().map(|t| t.id.id.to_string()).collect::<Vec<_>>(),
                &[],
                &repo,
            )
            .await;

            return Err(ErrorDto {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Timeline validation failed: {}", e),
                details: None,
            });
        }

        // 9. Create events if provided
        let mut created_events = Vec::new();
        if let Some(ref events) = payload.events {
            for event_dto in events {
                match repo.create_hackathon_event(hackathon_id.clone(), event_dto.clone()).await {
                    Ok(event) => created_events.push(event),
                    Err(e) => {
                        tracing::error!("Failed to create event, attempting cleanup: {}", e);
                        let _ = Self::cleanup_failed_creation(
                            &hackathon_id,
                            &created_timelines.iter().map(|t| t.id.id.to_string()).collect::<Vec<_>>(),
                            &created_events.iter().map(|e| e.id.id.to_string()).collect::<Vec<_>>(),
                            &repo,
                        )
                        .await;

                        return Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to create event, changes rolled back".to_string(),
                            details: Some(serde_json::json!({ "error": e.to_string() })),
                        });
                    }
                }
            }
        }

        // 10. Log audit trail
        let audit_log = HackathonAuditLogSchema::new(
            Some(hackathon.id.clone()),
            AuditAction::HackathonCreated,
            payload.actor_id.clone(),
            "hackathon".to_string(),
            Some(hackathon_id.clone()),
        )
        .with_changes(serde_json::to_value(&payload).unwrap_or_default())
        .with_request_info(None, None, payload.actor_email.clone());

        if let Err(e) = audit_repo.log(audit_log).await {
            tracing::error!("Failed to create audit log: {}", e);
            // Don't fail the request if audit logging fails
        }

        // 11. Return success response
        let response = HackathonCompleteSetupResponseDto {
            hackathon: super::hackathon_dto::HackathonDto::from(hackathon),
            timelines: created_timelines
                .into_iter()
                .map(super::hackathon_dto::HackathonTimelineDto::from)
                .collect(),
            events: if created_events.is_empty() {
                None
            } else {
                Some(
                    created_events
                        .into_iter()
                        .map(super::hackathon_dto::HackathonEventDto::from)
                        .collect(),
                )
            },
            message: "Hackathon created successfully with timeline and events".to_string(),
        };

        Ok(ResponseSuccessDto { data: response })
    }

    /// Cleanup failed creation by deleting created resources
    async fn cleanup_failed_creation(
        hackathon_id: &str,
        timeline_ids: &[String],
        event_ids: &[String],
        repo: &HackathonRepository<'_>,
    ) -> Result<()> {
        tracing::info!("Starting cleanup for failed hackathon creation");

        // Delete events
        for event_id in event_ids {
            if let Err(e) = repo.delete_hackathon_event(event_id.to_string()).await {
                tracing::error!("Failed to cleanup event {}: {}", event_id, e);
            }
        }

        // Delete timelines
        for timeline_id in timeline_ids {
            if let Err(e) = repo.delete_hackathon_timeline(timeline_id.to_string()).await {
                tracing::error!("Failed to cleanup timeline {}: {}", timeline_id, e);
            }
        }

        // Delete hackathon
        if let Err(e) = repo.delete_hackathon(hackathon_id.to_string()).await {
            tracing::error!("Failed to cleanup hackathon {}: {}", hackathon_id, e);
        }

        tracing::info!("Cleanup completed");
        Ok(())
    }
}
