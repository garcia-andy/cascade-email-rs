use serde::{Deserialize, Serialize};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

/// Create a new email message with the given parameters.
fn create_message(remitente: &str, destinatario: &str, subject: &str, content: &str) -> Message {
    let email = Message::builder()
        .from(remitente.parse().expect("Invalid email (remitente)"))
        .to(destinatario.parse().expect("Invalid email (destinatario)"))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(content))
        .unwrap();

    email
}

/// Open a connection to the given relay and authenticate with the given credentials.
fn open_connection(relay: &str, creds: &Credentials) -> SmtpTransport {
    SmtpTransport::relay(relay)
        .expect(format!("Could not connect to {}", relay).as_str())
        .credentials(creds.clone())
        .build()
}

/// Send the given email using the given relay and credentials.
fn send_email(email: &Message, relay: &str, creds: &Credentials) {
    let mailer = open_connection(relay, creds);
    match mailer.send(email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mailer {
    domain: String,
    username: String,
    password: String,
    relay: String,
    remitente: String,
    subject: String,
    content: String,
}

impl Mailer {
    pub fn from_json(path: &str) -> Self {
        let file = std::fs::File::open(path).expect("Failed to open file");
        let reader = std::io::BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader).expect("Error parsing json");

        let m: Mailer = serde_json::from_value(json["mailer"].clone()).expect("Error parsing json");

        Self::new(
            &m.domain,
            &m.username,
            &m.password,
            &m.relay,
            &m.remitente,
            &m.subject,
            &m.content,
        )
    }

    pub fn builder() -> Self {
        Self::new("", "", "", "", "", "", "")
    }

    pub fn new(
        domain: &str,
        username: &str,
        password: &str,
        relay: &str,
        remitente: &str,
        subject: &str,
        content: &str,
    ) -> Self {
        Self {
            domain: domain.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            relay: relay.to_owned(),
            remitente: format!("{remitente} <{username}@{domain}>"),
            subject: subject.to_owned(),
            content: content.to_owned(),
        }
    }

    pub fn set_domain(&mut self, domain: &str) -> &mut Self {
        self.domain = domain.to_owned();
        self
    }

    pub fn set_username(&mut self, username: &str) -> &mut Self {
        self.username = username.to_owned();
        self
    }

    pub fn set_password(&mut self, password: &str) -> &mut Self {
        self.password = password.to_owned();
        self
    }

    pub fn set_relay(&mut self, relay: &str) -> &mut Self {
        self.relay = relay.to_owned();
        self
    }

    pub fn set_remitente(&mut self, remitente: &str) -> &mut Self {
        self.remitente = format!("{remitente} <{}@{}>", self.username, self.domain);
        self
    }

    pub fn set_subject(&mut self, subject: &str) -> &mut Self {
        self.subject = subject.to_owned();
        self
    }

    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = content.to_owned();
        self
    }

    pub fn send_email(&self, name: &str, mail: &str) {
        let email = create_message(
            &self.remitente,
            &format!("{name} <{mail}>"),
            &self.subject,
            &self.content,
        );
        let creds = Credentials::new(self.username.clone(), self.password.clone());

        send_email(&email, &self.relay, &creds);
    }
}
