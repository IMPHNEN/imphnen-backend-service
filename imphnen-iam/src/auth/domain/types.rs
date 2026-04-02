use imphnen_entities::{RolesDetailQueryDto, users::UserProfileExtensionDto};

#[derive(Clone, Debug)]
pub struct LoginInput {
	pub email: String,
	pub password: String,
}

#[derive(Clone, Debug)]
pub struct RegisterInput {
	pub email: String,
	pub password: String,
	pub fullname: String,
	pub phone_number: Option<String>,
}

#[derive(Clone, Debug)]
pub struct VerifyEmailInput {
	pub email: String,
	pub otp: u32,
}

#[derive(Clone, Debug)]
pub struct ResendOtpInput {
	pub email: String,
}

#[derive(Clone, Debug)]
pub struct RefreshTokenInput {
	pub refresh_token: String,
}

#[derive(Clone, Debug)]
pub struct NewPasswordInput {
	pub token: String,
	pub password: String,
}

#[derive(Clone, Debug, Default)]
pub struct AuthTokens {
	pub access_token: String,
	pub refresh_token: String,
}

#[derive(Clone, Debug, Default)]
pub struct AuthUserDetail {
	pub id: String,
	pub email: String,
	pub fullname: String,
	pub legal_name: Option<String>,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub role: RolesDetailQueryDto,
	pub profile_extension: Option<UserProfileExtensionDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct LoginOutput {
	pub token: AuthTokens,
	pub user: AuthUserDetail,
}
