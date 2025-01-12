use crate::config::SmtpConfig;
use crate::domain::constants::TEMPLATE_DIR;
use crate::domain::entities::user::User;
use crate::infrastructure::mailing::error::Error;
use crate::infrastructure::mailing::mails::Mail;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use mail_template::render_email;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Sender {
    smtp_cfg: SmtpConfig,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Sender {
    pub fn new(smtp_cfg: SmtpConfig) -> Result<Self, Error> {
        let creds = Credentials::new(smtp_cfg.user.clone(), smtp_cfg.password.clone());

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_cfg.host)
            .map_err(|e| {
                tracing::error!("{}", e);
                Error::SmtpTransportError
            })?
            .port(smtp_cfg.port)
            .credentials(creds)
            .build();

        Ok(Self { smtp_cfg, mailer })
    }

    async fn send_mail(&self, to_email: &str, subject: &str, body_html: &str, body_txt: &str) -> Result<(), Error> {
        let from = format!("{} <{}>", self.smtp_cfg.email_from_name, self.smtp_cfg.email_from_email);
        let to = format!("<{}>", to_email);
        let email = Message::builder()
            .from(from.parse().map_err(|e| {
                tracing::error!("{}", e);
                Error::SenderParsingError
            })?)
            .to(to.parse().map_err(|e| {
                tracing::error!("{}", e);
                Error::ReceiverParsingError
            })?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(body_txt.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(body_html.to_string()),
                    ),
            )
            .unwrap();

        self.mailer.send(email).await.map_err(|e| {
            tracing::error!("{}", e);
            Error::SendError
        })?;

        Ok(())
    }

    fn get_template_paths(&self, template_name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let template_dir = format!("{}/{}", TEMPLATE_DIR, template_name);

        let html_path = Path::new(&template_dir).join("mail.html");
        let subject_path = Path::new(&template_dir).join("subject.txt");
        let text_path = Path::new(&template_dir).join("text.txt");

        (html_path, subject_path, text_path)
    }

    pub async fn send_confirmation_mail(&self, user: &User) -> Result<(), Error> {
        let (html_path, subject_path, text_path) = self.get_template_paths("account_confirmation");

        let html = fs::read_to_string(&html_path).map_err(|e| {
            tracing::error!("Failed to read HTML template: {:?}", e);
            Error::TemplateLoadError
        })?;

        let subject = fs::read_to_string(&subject_path).map_err(|e| {
            tracing::error!("Failed to read subject template: {:?}", e);
            Error::TemplateLoadError
        })?;

        let txt = fs::read_to_string(&text_path).map_err(|e| {
            tracing::error!("Failed to read text template: {:?}", e);
            Error::TemplateLoadError
        })?;

        let confirmation_url = format!(
            "http://localhost:3000/auth/activation?token={}",
            user.activation_token.clone().unwrap()
        ); // TODO: make activation token NOT NULL

        let rendered = render_email(
            &Mail::AccountConfirmation {
                username: user.username.clone(),
                confirmation_url,
            },
            &subject,
            &html,
            &txt,
        )
        .await?;

        self.send_mail(&user.email, rendered.subject(), rendered.html(), rendered.txt())
            .await
    }
}
