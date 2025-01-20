use async_trait::async_trait;
use oauth2::client::providers::ProviderKind;
use oauth2::client::{CsrfToken, Url};

use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use crate::infrastructure::models::user::UserInsert;

#[async_trait]
pub trait AuthService: 'static + Sync + Send {
    async fn register(&self, user: &mut UserInsert) -> Result<User, AuthError>;
    async fn login(&self, username: &str, password: &str) -> Result<User, AuthError>;
    async fn generate_oauth_url(&self, provider: ProviderKind) -> Result<(Url, CsrfToken), AuthError>;
    async fn oauth_callback(&self, provider: ProviderKind, code: String, state: String) -> Result<User, AuthError>;
    async fn activate_account(&self, token: String) -> Result<(), AuthError>;
    async fn request_password_reset(&self, email: &str, reset_url: &str) -> Result<(), AuthError>;
    async fn reset_password(&self, email: &str, new_password: &str, token: &str) -> Result<(), AuthError>;
}
