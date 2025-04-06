use rand::{rng, Rng};

pub struct OtpManager;

impl OtpManager {
	pub fn generate_otp() -> u32 {
		rng().random_range(100_000..1_000_000)
	}

	pub fn validate_otp(stored_otp: u32, user_otp: u32) -> bool {
		stored_otp == user_otp
	}
}
