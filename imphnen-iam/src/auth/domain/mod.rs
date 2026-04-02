pub mod types;

use async_trait::async_trait;
use imphnen_utils::AppError;
use types::{
	AuthTokens, LoginInput, LoginOutput, NewPasswordInput, RefreshTokenInput,
	RegisterInput, ResendOtpInput, VerifyEmailInput,
};

#[async_trait]
pub trait AuthService: Send + Sync {
	async fn login(&self, payload: LoginInput) -> Result<LoginOutput, AppError>;
	async fn login_mentor(&self, payload: LoginInput)
	-> Result<LoginOutput, AppError>;
	async fn register(&self, payload: RegisterInput) -> Result<(), AppError>;
	async fn resend_otp(&self, payload: ResendOtpInput) -> Result<(), AppError>;
	async fn refresh_token(
		&self,
		payload: RefreshTokenInput,
	) -> Result<AuthTokens, AppError>;
	async fn forgot_password(&self, payload: ResendOtpInput) -> Result<(), AppError>;
	async fn verify_email(&self, payload: VerifyEmailInput) -> Result<(), AppError>;
	async fn new_password(&self, payload: NewPasswordInput) -> Result<(), AppError>;
}
