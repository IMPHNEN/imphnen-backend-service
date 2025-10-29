use super::{
    AvailabilitySlotDto, BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListItemDto,
    SessionListResponseDto, SessionSchema, SessionsRepository, UpdateSessionStatusRequestDto,
    UpdateSessionStatusResponseDto,
};
use axum::{http::StatusCode, response::Response};
use chrono::{Duration, Utc};
use imphnen_entities::ResponseSuccessDto;
use imphnen_libs::AppState;
use imphnen_utils::{common_response, extract_id, get_iso_date, make_thing, success_response, validate_request};

pub struct SessionsService;

impl SessionsService {
    // ============================================
    // Book Session
    // ============================================
    pub async fn book_session(
        state: &AppState,
        mentor_id: String,
        user_id: String,
        dto: BookSessionRequestDto,
    ) -> Response {
        if let Err((status, message)) = validate_request(&dto) {
            return common_response(status, &message);
        }

        let mentor_thing = make_thing("mentors", &mentor_id);
        let mentee_thing = make_thing("users", &user_id);

        let schema = SessionSchema::from_book_request(mentor_thing.clone(), mentee_thing.clone(), dto);
        
        let repo = SessionsRepository::new(state);
        match repo.create_session(schema).await {
            Ok(created) => {
                let response = BookSessionResponseDto {
                    id: extract_id(&created.id),
                    mentor_id: extract_id(&created.mentor_id),
                    mentee_id: extract_id(&created.mentee_id),
                    topic: created.topic,
                    description: created.description,
                    scheduled_at: created.scheduled_at,
                    duration_minutes: created.duration_minutes,
                    session_type: created.session_type,
                    status: created.status,
                    created_at: created.created_at,
                };
                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
        }
    }

    // ============================================
    // Get Mentor's Sessions
    // ============================================
    pub async fn get_mentor_sessions(
        state: &AppState,
        mentor_id: String,
        _user_email: String,
        status_filter: Option<String>,
    ) -> Response {
        let mentor_thing = make_thing("mentors", &mentor_id);
        
        let repo = SessionsRepository::new(state);
        
        // Get count and sessions
        let count = match repo.count_mentor_sessions(&mentor_thing, status_filter.clone()).await {
            Ok(c) => c,
            Err(e) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to count sessions: {}", e)),
        };
        
        match repo.query_mentor_sessions(&mentor_thing, status_filter).await {
            Ok(sessions) => {
                let session_items: Vec<SessionListItemDto> = sessions
                    .into_iter()
                    .map(|s| SessionListItemDto {
                        id: s.id,
                        mentor_id: s.mentor_id,
                        mentee_id: s.mentee_id,
                        mentee_fullname: s.mentee_fullname,
                        mentee_email: s.mentee_email,
                        topic: s.topic,
                        scheduled_at: s.scheduled_at,
                        duration_minutes: s.duration_minutes,
                        session_type: s.session_type,
                        status: s.status,
                        rating: s.rating,
                        created_at: s.created_at,
                    })
                    .collect();

                let response = SessionListResponseDto {
                    sessions: session_items,
                    total: count,
                };
                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
        }
    }

    // ============================================
    // Get User's Sessions (as mentee)
    // ============================================
    pub async fn get_user_sessions(
        state: &AppState,
        user_id: String,
        status_filter: Option<String>,
    ) -> Response {
        let user_thing = make_thing("users", &user_id);
        
        let repo = SessionsRepository::new(state);
        
        // Get count and sessions
        let count = match repo.count_user_sessions(&user_thing, status_filter.clone()).await {
            Ok(c) => c,
            Err(e) => return common_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to count sessions: {}", e)),
        };
        
        match repo.query_user_sessions(&user_thing, status_filter).await {
            Ok(sessions) => {
                let session_items: Vec<SessionListItemDto> = sessions
                    .into_iter()
                    .map(|s| SessionListItemDto {
                        id: s.id,
                        mentor_id: s.mentor_id,
                        mentee_id: s.mentee_id,
                        mentee_fullname: s.mentee_fullname,
                        mentee_email: s.mentee_email,
                        topic: s.topic,
                        scheduled_at: s.scheduled_at,
                        duration_minutes: s.duration_minutes,
                        session_type: s.session_type,
                        status: s.status,
                        rating: s.rating,
                        created_at: s.created_at,
                    })
                    .collect();

                let response = SessionListResponseDto {
                    sessions: session_items,
                    total: count,
                };
                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
        }
    }

    // ============================================
    // Get Mentor Availability
    // ============================================
    pub async fn get_mentor_availability(state: &AppState, mentor_id: String) -> Response {
        let mentor_thing = make_thing("mentors", &mentor_id);

        let repo = SessionsRepository::new(state);
        match repo.query_booked_dates(&mentor_thing).await {
            Ok(booked_dates) => {
                // Generate sample availability slots (next 7 days)
                let mut slots = Vec::new();
                let today = Utc::now().date_naive();

                for i in 0..7 {
                    let date = today + Duration::days(i);
                    let date_str = date.format("%Y-%m-%d").to_string();

                    // Generate time slots (9 AM to 5 PM, every hour)
                    for hour in 9..17 {
                        let time_str = format!("{:02}:00", hour);
                        let datetime_str = format!("{}T{}:00Z", date_str, time_str);

                        // Check if this slot is booked
                        let is_booked = booked_dates.iter().any(|d| d.starts_with(&datetime_str[..13]));

                        slots.push(AvailabilitySlotDto {
                            date: date_str.clone(),
                            time: time_str,
                            available: !is_booked,
                        });
                    }
                }

                let response = MentorAvailabilityDto {
                    mentor_id,
                    availability_commitment: "Available weekdays 9 AM - 5 PM".to_string(),
                    preferred_formats: vec!["video_call".to_string(), "phone_call".to_string()],
                    slots,
                    booked_dates,
                };
                success_response(ResponseSuccessDto { data: response })
            }
            Err(e) => common_response(StatusCode::NOT_FOUND, &e),
        }
    }

    // ============================================
    // Update Session Status
    // ============================================
    pub async fn update_session_status(
        state: &AppState,
        session_id: String,
        _user_id: String,
        dto: UpdateSessionStatusRequestDto,
    ) -> Response {
        if let Err((status, message)) = validate_request(&dto) {
            return common_response(status, &message);
        }

        let session_thing = make_thing("sessions", &session_id);
        
        let repo = SessionsRepository::new(state);
        match repo.query_session_by_id(&session_thing).await {
            Ok(Some(mut session)) => {
                session.update_status(dto.clone());
                match repo.update_session(&session_thing, session).await {
                    Ok(updated) => {
                        let response = UpdateSessionStatusResponseDto {
                            id: extract_id(&updated.id),
                            status: updated.status,
                            meeting_link: updated.meeting_link,
                            updated_at: updated.updated_at,
                        };
                        success_response(ResponseSuccessDto { data: response })
                    }
                    Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
                }
            }
            Ok(None) => common_response(StatusCode::NOT_FOUND, "Session not found"),
            Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
        }
    }

    // ============================================
    // Submit Feedback
    // ============================================
    pub async fn submit_feedback(
        state: &AppState,
        session_id: String,
        user_id: String,
        dto: SessionFeedbackRequestDto,
    ) -> Response {
        if let Err((status, message)) = validate_request(&dto) {
            return common_response(status, &message);
        }

        let session_thing = make_thing("sessions", &session_id);
        
        let repo = SessionsRepository::new(state);
        match repo.query_session_by_id(&session_thing).await {
            Ok(Some(mut session)) => {
                // Authorization: Only mentee can submit feedback
                let mentee_id = extract_id(&session.mentee_id);
                if mentee_id != user_id {
                    return common_response(
                        StatusCode::FORBIDDEN,
                        "Unauthorized: Only the mentee can submit feedback",
                    );
                }

                // Validate session is completed
                if session.status != "completed" {
                    return common_response(
                        StatusCode::BAD_REQUEST,
                        "Feedback can only be submitted for completed sessions",
                    );
                }

                session.add_feedback(dto.clone());
                match repo.update_session(&session_thing, session).await {
                    Ok(updated) => {
                        let response = SessionFeedbackResponseDto {
                            id: extract_id(&updated.id),
                            feedback: dto.feedback,
                            rating: dto.rating,
                            submitted_at: updated.feedback_submitted_at.unwrap_or_else(get_iso_date),
                        };
                        success_response(ResponseSuccessDto { data: response })
                    }
                    Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
                }
            }
            Ok(None) => common_response(StatusCode::NOT_FOUND, "Session not found"),
            Err(e) => common_response(StatusCode::BAD_REQUEST, &e),
        }
    }
}
