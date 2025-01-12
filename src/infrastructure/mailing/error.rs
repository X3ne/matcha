#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Template error: {0}")]
    TemplateError(#[from] mail_template::error::Error),
    #[error("Error parsing sender information")]
    SenderParsingError,
    #[error("Error parsing receiver information")]
    ReceiverParsingError,
    #[error("Error sending email")]
    SendError,
    #[error("Smtp transport error")]
    SmtpTransportError,
    #[error("Error loading template")]
    TemplateLoadError,
}
