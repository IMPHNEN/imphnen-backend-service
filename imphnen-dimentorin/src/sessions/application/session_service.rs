use std::sync::Arc;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;
use imphnen_utils::AppError;
use crate::sessions::domain::{SessionEntity, SessionRepository, SessionService};
use crate::sessions::infrastructure::http::dto::{
    AvailabilitySlotDto, BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionDetailDto, SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListItemDto,
    SessionListResponseDto, UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
};

pub struct SessionServiceImpl {
    repo: Arc<dyn SessionRepository>,
}

impl SessionServiceImpl {
    pub fn new(repo: Arc<dyn SessionRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn book_session(
        &self,
        mentor_id: String,
        user_id: String,
        dto: BookSessionRequestDto,
    ) -> Result<BookSessionResponseDto, AppError> {
        let scheduled_at = DateTime::parse_from_rfc3339(&dto.scheduled_at)
            .map_err(|e| AppError::BadRequestError(format!("Invalid scheduled_at format: {}", e)))?
            .with_timezone(&Utc);

        let mentor_uuid = Uuid::parse_str(&mentor_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

        let mentee_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid user ID: {}", e)))?;

        let entity = SessionEntity {
            id: Uuid::new_v4(),
            mentor_id: mentor_uuid,
            mentee_id: mentee_uuid,
            topic: dto.topic,
            description: dto.description,
            scheduled_at,
            duration_minutes: dto.duration_minutes.unwrap_or(60),
            meeting_link: None,
            session_type: dto.session_type.unwrap_or_else(|| "video_call".to_string()),
            status: "pending".to_string(),
            feedback: None,
            rating: None,
            feedback_submitted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created = self.repo.create(entity).await?;

        Ok(BookSessionResponseDto {
            id: created.id.to_string(),
            mentor_id: created.mentor_id.to_string(),
            mentee_id: created.mentee_id.to_string(),
            topic: created.topic,
            description: created.description,
            scheduled_at: created.scheduled_at.to_rfc3339(),
            duration_minutes: created.duration_minutes,
            session_type: created.session_type,
            status: created.status,
            created_at: created.created_at.to_rfc3339(),
        })
    }

    async fn get_mentor_sessions(
        &self,
        mentor_id: String,
        status_filter: Option<String>,
    ) -> Result<SessionListResponseDto, AppError> {
        let mentor_uuid = Uuid::parse_str(&mentor_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

        let count = self
            .repo
            .count_by_mentor(mentor_uuid, status_filter.clone())
            .await?;

        let sessions = self
            .repo
            .find_by_mentor_id(mentor_uuid, status_filter)
            .await?;

        let items: Vec<SessionListItemDto> = sessions
            .into_iter()
            .map(|s| SessionListItemDto {
                id: s.id.to_string(),
                mentor_id: s.mentor_id.to_string(),
                mentee_id: s.mentee_id.to_string(),
                mentee_fullname: None,
                mentee_email: None,
                topic: s.topic,
                scheduled_at: s.scheduled_at.to_rfc3339(),
                duration_minutes: s.duration_minutes,
                session_type: s.session_type,
                status: s.status,
                rating: s.rating,
                created_at: s.created_at.to_rfc3339(),
            })
            .collect();

        Ok(SessionListResponseDto {
            sessions: items,
            total: count,
        })
    }

    async fn get_user_sessions(
        &self,
        user_id: String,
        status_filter: Option<String>,
    ) -> Result<SessionListResponseDto, AppError> {
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid user ID: {}", e)))?;

        let count = self
            .repo
            .count_by_mentee(user_uuid, status_filter.clone())
            .await?;

        let sessions = self
            .repo
            .find_by_mentee_id(user_uuid, status_filter)
            .await?;

        let items: Vec<SessionListItemDto> = sessions
            .into_iter()
            .map(|s| SessionListItemDto {
                id: s.id.to_string(),
                mentor_id: s.mentor_id.to_string(),
                mentee_id: s.mentee_id.to_string(),
                mentee_fullname: None,
                mentee_email: None,
                topic: s.topic,
                scheduled_at: s.scheduled_at.to_rfc3339(),
                duration_minutes: s.duration_minutes,
                session_type: s.session_type,
                status: s.status,
                rating: s.rating,
                created_at: s.created_at.to_rfc3339(),
            })
            .collect();

        Ok(SessionListResponseDto {
            sessions: items,
            total: count,
        })
    }

    async fn get_mentor_availability(
        &self,
        mentor_id: String,
    ) -> Result<MentorAvailabilityDto, AppError> {
        let mentor_uuid = Uuid::parse_str(&mentor_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

        let booked_dates = self.repo.find_booked_dates(mentor_uuid).await?;

        let mut slots = Vec::new();
        let today = Utc::now().date_naive();

        for i in 0..7 {
            let date = today + Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();

            for hour in 9..17 {
                let time_str = format!("{:02}:00", hour);
                let datetime_prefix = format!("{}T{}", date_str, time_str);

                let is_booked = booked_dates
                    .iter()
                    .any(|d| d.starts_with(&datetime_prefix[..13]));

                slots.push(AvailabilitySlotDto {
                    date: date_str.clone(),
                    time: time_str,
                    available: !is_booked,
                });
            }
        }

        Ok(MentorAvailabilityDto {
            mentor_id,
            availability_commitment: "Available weekdays 9 AM - 5 PM".to_string(),
            preferred_formats: vec!["video_call".to_string(), "phone_call".to_string()],
            slots,
            booked_dates,
        })
    }

    async fn update_session_status(
        &self,
        session_id: String,
        _user_id: String,
        dto: UpdateSessionStatusRequestDto,
    ) -> Result<UpdateSessionStatusResponseDto, AppError> {
        let session_uuid = Uuid::parse_str(&session_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid session ID: {}", e)))?;

        let mut session = self
            .repo
            .find_by_id(session_uuid)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

        session.status = dto.status;
        if let Some(link) = dto.meeting_link {
            session.meeting_link = Some(link);
        }
        session.updated_at = Utc::now();

        let updated = self.repo.update(session_uuid, session).await?;

        Ok(UpdateSessionStatusResponseDto {
            id: updated.id.to_string(),
            status: updated.status,
            meeting_link: updated.meeting_link,
            updated_at: updated.updated_at.to_rfc3339(),
        })
    }

    async fn submit_feedback(
        &self,
        session_id: String,
        user_id: String,
        dto: SessionFeedbackRequestDto,
    ) -> Result<SessionFeedbackResponseDto, AppError> {
        let session_uuid = Uuid::parse_str(&session_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid session ID: {}", e)))?;

        let mut session = self
            .repo
            .find_by_id(session_uuid)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

        if session.mentee_id.to_string() != user_id {
            return Err(AppError::ForbiddenError(
                "Only the mentee can submit feedback".to_string(),
            ));
        }

        if session.status != "completed" {
            return Err(AppError::BadRequestError(
                "Feedback can only be submitted for completed sessions".to_string(),
            ));
        }

        session.feedback = Some(dto.feedback.clone());
        session.rating = Some(dto.rating);
        session.feedback_submitted_at = Some(Utc::now());
        session.updated_at = Utc::now();

        let updated = self.repo.update(session_uuid, session).await?;
        let submitted_at = updated
            .feedback_submitted_at
            .unwrap_or_else(Utc::now)
            .to_rfc3339();

        Ok(SessionFeedbackResponseDto {
            id: updated.id.to_string(),
            feedback: dto.feedback,
            rating: dto.rating,
            submitted_at,
        })
    }

    async fn get_session_detail(
        &self,
        session_id: String,
    ) -> Result<SessionDetailDto, AppError> {
        let session_uuid = Uuid::parse_str(&session_id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid session ID: {}", e)))?;

        let session = self
            .repo
            .find_by_id(session_uuid)
            .await?
            .ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

        Ok(SessionDetailDto {
            id: session.id.to_string(),
            mentor_id: session.mentor_id.to_string(),
            mentor_fullname: None,
            mentee_id: session.mentee_id.to_string(),
            mentee_fullname: None,
            topic: session.topic,
            description: session.description,
            scheduled_at: session.scheduled_at.to_rfc3339(),
            duration_minutes: session.duration_minutes,
            meeting_link: session.meeting_link,
            session_type: session.session_type,
            status: session.status,
            feedback: session.feedback,
            rating: session.rating,
            feedback_submitted_at: session.feedback_submitted_at.map(|dt| dt.to_rfc3339()),
            created_at: session.created_at.to_rfc3339(),
            updated_at: session.updated_at.to_rfc3339(),
        })
    }
}
