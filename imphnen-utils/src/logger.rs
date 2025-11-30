use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt};

/// Initializes the logger using tracing and tracing-subscriber.
/// Loads environment variables from `.env` and sets log level from `RUST_LOG`.
pub fn init_logger() {
    dotenv().ok();


    // Set up the tracing subscriber with EnvFilter from RUST_LOG
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("warn"))
        .unwrap();

    fmt()
        .with_env_filter(filter)
        .init();
}
