use imphnen_libs::{ENV, Env};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

use crate::error::EmailError;

pub fn send_email(
	to: &str,
	subject: &str,
	body: &str,
) -> Result<(), Box<dyn Error>> {
	let env = &ENV;
	let message = build_message(to, subject, body, env)?;
	let mailer = build_transport(env)?;
	mailer.send(&message).map_err(|e| {
		tracing::error!("Failed to send email to {}: {}", to, e);
		Box::new(EmailError::Transport(e.to_string())) as Box<dyn Error>
	})?;
	tracing::info!("Email sent to: {}", to);
	Ok(())
}

fn build_message(
	to: &str,
	subject: &str,
	body: &str,
	env: &Env,
) -> Result<Message, Box<dyn Error>> {
	let sender_name = env.smtp_name.replace("-", " ");
	Message::builder()
		.from(Mailbox::new(Some(sender_name), env.smtp_email.parse()?))
		.to(to.parse()?)
		.subject(subject)
		.body(body.to_string())
		.map_err(|e| Box::new(EmailError::MessageBuild(e.to_string())) as Box<dyn Error>)
}

fn build_transport(env: &Env) -> Result<SmtpTransport, Box<dyn Error>> {
	let credentials =
		Credentials::new(env.smtp_email.clone(), env.smtp_password.replace("-", " "));
	Ok(
		SmtpTransport::relay(&env.smtp_host)?
			.credentials(credentials)
			.build(),
	)
}
