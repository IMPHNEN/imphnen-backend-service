use super::session_booking_service::SessionBookingService;
use super::session_query_service::SessionQueryService;
use crate::sessions::domain::{
	BookSessionCommand, BookedSession, MentorAvailability, SessionDetail,
	SessionFeedbackCommand, SessionFeedbackResult, SessionList, SessionRepository,
	SessionService, UpdateSessionStatusCommand, UpdatedSessionStatus,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use std::sync::Arc;

pub struct SessionServiceImpl {
	booking: SessionBookingService,
	query: SessionQueryService,
}

impl SessionServiceImpl {
	pub fn new(repo: Arc<dyn SessionRepository>) -> Self {
		Self {
			booking: SessionBookingService {
				repo: Arc::clone(&repo),
			},
			query: SessionQueryService { repo },
		}
	}
}

#[async_trait]
impl SessionService for SessionServiceImpl {
	async fn book_session(
		&self,
		mentor_id: String,
		user_id: String,
		cmd: BookSessionCommand,
	) -> Result<BookedSession, AppError> {
		self.booking.book_session(mentor_id, user_id, cmd).await
	}

	async fn get_mentor_sessions(
		&self,
		mentor_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError> {
		self
			.query
			.get_mentor_sessions(mentor_id, status_filter)
			.await
	}

	async fn get_user_sessions(
		&self,
		user_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError> {
		self.query.get_user_sessions(user_id, status_filter).await
	}

	async fn get_mentor_availability(
		&self,
		mentor_id: String,
	) -> Result<MentorAvailability, AppError> {
		self.query.get_mentor_availability(mentor_id).await
	}

	async fn update_session_status(
		&self,
		session_id: String,
		user_id: String,
		cmd: UpdateSessionStatusCommand,
	) -> Result<UpdatedSessionStatus, AppError> {
		self
			.booking
			.update_session_status(session_id, user_id, cmd)
			.await
	}

	async fn submit_feedback(
		&self,
		session_id: String,
		user_id: String,
		cmd: SessionFeedbackCommand,
	) -> Result<SessionFeedbackResult, AppError> {
		self.booking.submit_feedback(session_id, user_id, cmd).await
	}

	async fn get_session_detail(
		&self,
		session_id: String,
	) -> Result<SessionDetail, AppError> {
		self.query.get_session_detail(session_id).await
	}
}
