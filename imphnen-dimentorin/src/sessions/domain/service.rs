use super::session_types::{
	BookSessionCommand, BookedSession, MentorAvailability, SessionDetail,
	SessionFeedbackCommand, SessionFeedbackResult, SessionList,
	UpdateSessionStatusCommand, UpdatedSessionStatus,
};
use async_trait::async_trait;
use imphnen_utils::AppError;

#[async_trait]
pub trait SessionService: Send + Sync {
	async fn book_session(
		&self,
		mentor_id: String,
		user_id: String,
		cmd: BookSessionCommand,
	) -> Result<BookedSession, AppError>;

	async fn get_mentor_sessions(
		&self,
		mentor_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError>;

	async fn get_user_sessions(
		&self,
		user_id: String,
		status_filter: Option<String>,
	) -> Result<SessionList, AppError>;

	async fn get_mentor_availability(
		&self,
		mentor_id: String,
	) -> Result<MentorAvailability, AppError>;

	async fn update_session_status(
		&self,
		session_id: String,
		user_id: String,
		cmd: UpdateSessionStatusCommand,
	) -> Result<UpdatedSessionStatus, AppError>;

	async fn submit_feedback(
		&self,
		session_id: String,
		user_id: String,
		cmd: SessionFeedbackCommand,
	) -> Result<SessionFeedbackResult, AppError>;

	async fn get_session_detail(
		&self,
		session_id: String,
	) -> Result<SessionDetail, AppError>;
}
