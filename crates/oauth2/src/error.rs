pub(super) type Result<T> = std::result::Result<T, OAuth2Error>;

#[derive(Debug, thiserror::Error)]
pub enum OAuth2Error {
    #[error("Error requesting data from indexer")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid provider")]
    InvalidProvider,
    #[error("Failed to create token")]
    FailedToCreateToken,
    #[error("Failed to validate token")]
    FailedToValidateToken,
    #[error("Failed to request token")]
    FailedToRequestToken,
    #[error("Failed to request users info")]
    FailedToRequestUserInfo,
}
