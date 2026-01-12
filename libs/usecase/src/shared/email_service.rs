use async_trait::async_trait;
use thiserror::Error;

pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Error)]
pub enum EmailServiceError {
    #[error("メールの送信に失敗しました: {0}")]
    SendError(#[source] anyhow::Error),
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, message: EmailMessage) -> Result<(), EmailServiceError>;
}
