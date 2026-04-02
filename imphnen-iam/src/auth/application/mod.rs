use std::sync::Arc;
use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;
use imphnen_libs::{environment, encode_access_token, encode_refresh_token, encode_reset_password_token,
    decode_access_token, decode_refresh_token, hash_password, send_email, verify_password};
use imphnen_utils::{AppError, get_iso_date};
use imphnen_utils::generate_otp::OtpManager;
use imphnen_entities::{RolesDetailQueryDto, users::UserProfileExtensionDto};
use crate::auth::domain::AuthService;
use crate::auth::infrastructure::http::dto::{
    AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto,
    AuthRefreshTokenRequestDto, AuthRegisterRequestDto, AuthResendOtpRequestDto,
    AuthVerifyEmailRequestDto, TokenDto,
};
use crate::users::domain::{UserEntity, UserRepository};
use crate::users::infrastructure::http::dto::UsersDetailItemDto;
use crate::roles::domain::RoleRepository;

pub struct AuthServiceImpl {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
}

impl AuthServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>, role_repo: Arc<dyn RoleRepository>) -> Self {
        Self { user_repo, role_repo }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, payload: AuthLoginRequestDto) -> Result<AuthLoginResponsetDto, AppError> {
        let user = self.user_repo.find_by_email(payload.email.clone()).await
            .map_err(|_| AppError::AuthenticationError("Email or password not correct".into()))?;
        if !user.is_active {
            return Err(AppError::AuthenticationError("Account not active, please verify your email".into()));
        }
        let valid = verify_password(&payload.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Password verification failed".into()))?;
        if !valid {
            return Err(AppError::AuthenticationError("Email or password not correct".into()));
        }
        let access_token = encode_access_token(payload.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate access token".into()))?;
        let refresh_token = encode_refresh_token(payload.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate refresh token".into()))?;
        Ok(AuthLoginResponsetDto {
            user: UsersDetailItemDto::from(user),
            token: TokenDto { access_token, refresh_token },
        })
    }

    async fn login_mentor(&self, payload: AuthLoginRequestDto) -> Result<AuthLoginResponsetDto, AppError> {
        let user = self.user_repo.find_by_email(payload.email.clone()).await
            .map_err(|_| AppError::AuthenticationError("Email or password not correct".into()))?;
        if !user.is_active {
            return Err(AppError::AuthenticationError("Account not active, please verify your email".into()));
        }
        let valid = verify_password(&payload.password, &user.password)
            .map_err(|_| AppError::InternalServerError("Password verification failed".into()))?;
        if !valid {
            return Err(AppError::AuthenticationError("Email or password not correct".into()));
        }
        if user.role.name != "Mentor" {
            return Err(AppError::ForbiddenError("User does not have mentor privileges".into()));
        }
        let access_token = encode_access_token(payload.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate access token".into()))?;
        let refresh_token = encode_refresh_token(payload.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate refresh token".into()))?;
        Ok(AuthLoginResponsetDto {
            user: UsersDetailItemDto::from(user),
            token: TokenDto { access_token, refresh_token },
        })
    }

    async fn register(&self, payload: AuthRegisterRequestDto) -> Result<(), AppError> {
        let role = self.role_repo.find_by_name("User".into()).await
            .map_err(|_| AppError::NotFoundError("Role not found".into()))?;
        if self.user_repo.find_by_email(payload.email.clone()).await.is_ok() {
            return Err(AppError::BadRequestError("User already exists".into()));
        }
        let hashed = hash_password(&payload.password)
            .map_err(|e| { error!("Failed to hash password: {}", e); AppError::InternalServerError("Failed to hash password".into()) })?;
        let otp = OtpManager::generate_otp();
        send_email(&payload.email, "OTP Verification", &format!("your otp code is {}", otp.code))
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        self.user_repo.create(UserEntity {
            id: Uuid::new_v4().to_string(),
            email: payload.email.clone(),
            fullname: payload.fullname.clone(),
            password: hashed,
            is_active: false,
            is_deleted: false,
            role: RolesDetailQueryDto { id: role.id.to_string(), name: role.name, ..Default::default() },
            profile_extension: Some(UserProfileExtensionDto { phone_number: payload.phone_number, ..Default::default() }),
            created_at: get_iso_date(),
            updated_at: get_iso_date(),
            ..Default::default()
        }).await?;
        Ok(())
    }

    async fn resend_otp(&self, payload: AuthResendOtpRequestDto) -> Result<(), AppError> {
        self.user_repo.find_by_email(payload.email.clone()).await
            .map_err(|_| AppError::NotFoundError("User not found".into()))?;
        let otp = OtpManager::generate_otp();
        send_email(&payload.email, "OTP Verification", &format!("Your OTP code is {}", otp.code))
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        Ok(())
    }

    async fn refresh_token(&self, payload: AuthRefreshTokenRequestDto) -> Result<TokenDto, AppError> {
        let email = decode_refresh_token(&payload.refresh_token)
            .map_err(|_| AppError::AuthenticationError("Invalid refresh token".into()))?.claims.sub;
        let user = self.user_repo.find_by_email(email.clone()).await
            .map_err(|_| AppError::AuthenticationError("User not found".into()))?;
        let access_token = encode_access_token(user.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate access token".into()))?;
        let refresh_token = encode_refresh_token(user.email.clone(), user.id.clone())
            .map_err(|_| AppError::InternalServerError("Failed to generate refresh token".into()))?;
        Ok(TokenDto { access_token, refresh_token })
    }

    async fn forgot_password(&self, payload: AuthResendOtpRequestDto) -> Result<(), AppError> {
        let user_repo = Arc::clone(&self.user_repo);
        tokio::spawn(async move {
            if let Ok(user) = user_repo.find_by_email(payload.email.clone()).await {
                match encode_reset_password_token(user.email.clone(), user.id.clone()) {
                    Ok(token) => {
                        let fe_url = environment::ENV.fe_url.clone();
                        let msg = format!(
                            "You have requested a password reset. Please click the link below: {fe_url}/auth/reset-password?token={token}"
                        );
                        if let Err(e) = send_email(&payload.email, "Reset Password Request", &msg) {
                            error!("Failed to send reset password email: {}", e);
                        }
                    }
                    Err(e) => error!("Failed to generate reset token: {:?}", e),
                }
            }
        });
        Ok(())
    }

    async fn verify_email(&self, payload: AuthVerifyEmailRequestDto) -> Result<(), AppError> {
        let user = self.user_repo.find_by_email(payload.email.clone()).await
            .map_err(|_| AppError::NotFoundError("User not found".into()))?;
        if user.is_active {
            return Err(AppError::BadRequestError("User already active".into()));
        }
        self.user_repo.update(UserEntity { is_active: true, ..user }).await?;
        Ok(())
    }

    async fn new_password(&self, payload: AuthNewPasswordRequestDto) -> Result<(), AppError> {
        let email = decode_access_token(&payload.token)
            .map_err(|_| AppError::BadRequestError("Invalid or missing token".into()))?.claims.sub;
        let user = self.user_repo.find_by_email(email).await
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        let hashed = hash_password(&payload.password)
            .map_err(|e| { error!("Failed to hash new password: {}", e); AppError::InternalServerError("Failed to hash password".into()) })?;
        self.user_repo.update(UserEntity { password: hashed, ..user }).await?;
        Ok(())
    }
}
