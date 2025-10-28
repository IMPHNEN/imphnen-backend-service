use std::pin::Pin;
use std::future::Future;
use serde::Deserialize;
// Type alias to shorten complex future return types used across the service trait
type ListServiceFut<T> = Pin<Box<dyn Future<Output = Result<imphnen_libs::ResponseListSuccessDto<Vec<T>>, ErrorDto>> + Send>>;
use super::hackathon_dto::{
    HackathonCreateRequestDto, HackathonDto, HackathonEventCreateRequestDto, HackathonEventDto,
    HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
    HackathonSubmissionDto, HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
    HackathonTimelineDto, HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
};
use super::hackathon_repository::HackathonRepository;
use super::hackathon_schema::SubmissionStatus;
use super::hackathon_audit_schema::{AuditAction, HackathonAuditLogSchema};
use super::hackathon_audit_repository::HackathonAuditRepository;
use super::hackathon_validation::{
    validate_dates, validate_organizers, validate_prizes,
};
use crate::{AppState, ResponseSuccessDto, ErrorDto};
use imphnen_utils::{validator::validate_request};
use imphnen_libs::{MetaRequestDto, ResponseListSuccessDto};
use axum::http::StatusCode;

use tracing::error;

// Helper function to check if optional string is non-empty
fn is_non_empty_string(opt: &Option<String>) -> bool {
    opt.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false)
}

