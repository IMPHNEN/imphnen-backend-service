use crate::users::infrastructure::http::dto::UsersDetailItemDto;
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthLoginRequestDto {
    #[zod(email, min_length(1))]
    pub email: String,
    #[zod(min_length(1))]
    pub password: String,
}

impl ZodValidate for AuthLoginRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct TokenDto {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginResponsetDto {
    pub token: TokenDto,
    pub user: UsersDetailItemDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthRegisterRequestDto {
    #[zod(email, min_length(1))]
    pub email: String,
    #[zod(min_length(8), regex(pattern = "^[A-Za-z\\d@$!%*?&]{8,}$"))]
    pub password: String,
    #[zod(min_length(2))]
    pub fullname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

impl ZodValidate for AuthRegisterRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthVerifyEmailRequestDto {
    #[zod(email, min_length(1))]
    pub email: String,
    pub otp: u32,
}

impl ZodValidate for AuthVerifyEmailRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthResendOtpRequestDto {
    #[zod(email, min_length(1))]
    pub email: String,
}

impl ZodValidate for AuthResendOtpRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthRefreshTokenRequestDto {
    #[zod(min_length(1))]
    pub refresh_token: String,
}

impl ZodValidate for AuthRefreshTokenRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct AuthNewPasswordRequestDto {
    #[zod(min_length(1))]
    pub token: String,
    #[zod(min_length(8), regex(pattern = "^[A-Za-z\\d@$!%*?&]{8,}$"))]
    pub password: String,
}

impl ZodValidate for AuthNewPasswordRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}
