pub mod v1;

// Re-export core entity types used across the hackathon system
pub use imphnen_entities::{
    CountResult,
    Error,
    ErrorDto,
    MessageResponseDto,
    MetaRequestDto,
    MetaResponseDto,
    ResponseListSuccessDto,
    ResponseSuccessDto,
};

// Explicitly import only what we need from libs and utils to avoid pollution
pub use imphnen_libs::{
    AppState,
};

// Re-export public v1 API
pub use v1::hackathon::hackathon_controller::hackathon_routes;