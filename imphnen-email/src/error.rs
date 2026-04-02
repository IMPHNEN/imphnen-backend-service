use std::fmt;

#[derive(Debug)]
pub enum EmailError {
	SmtpConfig(String),
	MessageBuild(String),
	Transport(String),
}

impl fmt::Display for EmailError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EmailError::SmtpConfig(msg) => write!(f, "SMTP configuration error: {msg}"),
			EmailError::MessageBuild(msg) => write!(f, "Message building error: {msg}"),
			EmailError::Transport(msg) => write!(f, "SMTP transport error: {msg}"),
		}
	}
}

impl std::error::Error for EmailError {}
