use crate::domain::SubscriberEmail;
use secrecy::{ExposeSecret, Secret};
use sendgrid::error::SendgridError;
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};

#[derive(Clone)]
pub struct EmailClient {
    sg_client: Sender,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, authorization_token: Secret<String>) -> Self {
        Self {
            sg_client: Sender::new(authorization_token.expose_secret().to_string()),
            sender,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), SendgridError> {
        // personalization is a struct that hold information about the recipients
        // it also holds other necessary information to successfully send the email
        // if confused here, please refer to the sendgrid client doc on github
        // V3 is the implemented verson here
        let p = Personalization::new(Email::new(recipient.as_ref()));

        let message = Message::new(Email::new(self.sender.as_ref()))
            .set_subject(subject)
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(text_content),
            )
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(html_content),
            )
            .add_personalization(p);

        self.sg_client.send(&message).await?.error_for_status()?;
        Ok(())
    }
}
