use crate::domain::subscriber::ValidEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Serialize)]
struct EmailRequestFrom<'a> {
    email: &'a str,
    name: &'a str,
}

#[derive(serde::Serialize)]
struct EmailRequest<'a> {
    from: EmailRequestFrom<'a>,
    to: Vec<&'a str>,
    subject: &'a str,
    text: &'a str,
    category: &'a str,
}

pub struct EmailClient {
    http_client: reqwest::Client,
    base_url: String,
    sender: ValidEmail,
    name: String,
    authorization_token: SecretString,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: ValidEmail,
        name: String,
        authorization_token: SecretString,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        Self {
            http_client,
            base_url,
            sender,
            name,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: ValidEmail,
        subject: &str,
        content: &str,
    ) -> Result<(), reqwest::Error> {
        let email_request_sender = EmailRequestFrom {
            email: self.sender.as_ref(),
            name: self.name.as_str(),
        };
        let email_request = EmailRequest {
            from: email_request_sender,
            to: vec![recipient.as_ref()],
            subject,
            text: content,
            category: "Integration Test",
        };

        let mut auth_header_value = String::from("Bearer ");
        auth_header_value.push_str(self.authorization_token.expose_secret());

        self.http_client
            .post(self.base_url.clone())
            .header("Authorization", auth_header_value)
            .json(&email_request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, faker::internet::en::SafeEmail};
    use secrecy::SecretString;
    use wiremock::Request;
    use wiremock::matchers::{self, any};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{domain::subscriber::ValidEmail, email_client::EmailClient};

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }
    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }
    /// Generate a random subscriber email
    fn email() -> ValidEmail {
        ValidEmail::parse(SafeEmail().fake()).unwrap()
    }

    /// Get a test instance of `EmailClient`.
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            "Tyrell Corporation".to_string(),
            SecretString::from("Hey there"),
            std::time::Duration::from_millis(200),
        )
    }

    struct SendEmailBodyMatcher;
    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("from").is_some()
                    && body.get("to").is_some()
                    && body.get("subject").is_some()
                    && body.get("text").is_some()
                    && body.get("category").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_expected_email_request() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(matchers::header_exists("Authorization"))
            .and(matchers::header("Content-Type", "application/json"))
            .and(matchers::method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = email_client
            .send_email(email(), &subject(), &content())
            .await;
    }

    #[tokio::test]
    async fn send_email_succeeds_with_200_response() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_with_500_response() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200)
            // 3 minutes!
            .set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(email(), &subject(), &content())
            .await;

        assert_err!(outcome);
    }
}
