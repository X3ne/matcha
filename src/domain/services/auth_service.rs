use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use async_trait::async_trait;
use oauth2::client::providers::ProviderKind;
use oauth2::client::{CsrfToken, Url};

#[async_trait]
pub trait AuthService: 'static + Sync + Send {
    async fn login(&self, email: &str, password: &str) -> Result<(), AuthError>;
    async fn generate_oauth_url(&self, provider: ProviderKind) -> Result<(Url, CsrfToken), AuthError>;
    async fn oauth_callback(&self, provider: ProviderKind, code: String, state: String) -> Result<User, AuthError>;
    async fn register(&self, email: &str, password: &str) -> Result<(), AuthError>;
    async fn logout(&self, email: &str) -> Result<(), AuthError>;
}
