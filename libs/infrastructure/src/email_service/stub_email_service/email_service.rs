use async_trait::async_trait;
use usecase::shared::email_service::{EmailMessage, EmailService, EmailServiceError};

pub struct StubEmailService;

impl StubEmailService {
    pub fn new() -> Self {
        StubEmailService {}
    }
}

impl Default for StubEmailService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailService for StubEmailService {
    async fn send_email(
        &self,
        EmailMessage { to, subject, body }: EmailMessage,
    ) -> Result<(), EmailServiceError> {
        // スタブ実装では、メール送信をシミュレートするだけです
        tracing::info!(
            "StubEmailService: Sending email to: {}, subject: {}, body: {}",
            to,
            subject,
            body
        );

        Ok(())
    }
}
