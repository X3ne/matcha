use std::fs;
use std::path::{Path, PathBuf};

use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use mail_template::render_email;

use crate::config::SmtpConfig;
use crate::domain::constants::{ACCOUNT_CONFIRMATION_TEMPLATE, RESET_PASSWORD_TEMPLATE, TEMPLATE_DIR};
use crate::domain::entities::user::User;
use crate::infrastructure::mailing::error::Error;
use crate::infrastructure::mailing::mails::Mail;

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

    fn get_template_content(&self, template_path: PathBuf) -> Result<String, Error> {
        let content = fs::read_to_string(&template_path).map_err(|e| {
            tracing::error!("Failed to read template: {:?}", e);
            Error::TemplateLoadError
        })?;

        Ok(content)
    }

    pub async fn send_confirmation_mail(&self, user: &User, confirmation_url: String) -> Result<(), Error> {
        let (html_path, subject_path, text_path) = self.get_template_paths(ACCOUNT_CONFIRMATION_TEMPLATE);

        let html = self.get_template_content(html_path)?;
        let subject = self.get_template_content(subject_path)?;
        let txt = self.get_template_content(text_path)?;

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

    pub async fn send_password_reset_mail(&self, email: &str, reset_token: &str) -> Result<(), Error> {
        let (html_path, subject_path, text_path) = self.get_template_paths(RESET_PASSWORD_TEMPLATE);

        let html = self.get_template_content(html_path)?;
        let subject = self.get_template_content(subject_path)?;
        let txt = self.get_template_content(text_path)?;

        let reset_url = format!(
            "http://localhost:3000/v1/auth/reset?token={}", // TODO: make this configurable
            reset_token
        );

        let rendered = render_email(&Mail::ResetPassword { reset_url }, &subject, &html, &txt).await?;

        self.send_mail(email, rendered.subject(), rendered.html(), rendered.txt())
            .await
    }
}
