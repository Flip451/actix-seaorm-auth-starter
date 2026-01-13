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

impl StubEmailService {
    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
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
