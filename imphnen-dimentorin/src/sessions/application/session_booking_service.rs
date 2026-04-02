use crate::sessions::domain::{
	BookSessionCommand, BookedSession, SessionEntity, SessionFeedbackCommand,
	SessionFeedbackResult, SessionRepository, UpdateSessionStatusCommand,
	UpdatedSessionStatus,
};
use chrono::{DateTime, Utc};
use imphnen_utils::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct SessionBookingService {
	pub repo: Arc<dyn SessionRepository>,
}

impl SessionBookingService {
	pub async fn book_session(
		&self,
		mentor_id: String,
		user_id: String,
		cmd: BookSessionCommand,
	) -> Result<BookedSession, AppError> {
		let scheduled_at = DateTime::parse_from_rfc3339(&cmd.scheduled_at)
			.map_err(|e| {
				AppError::BadRequestError(format!("Invalid scheduled_at format: {}", e))
			})?
			.with_timezone(&Utc);

		let mentor_uuid = Uuid::parse_str(&mentor_id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid mentor ID: {}", e)))?;

		let mentee_uuid = Uuid::parse_str(&user_id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid user ID: {}", e)))?;

		let entity = SessionEntity {
			id: Uuid::new_v4(),
			mentor_id: mentor_uuid,
			mentee_id: mentee_uuid,
			topic: cmd.topic,
			description: cmd.description,
			scheduled_at,
			duration_minutes: cmd.duration_minutes.unwrap_or(60),
			meeting_link: None,
			session_type: cmd.session_type.unwrap_or_else(|| "video_call".to_string()),
			status: "pending".to_string(),
			feedback: None,
			rating: None,
			feedback_submitted_at: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let created = self.repo.create(entity).await?;

		Ok(BookedSession {
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

	pub async fn update_session_status(
		&self,
		session_id: String,
		_user_id: String,
		cmd: UpdateSessionStatusCommand,
	) -> Result<UpdatedSessionStatus, AppError> {
		let session_uuid = Uuid::parse_str(&session_id).map_err(|e| {
			AppError::BadRequestError(format!("Invalid session ID: {}", e))
		})?;

		let mut session = self
			.repo
			.find_by_id(session_uuid)
			.await?
			.ok_or_else(|| AppError::NotFoundError("Session not found".to_string()))?;

		session.status = cmd.status;
		if let Some(link) = cmd.meeting_link {
			session.meeting_link = Some(link);
		}
		session.updated_at = Utc::now();

		let updated = self.repo.update(session_uuid, session).await?;

		Ok(UpdatedSessionStatus {
			id: updated.id.to_string(),
			status: updated.status,
			meeting_link: updated.meeting_link,
			updated_at: updated.updated_at.to_rfc3339(),
		})
	}

	pub async fn submit_feedback(
		&self,
		session_id: String,
		user_id: String,
		cmd: SessionFeedbackCommand,
	) -> Result<SessionFeedbackResult, AppError> {
		let session_uuid = Uuid::parse_str(&session_id).map_err(|e| {
			AppError::BadRequestError(format!("Invalid session ID: {}", e))
		})?;

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

		session.feedback = Some(cmd.feedback.clone());
		session.rating = Some(cmd.rating);
		session.feedback_submitted_at = Some(Utc::now());
		session.updated_at = Utc::now();

		let updated = self.repo.update(session_uuid, session).await?;
		let submitted_at = updated
			.feedback_submitted_at
			.unwrap_or_else(Utc::now)
			.to_rfc3339();

		Ok(SessionFeedbackResult {
			id: updated.id.to_string(),
			feedback: cmd.feedback,
			rating: cmd.rating,
			submitted_at,
		})
	}
}
