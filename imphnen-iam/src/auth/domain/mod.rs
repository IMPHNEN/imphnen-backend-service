use async_trait::async_trait;
use imphnen_utils::AppError;
use crate::auth::infrastructure::http::dto::{
    AuthLoginRequestDto, AuthLoginResponsetDto, AuthRegisterRequestDto,
    AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, AuthNewPasswordRequestDto,
    AuthRefreshTokenRequestDto, TokenDto,
};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login(&self, payload: AuthLoginRequestDto) -> Result<AuthLoginResponsetDto, AppError>;
    async fn login_mentor(&self, payload: AuthLoginRequestDto) -> Result<AuthLoginResponsetDto, AppError>;
    async fn register(&self, payload: AuthRegisterRequestDto) -> Result<(), AppError>;
    async fn resend_otp(&self, payload: AuthResendOtpRequestDto) -> Result<(), AppError>;
    async fn refresh_token(&self, payload: AuthRefreshTokenRequestDto) -> Result<TokenDto, AppError>;
    async fn forgot_password(&self, payload: AuthResendOtpRequestDto) -> Result<(), AppError>;
    async fn verify_email(&self, payload: AuthVerifyEmailRequestDto) -> Result<(), AppError>;
    async fn new_password(&self, payload: AuthNewPasswordRequestDto) -> Result<(), AppError>;
}