pub trait HackathonServiceTrait: Send + Sync + 'static {
    // Hackathon operations
    fn create_hackathon(
        payload: HackathonCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>>;
    fn get_hackathon(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>>;
    fn list_hackathons(
        meta: MetaRequestDto,
        state: &AppState,
    ) -> ListServiceFut<HackathonDto>;
    fn update_hackathon(
        id: String,
        payload: HackathonUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>>;
    fn delete_hackathon(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>>;

    // Hackathon Events operations
    fn create_hackathon_event(
        hackathon_id: String,
        payload: HackathonEventCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonEventDto>, ErrorDto>> + Send>>;
    fn list_hackathon_events(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> ListServiceFut<HackathonEventDto>;
    fn update_hackathon_event(
        id: String,
        payload: HackathonEventUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonEventDto>, ErrorDto>> + Send>>;
    fn delete_hackathon_event(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>>;

    // Hackathon Timeline operations
    fn create_hackathon_timeline(
        hackathon_id: String,
        payload: HackathonTimelineCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonTimelineDto>, ErrorDto>> + Send>>;
    fn list_hackathon_timeline(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> ListServiceFut<HackathonTimelineDto>;
    fn update_hackathon_timeline(
        id: String,
        payload: HackathonTimelineUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonTimelineDto>, ErrorDto>> + Send>>;
    fn delete_hackathon_timeline(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>>;

    // Hackathon Submissions operations
    fn create_hackathon_submission(
        hackathon_id: String,
        team_id: String,
        payload: HackathonSubmissionCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn get_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn list_hackathon_submissions(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> ListServiceFut<HackathonSubmissionDto>;
    fn list_submissions_by_team(
        meta: MetaRequestDto,
        team_id: String,
        state: &AppState,
    ) -> ListServiceFut<HackathonSubmissionDto>;
    fn update_hackathon_submission(
        id: String,
        payload: HackathonSubmissionUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn submit_hackathon_submission(
        id: String,
        user_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn update_submission_status(
        id: String,
        status: SubmissionStatus,
        feedback: Option<String>,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn delete_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>>;

    // Participants
    fn register_participant(
        hackathon_id: String,
        payload: super::hackathon_dto::RegisterParticipantRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<super::hackathon_dto::HackathonParticipantDto>, ErrorDto>> + Send>>;

    fn list_participants(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> ListServiceFut<super::hackathon_dto::HackathonParticipantDto>;
}

#[derive(Clone)]
pub struct HackathonService;

impl HackathonServiceTrait for HackathonService {
    fn create_hackathon(
        payload: HackathonCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // 1. Validate request input
            if let Err((_, error_message)) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": error_message })),
                });
            }

            // 2. Validate dates consistency
            if let Err(e) = validate_dates(
                &payload.start_date,
                &payload.end_date,
                &payload.registration_deadline,
            ) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: e.to_string(),
                    details: None,
                });
            }

            // 3. Validate organizers
            if let Err(e) = validate_organizers(&payload.organizers) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: e.to_string(),
                    details: None,
                });
            }

            // 4. Validate prizes if provided
            if let Some(ref prizes) = payload.prizes {
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

            let repo = HackathonRepository::new(&state);
            let audit_repo = HackathonAuditRepository::new(&state);

            // 5. Create hackathon
            match repo.create_hackathon(payload.clone()).await {
                Ok(hackathon) => {
                    // 6. Create audit log
                    let audit_log = HackathonAuditLogSchema::new(
                        Some(hackathon.id.clone()),
                        AuditAction::HackathonCreated,
                        payload.organizers.first().unwrap_or(&"system".to_string()).clone(),
                        "hackathon".to_string(),
                        Some(hackathon.id.id.to_string()),
                    )
                    .with_changes(serde_json::to_value(&hackathon).unwrap_or_default());

                    if let Err(e) = audit_repo.log(audit_log).await {
                        tracing::error!("Failed to create audit log: {}", e);
                        // Don't fail the request if audit logging fails
                    }

                    let dto = HackathonDto::from(hackathon);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to create hackathon: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to create hackathon".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn get_hackathon(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.get_hackathon_by_id(id).await {
                Ok(hackathon) => {
                    let dto = HackathonDto::from(hackathon);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to get hackathon: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::NOT_FOUND.as_u16(),
                        message: "Hackathon not found".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn list_hackathons(
        meta: MetaRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonDto>>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.list_hackathons(meta).await {
                Ok(result) => {
                    let dtos: Vec<HackathonDto> = result.data.into_iter().map(HackathonDto::from).collect();
                    Ok(ResponseListSuccessDto {
                        data: dtos,
                        meta: result.meta,
                    })
                }
                Err(e) => {
                    error!("Failed to list hackathons: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to list hackathons".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn update_hackathon(
        id: String,
        payload: HackathonUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // 1. Validate request
            if let Err(errors) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": errors.1 })),
                });
            }

            let repo = HackathonRepository::new(&state);
            let audit_repo = HackathonAuditRepository::new(&state);

            // 2. Get existing hackathon for validation
            let existing = match repo.get_hackathon_by_id(id.clone()).await {
                Ok(h) => h,
                Err(_) => {
                    return Err(ErrorDto {
                        status: StatusCode::NOT_FOUND.as_u16(),
                        message: "Hackathon not found".to_string(),
                        details: None,
                    });
                }
            };

            // 3. Validate dates consistency
            let start_date = payload.start_date.unwrap_or(existing.start_date);
            let end_date = payload.end_date.unwrap_or(existing.end_date);
            let registration_deadline = payload.registration_deadline.unwrap_or(existing.registration_deadline);

            if let Err(e) = validate_dates(&start_date, &end_date, &registration_deadline) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: e.to_string(),
                    details: None,
                });
            }

            // 4. Validate organizers if being updated
            if let Some(ref organizers) = payload.organizers {
                if let Err(e) = validate_organizers(organizers) {
                    return Err(ErrorDto {
                        status: StatusCode::BAD_REQUEST.as_u16(),
                        message: e.to_string(),
                        details: None,
                    });
                }
            }

            // 5. Validate prizes if being updated
            if let Some(ref prizes) = payload.prizes {
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

            // 6. Store old value for audit log
            let old_value = serde_json::to_value(&existing).unwrap_or_default();

            // 7. Update hackathon
            match repo.update_hackathon(id.clone(), payload.clone()).await {
                Ok(hackathon) => {
                    // 8. Create audit log
                    let new_value = serde_json::to_value(&hackathon).unwrap_or_default();
                    let changes = serde_json::to_value(&payload).unwrap_or_default();

                    let audit_log = HackathonAuditLogSchema::new(
                        Some(hackathon.id.clone()),
                        AuditAction::HackathonUpdated,
                        existing.organizers.first().unwrap_or(&"system".to_string()).clone(),
                        "hackathon".to_string(),
                        Some(id),
                    )
                    .with_changes(changes)
                    .with_old_new_values(old_value, new_value);

                    if let Err(e) = audit_repo.log(audit_log).await {
                        tracing::error!("Failed to create audit log: {}", e);
                    }

                    let dto = HackathonDto::from(hackathon);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to update hackathon: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to update hackathon".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn delete_hackathon(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.delete_hackathon(id).await {
                Ok(message) => Ok(ResponseSuccessDto { data: message }),
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("Failed to delete") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Hackathon not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to delete hackathon: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to delete hackathon".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn create_hackathon_event(
        hackathon_id: String,
        payload: HackathonEventCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonEventDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err((_, error_message)) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": error_message })),
                });
            }

            // Business logic validation
            if payload.end_time <= payload.start_time {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "End time must be after start time".to_string(),
                    details: None,
                });
            }

            let repo = HackathonRepository::new(&state);

            // Verify hackathon exists
            if repo.get_hackathon_by_id(hackathon_id.clone()).await.is_err() {
                return Err(ErrorDto {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: "Hackathon not found".to_string(),
                    details: None,
                });
            }

            match repo.create_hackathon_event(hackathon_id, payload).await {
                Ok(event) => {
                    let dto = HackathonEventDto::from(event);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to create hackathon event: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to create hackathon event".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn list_hackathon_events(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonEventDto>>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.list_hackathon_events(meta, hackathon_id).await {
                Ok(result) => {
                    let dtos: Vec<HackathonEventDto> = result.data.into_iter().map(HackathonEventDto::from).collect();
                    Ok(ResponseListSuccessDto {
                        data: dtos,
                        meta: result.meta,
                    })
                }
                Err(e) => {
                    error!("Failed to list hackathon events: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to list hackathon events".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn update_hackathon_event(
        id: String,
        payload: HackathonEventUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonEventDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err((_, error_message)) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": error_message })),
                });
            }

            let repo = HackathonRepository::new(&state);

            match repo.update_hackathon_event(id, payload).await {
                Ok(event) => {
                    let dto = HackathonEventDto::from(event);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("not found") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Event not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to update hackathon event: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to update hackathon event".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn delete_hackathon_event(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.delete_hackathon_event(id).await {
                Ok(message) => Ok(ResponseSuccessDto { data: message }),
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("Failed to delete") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Event not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to delete hackathon event: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to delete hackathon event".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn create_hackathon_timeline(
        hackathon_id: String,
        payload: HackathonTimelineCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonTimelineDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err(errors) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": errors.1 })),
                });
            }

            // Business logic validation
            if payload.end_date <= payload.start_date {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "End date must be after start date".to_string(),
                    details: None,
                });
            }

            let repo = HackathonRepository::new(&state);

            // Verify hackathon exists
            if repo.get_hackathon_by_id(hackathon_id.clone()).await.is_err() {
                return Err(ErrorDto {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: "Hackathon not found".to_string(),
                    details: None,
                });
            }

            match repo.create_hackathon_timeline(hackathon_id, payload).await {
                Ok(timeline) => {
                    let dto = HackathonTimelineDto::from(timeline);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to create hackathon timeline: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to create hackathon timeline".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn list_hackathon_timeline(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonTimelineDto>>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.list_hackathon_timeline(meta, hackathon_id).await {
                Ok(result) => {
                    let dtos: Vec<HackathonTimelineDto> = result.data.into_iter().map(HackathonTimelineDto::from).collect();
                    Ok(ResponseListSuccessDto {
                        data: dtos,
                        meta: result.meta,
                    })
                }
                Err(e) => {
                    error!("Failed to list hackathon timeline: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to list hackathon timeline".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn update_hackathon_timeline(
        id: String,
        payload: HackathonTimelineUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonTimelineDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err(errors) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": errors.1 })),
                });
            }

            let repo = HackathonRepository::new(&state);

            match repo.update_hackathon_timeline(id, payload).await {
                Ok(timeline) => {
                    let dto = HackathonTimelineDto::from(timeline);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("not found") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Timeline not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to update hackathon timeline: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to update hackathon timeline".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn delete_hackathon_timeline(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.delete_hackathon_timeline(id).await {
                Ok(message) => Ok(ResponseSuccessDto { data: message }),
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("Failed to delete") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Timeline not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to delete hackathon timeline: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to delete hackathon timeline".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn create_hackathon_submission(
        hackathon_id: String,
        team_id: String,
        payload: HackathonSubmissionCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err(errors) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": errors.1 })),
                });
            }

            let repo = HackathonRepository::new(&state);

            // Verify hackathon exists
            if repo.get_hackathon_by_id(hackathon_id.clone()).await.is_err() {
                return Err(ErrorDto {
                    status: StatusCode::NOT_FOUND.as_u16(),
                    message: "Hackathon not found".to_string(),
                    details: None,
                });
            }

            match repo.create_hackathon_submission(hackathon_id, team_id, payload).await {
                Ok(submission) => {
                    let dto = HackathonSubmissionDto::from(submission);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to create hackathon submission: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to create hackathon submission".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn get_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.get_hackathon_submission_by_id(id).await {
                Ok(submission) => {
                    let dto = HackathonSubmissionDto::from(submission);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    error!("Failed to get hackathon submission: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::NOT_FOUND.as_u16(),
                        message: "Submission not found".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

    fn list_hackathon_submissions(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonSubmissionDto>>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.list_hackathon_submissions(meta, hackathon_id).await {
                Ok(result) => {
                    let dtos: Vec<HackathonSubmissionDto> = result.data.into_iter().map(HackathonSubmissionDto::from).collect();
                    Ok(ResponseListSuccessDto {
                        data: dtos,
                        meta: result.meta,
                    })
                }
                Err(e) => {
                    error!("Failed to list hackathon submissions: {}", e);
                    Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to list hackathon submissions".to_string(),
                        details: None,
                    })
                }
            }
        })
    }

        fn list_submissions_by_team(
            meta: MetaRequestDto,
            team_id: String,
            state: &AppState,
        ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonSubmissionDto>>, ErrorDto>> + Send>> {
            let state = state.to_owned();
            Box::pin(async move {
                let repo = HackathonRepository::new(&state);

                match repo.list_submissions_by_team(meta, team_id).await {
                    Ok(result) => {
                        let dtos: Vec<HackathonSubmissionDto> = result.data.into_iter().map(HackathonSubmissionDto::from).collect();
                        Ok(ResponseListSuccessDto {
                            data: dtos,
                            meta: result.meta,
                        })
                    }
                    Err(e) => {
                        error!("Failed to list submissions by team: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to list submissions".to_string(),
                            details: None,
                        })
                    }
                }
            })
        }

        fn update_submission_status(
            id: String,
            status: SubmissionStatus,
            feedback: Option<String>,
            state: &AppState,
        ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
            let state = state.to_owned();
            Box::pin(async move {
                let repo = HackathonRepository::new(&state);

                match repo.update_submission_status(id, status, feedback).await {
                    Ok(submission) => {
                        let dto = HackathonSubmissionDto::from(submission);
                        Ok(ResponseSuccessDto { data: dto })
                    }
                    Err(e) => {
                        let msg = e.to_string();
                        if msg.contains("not found") {
                            Err(ErrorDto { status: StatusCode::NOT_FOUND.as_u16(), message: "Submission not found".to_string(), details: None })
                        } else {
                            error!("Failed to update submission status: {}", e);
                            Err(ErrorDto { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: "Failed to update submission status".to_string(), details: None })
                        }
                    }
                }
            })
        }

    fn update_hackathon_submission(
        id: String,
        payload: HackathonSubmissionUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        
        let state = state.to_owned();
        Box::pin(async move {
            // Validate request
            if let Err(errors) = validate_request(&payload) {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Validation failed".to_string(),
                    details: Some(serde_json::json!({ "validation_errors": errors.1 })),
                });
            }

            let repo = HackathonRepository::new(&state);

            match repo.update_hackathon_submission(id, payload).await {
                Ok(submission) => {
                    let dto = HackathonSubmissionDto::from(submission);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("not found") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Submission not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to update hackathon submission: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to update hackathon submission".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn submit_hackathon_submission(
        id: String,
        user_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            // Get submission to extract hackathon_id for timeline validation and team_id for leader check
            let submission = match repo.get_hackathon_submission_by_id(id.clone()).await {
                Ok(sub) => sub,
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("not found") {
                        return Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Submission not found".to_string(),
                            details: None,
                        });
                    } else {
                        error!("Failed to get submission for validation: {}", e);
                        return Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to validate submission".to_string(),
                            details: None,
                        });
                    }
                }
            };

            // VALIDATION 1: Check if user is the team leader
            if let Some(team_id_thing) = &submission.team_id {
                let team_id = team_id_thing.id.to_raw();
                
                // Use parameterized query to prevent SQL injection
                match state.surrealdb_ws
                    .query("SELECT leader_id FROM type::table($table) WHERE id = type::thing($table, $team_id)")
                    .bind(("table", "app_teams"))
                    .bind(("team_id", team_id.clone()))
                    .await 
                {
                    Ok(mut result) => {
                        #[derive(Debug, Deserialize)]
                        struct TeamLeader {
                            leader_id: surrealdb::sql::Thing,
                        }
                        
                        let team: Option<TeamLeader> = result.take(0).ok().flatten();
                        if let Some(team) = team {
                            let leader_id = team.leader_id.id.to_raw();
                            if leader_id != user_id {
                                return Err(ErrorDto {
                                    status: StatusCode::FORBIDDEN.as_u16(),
                                    message: "Only team leader can submit the project".to_string(),
                                    details: Some(serde_json::json!({
                                        "team_leader_id": leader_id,
                                        "your_user_id": user_id
                                    })),
                                });
                            }
                        } else {
                            return Err(ErrorDto {
                                status: StatusCode::NOT_FOUND.as_u16(),
                                message: "Team not found".to_string(),
                                details: None,
                            });
                        }
                    }
                    Err(e) => {
                        error!("Failed to get team information: {}", e);
                        return Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to verify team leader".to_string(),
                            details: None,
                        });
                    }
                }
            } else {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Submission has no associated team".to_string(),
                    details: None,
                });
            }

            // VALIDATION 2: Must have repository_url OR upload_file_url
            let has_repo = is_non_empty_string(&submission.repository_url);
            let has_upload = is_non_empty_string(&submission.upload_file_url);
            
            if !has_repo && !has_upload {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Submission must include either repository URL or uploaded file (zip/pdf)".to_string(),
                    details: Some(serde_json::json!({
                        "required": "repository_url OR upload_file_url"
                    })),
                });
            }

            // VALIDATION 3: Must have at least one social media contact
            let has_contact = [
                &submission.contact_instagram,
                &submission.contact_twitter,
                &submission.contact_linkedin,
                &submission.contact_facebook,
                &submission.contact_youtube,
                &submission.contact_tiktok,
                &submission.contact_other,
            ].iter().any(|contact| is_non_empty_string(contact));

            if !has_contact {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Submission must include at least one social media contact for demo".to_string(),
                    details: Some(serde_json::json!({
                        "required": "At least one of: contact_instagram, contact_twitter, contact_linkedin, contact_facebook, contact_youtube, contact_tiktok, contact_other"
                    })),
                });
            }

            // Check submission timeline phase
            match repo.get_submission_timeline_phase(submission.hackathon_id.id.to_raw()).await {
                Ok(Some(timeline_phase)) => {
                    let current_time = chrono::Utc::now();
                    if current_time < timeline_phase.start_date || current_time > timeline_phase.end_date {
                        return Err(ErrorDto {
                            status: StatusCode::BAD_REQUEST.as_u16(),
                            message: "Submission is not allowed outside the designated submission period".to_string(),
                            details: Some(serde_json::json!({
                                "start_date": timeline_phase.start_date,
                                "end_date": timeline_phase.end_date,
                                "current_time": current_time
                            })),
                        });
                    }
                }
                Ok(None) => {
                    // If no timeline phase defined, allow submission (backward compatibility)
                }
                Err(e) => {
                    error!("Failed to get submission timeline phase: {}", e);
                    return Err(ErrorDto {
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        message: "Failed to validate submission period".to_string(),
                        details: None,
                    });
                }
            }

            match repo.submit_hackathon_submission(id).await {
                Ok(submission) => {
                    let dto = HackathonSubmissionDto::from(submission);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("not found") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Submission not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to submit hackathon submission: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to submit hackathon submission".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn delete_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.delete_hackathon_submission(id).await {
                Ok(message) => Ok(ResponseSuccessDto { data: message }),
                Err(e) => {
                    let error_msg = e.to_string();
                    if error_msg.contains("Failed to delete") {
                        Err(ErrorDto {
                            status: StatusCode::NOT_FOUND.as_u16(),
                            message: "Submission not found".to_string(),
                            details: None,
                        })
                    } else {
                        error!("Failed to delete hackathon submission: {}", e);
                        Err(ErrorDto {
                            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            message: "Failed to delete hackathon submission".to_string(),
                            details: None,
                        })
                    }
                }
            }
        })
    }

    fn register_participant(
        hackathon_id: String,
        payload: super::hackathon_dto::RegisterParticipantRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<super::hackathon_dto::HackathonParticipantDto>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            // Validate
            if let Err((_, errors)) = imphnen_utils::validator::validate_request(&payload) {
                return Err(ErrorDto { status: StatusCode::BAD_REQUEST.as_u16(), message: "Validation failed".to_string(), details: Some(serde_json::json!({ "validation_errors": errors })) });
            }

            let repo = HackathonRepository::new(&state);

            // ensure hackathon exists
            if repo.get_hackathon_by_id(hackathon_id.clone()).await.is_err() {
                return Err(ErrorDto { status: StatusCode::NOT_FOUND.as_u16(), message: "Hackathon not found".to_string(), details: None });
            }

            match repo.create_hackathon_participant(hackathon_id, payload.user_id).await {
                Ok(schema) => {
                    let dto = super::hackathon_dto::HackathonParticipantDto::from(schema);
                    Ok(ResponseSuccessDto { data: dto })
                }
                Err(e) => {
                    tracing::error!("Failed to register participant: {}", e);
                    Err(ErrorDto { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: "Failed to register participant".to_string(), details: None })
                }
            }
        })
    }

    fn list_participants(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> ListServiceFut<super::hackathon_dto::HackathonParticipantDto> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            match repo.list_hackathon_participants(meta, hackathon_id).await {
                Ok(result) => {
                    let dtos: Vec<super::hackathon_dto::HackathonParticipantDto> = result.data.into_iter().map(super::hackathon_dto::HackathonParticipantDto::from).collect();
                    Ok(ResponseListSuccessDto { data: dtos, meta: result.meta })
                }
                Err(e) => {
                    tracing::error!("Failed to list participants: {}", e);
                    Err(ErrorDto { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: "Failed to list participants".to_string(), details: None })
                }
            }
        })
    }
}