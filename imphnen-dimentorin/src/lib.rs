pub mod v1;

// Explicitly export only what's needed from v1
pub use v1::dimentorin_router;
pub use v1::mentors::mentors_router;
pub use v1::sessions::{
    sessions_router, BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListItemDto,
    SessionListResponseDto, UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
    AvailabilitySlotDto,
};

