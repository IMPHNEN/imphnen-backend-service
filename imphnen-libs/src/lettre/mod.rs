use super::Env;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub fn send_email(
	to: &str,
	subject: &str,
	body: &str,
) -> Result<(), Box<dyn Error>> {
	let env = Env::new();
	let host = env.smtp_host;
	let sender_email = env.smtp_email;
	let sender_name = env.smtp_name;
	let sender_password = env.smtp_password;
	let recipient_email = to;
	let email = Message::builder()
		.from(Mailbox::new(
			Some(sender_name.replace("-", " ")),
			sender_email.parse()?,
		))
		.to(recipient_email.parse()?)
		.subject(subject)
		.body(body.to_string())?;
	let smtp_credentials =
		Credentials::new(sender_email, sender_password.replace("-", " "));
	let mailer = SmtpTransport::relay(&host)?
		.credentials(smtp_credentials)
		.build();
	match mailer.send(&email) {
		Ok(_) => {
			println!("Email sent successfully to {}", to);
			Ok(())
		}
		Err(e) => {
			println!("Failed to send email: {}", e);
			Err(Box::new(e))
		}
	}
}
