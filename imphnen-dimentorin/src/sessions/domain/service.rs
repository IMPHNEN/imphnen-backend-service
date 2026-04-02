use async_trait::async_trait;
use imphnen_utils::AppError;
use crate::sessions::infrastructure::http::dto::{
    BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionDetailDto, SessionFeedbackRequestDto, SessionFeedbackResponseDto,
    SessionListResponseDto, UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
};

#[async_trait]
pub trait SessionService: Send + Sync {
    async fn book_session(
        &self,
        mentor_id: String,
        user_id: String,
        dto: BookSessionRequestDto,
    ) -> Result<BookSessionResponseDto, AppError>;

    async fn get_mentor_sessions(
        &self,
        mentor_id: String,
        status_filter: Option<String>,
    ) -> Result<SessionListResponseDto, AppError>;

    async fn get_user_sessions(
        &self,
        user_id: String,
        status_filter: Option<String>,
    ) -> Result<SessionListResponseDto, AppError>;

    async fn get_mentor_availability(
        &self,
        mentor_id: String,
    ) -> Result<MentorAvailabilityDto, AppError>;

    async fn update_session_status(
        &self,
        session_id: String,
        user_id: String,
        dto: UpdateSessionStatusRequestDto,
    ) -> Result<UpdateSessionStatusResponseDto, AppError>;

    async fn submit_feedback(
        &self,
        session_id: String,
        user_id: String,
        dto: SessionFeedbackRequestDto,
    ) -> Result<SessionFeedbackResponseDto, AppError>;

    async fn get_session_detail(
        &self,
        session_id: String,
    ) -> Result<SessionDetailDto, AppError>;
}
