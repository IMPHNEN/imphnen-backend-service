use crate::{
	v1::{
		auth, gacha, users, AuthLoginRequestDto, AuthLoginResponsetDto,
		AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, GachaCreateClaimRequestDto,
		GachaCreateItemRequestDto, GachaCreateRollRequestDto, CreateUserRequestDto, UpdateRequestDto
	},
	MessageResponseDto, MetaRequestDto, MetaResponseDto, ResponseSuccessDto,
};

use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
     auth::auth_controller::post_login,
     auth::auth_controller::post_register,
     auth::auth_controller::post_verify_email,
     auth::auth_controller::post_resend_otp,
     gacha::gacha_controller::post_create_gacha_claim,
     gacha::gacha_controller::post_create_gacha_item,
     gacha::gacha_controller::post_create_gacha_roll,
	 users::user_controller::post_create_user,
	 users::user_controller::get_user,
	 users::user_controller::put_user,
	 users::user_controller::delete_user,
	 users::user_controller::get_user_by_id
    ),
    components(
        schemas(
           MetaRequestDto,
           MetaResponseDto,
           MessageResponseDto,
           AuthLoginRequestDto,
           AuthLoginResponsetDto,
           AuthVerifyEmailRequestDto,
           AuthResendOtpRequestDto,
           ResponseSuccessDto<AuthLoginResponsetDto>,
           GachaCreateClaimRequestDto,
           GachaCreateItemRequestDto,
           GachaCreateRollRequestDto,
		   CreateUserRequestDto, 
		   UpdateRequestDto, 
        )
    ),
    info(
        title = "IMPHNEN API",
        description = "IMPHNEN API Documentation",
        version = "0.1.0",
        contact(
            name = "Maulana Sodiqin",
            url = ""
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "List of Authentication Endpoints"),
    )
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}
	}
}
