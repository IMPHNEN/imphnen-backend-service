use std::pin::Pin;
use std::future::Future;
use super::hackathon_dto::{
    HackathonCreateRequestDto, HackathonDto, HackathonEventCreateRequestDto, HackathonEventDto,
    HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
    HackathonSubmissionDto, HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
    HackathonTimelineDto, HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
};
use super::hackathon_repository::HackathonRepository;
use crate::{AppState, ResponseSuccessDto, ErrorDto};
use imphnen_utils::{validator::validate_request};
use imphnen_libs::{MetaRequestDto, ResponseListSuccessDto};
use axum::http::StatusCode;

use tracing::error;

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
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonDto>>, ErrorDto>> + Send>>;
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
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonEventDto>>, ErrorDto>> + Send>>;
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
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonTimelineDto>>, ErrorDto>> + Send>>;
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
    fn list_hackathon_submissions(
        meta: MetaRequestDto,
        hackathon_id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseListSuccessDto<Vec<HackathonSubmissionDto>>, ErrorDto>> + Send>>;
    fn update_hackathon_submission(
        id: String,
        payload: HackathonSubmissionUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn submit_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>>;
    fn delete_hackathon_submission(
        id: String,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<String>, ErrorDto>> + Send>>;
}

#[derive(Clone)]
pub struct HackathonService;

impl HackathonServiceTrait for HackathonService {
    fn create_hackathon(
        payload: HackathonCreateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonDto>, ErrorDto>> + Send>> {
        let payload = payload;
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
            if payload.end_date <= payload.start_date {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "End date must be after start date".to_string(),
                    details: None,
                });
            }

            if payload.registration_deadline >= payload.start_date {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Registration deadline must be before start date".to_string(),
                    details: None,
                });
            }

            if payload.organizers.is_empty() {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "At least one organizer is required".to_string(),
                    details: None,
                });
            }

            let repo = HackathonRepository::new(&state);

            match repo.create_hackathon(payload).await {
                Ok(hackathon) => {
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
        let payload = payload;
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

            // Get existing hackathon for validation
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

            // Business logic validation
            let start_date = payload.start_date.unwrap_or(existing.start_date);
            let end_date = payload.end_date.unwrap_or(existing.end_date);
            let registration_deadline = payload.registration_deadline.unwrap_or(existing.registration_deadline);

            if end_date <= start_date {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "End date must be after start date".to_string(),
                    details: None,
                });
            }

            if registration_deadline >= start_date {
                return Err(ErrorDto {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "Registration deadline must be before start date".to_string(),
                    details: None,
                });
            }

            match repo.update_hackathon(id, payload).await {
                Ok(hackathon) => {
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
        let payload = payload;
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
        let payload = payload;
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
        let payload = payload;
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
        let payload = payload;
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
        let payload = payload;
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

    fn update_hackathon_submission(
        id: String,
        payload: HackathonSubmissionUpdateRequestDto,
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        let payload = payload;
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
        state: &AppState,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseSuccessDto<HackathonSubmissionDto>, ErrorDto>> + Send>> {
        let state = state.to_owned();
        Box::pin(async move {
            let repo = HackathonRepository::new(&state);

            // Get submission to extract hackathon_id for timeline validation
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
}