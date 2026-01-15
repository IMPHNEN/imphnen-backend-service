/// Google OAuth integration module
pub mod google_oauth_controller;
pub mod google_oauth_dto;
pub mod google_oauth_service;

// Export only essential types and functions from Google OAuth submodules
pub use google_oauth_controller::GoogleOauthController;