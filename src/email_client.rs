use crate::domain::SubscriberEmail;
use secrecy::Secret;

#[derive(Clone)]
pub struct EmailClient {
    http_client: reqwest::Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
    ) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange

        // Act

        // Assert
    }
}
