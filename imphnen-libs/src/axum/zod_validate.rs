use serde_json::Value;

pub trait ZodValidate: Sized {
	fn zod_validate(value: &Value) -> Result<Self, String>;
}
