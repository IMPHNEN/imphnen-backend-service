//! Email sending utilities using Lettre SMTP client.
//!
//! This module provides functionality for sending emails through SMTP
//! with proper error handling and logging.

use crate::environment::ENV;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;
use std::fmt;

/// Custom error type for email operations.
#[derive(Debug)]
pub enum EmailError {
    /// SMTP configuration error
    SmtpConfig(String),
    /// Message building error
    MessageBuild(String),
    /// SMTP transport error
    Transport(String),
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::SmtpConfig(msg) => write!(f, "SMTP configuration error: {}", msg),
            EmailError::MessageBuild(msg) => write!(f, "Message building error: {}", msg),
            EmailError::Transport(msg) => write!(f, "SMTP transport error: {}", msg),
        }
    }
}

impl Error for EmailError {}

/// Send an email using the configured SMTP settings.
///
/// This function constructs and sends an email using the SMTP configuration
/// from environment variables. It handles sender name normalization and
/// proper error reporting.
///
/// # Arguments
/// * `to` - Recipient email address
/// * `subject` - Email subject line
/// * `body` - Email body content (plain text)
///
/// # Returns
/// * `Ok(())` - Email sent successfully
/// * `Err(EmailError)` - Email sending failed
///
/// # Example
/// ```
/// use imphnen_libs::send_email;
///
/// send_email("user@example.com", "Welcome!", "Hello, welcome to our service!")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
    let env = &ENV;

    // Build the email message
    let message = build_email_message(to, subject, body, env)?;

    // Create SMTP transport
    let mailer = create_smtp_transport(env)?;

    // Send the email
    mailer.send(&message).map_err(|e| {
        log::error!("Failed to send email to {}: {}", to, e);
        Box::new(EmailError::Transport(e.to_string())) as Box<dyn Error>
    })?;

    log::info!("Email sent successfully to: {}", to);
    Ok(())
}

/// Build an email message with proper sender and recipient configuration.
///
/// # Arguments
/// * `to` - Recipient email address
/// * `subject` - Email subject
/// * `body` - Email body
/// * `env` - Environment configuration
///
/// # Returns
/// Email message or error
fn build_email_message(
    to: &str,
    subject: &str,
    body: &str,
    env: &crate::environment::Env,
) -> Result<Message, Box<dyn Error>> {
    let sender_name = env.smtp_name.replace("-", " "); // Normalize sender name

    Message::builder()
        .from(Mailbox::new(Some(sender_name), env.smtp_email.parse()?))
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())
        .map_err(|e| Box::new(EmailError::MessageBuild(e.to_string())) as Box<dyn Error>)
}

/// Create SMTP transport with authentication.
///
/// # Arguments
/// * `env` - Environment configuration
///
/// # Returns
/// Configured SMTP transport or error
fn create_smtp_transport(env: &crate::environment::Env) -> Result<SmtpTransport, Box<dyn Error>> {
    let credentials = Credentials::new(
        env.smtp_email.clone(),
        env.smtp_password.replace("-", " "), // Normalize password
    );

    Ok(SmtpTransport::relay(&env.smtp_host)?
        .credentials(credentials)
        .build())
        
}
