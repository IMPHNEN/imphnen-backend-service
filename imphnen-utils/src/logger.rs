use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logger() {
	dotenv().ok();

	let filter = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("warn"))
		.expect("valid log filter");

	fmt().with_env_filter(filter).init();
}
