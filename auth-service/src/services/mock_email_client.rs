use crate::domain::email::Email;
use crate::domain::email_client::EmailClient;
use secrecy::ExposeSecret;

#[derive(Default)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<(), String> {
        // Our mock email client will simply log the recipient, subject, and content to standard output
        println!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.0.expose_secret(),
            subject,
            content
        );

        Ok(())
    }
}
