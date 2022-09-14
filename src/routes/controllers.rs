use gotham::prelude::StaticResponseExtender;
use gotham::state::{State, StateData, FromState};

use std::env;

use lettre::Transport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};
use lettre::message::{MultiPart, SinglePart, header};

use serde::{Deserialize, Serialize};

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct SendMailQueryStringExtractor {
    email: String,
    subject: String,
    body: String,
    fallback: String,
}

#[derive(Serialize)]
struct MailContent {
    email: String,
    subject: String,
    body: String,
    fallback: String,
}

pub fn send_mail(mut state: State) -> (State, &'static str) {
    dotenv::dotenv().ok();

    let query_param = SendMailQueryStringExtractor::take_from(&mut state);

    let email_config = MailContent {
        email: query_param.email,
        subject: query_param.subject,
        body: query_param.body,
        fallback: query_param.fallback
    };

    let sender_email = env::var("EMAIL").unwrap();
    let sender_pass = env::var("PASSWORD").unwrap();
    
    let email = Message::builder()
        .from(sender_email.parse().unwrap())
        .to(email_config.email.parse().unwrap())
        .subject(email_config.subject)
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from(email_config.fallback)), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(email_config.body),
                ),
        )
        .expect("Something was wrong while the email was created");

    let creds = Credentials::new(sender_email, sender_pass);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    let result;

    match mailer.send(&email) {
        Ok(_) => result = "Email sent successfully!",
        Err(_) => result = "Could not send email",
    };

    (state, result)
}