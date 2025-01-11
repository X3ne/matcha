use std::sync::Arc;

use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use crate::domain::repositories::oauth_account_repo::OAuthAccountRepository;
use crate::domain::repositories::oauth_provider_repo::OAuthProviderRepository;
use crate::domain::repositories::user_repo::UserRepository;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::models::oauth::OAuthAccountInsert;
use crate::infrastructure::models::user::UserInsert;
use crate::infrastructure::repositories::oauth_account_repo::PgOAuthAccountRepository;
use crate::infrastructure::repositories::oauth_provider_repo::PgOAuthProviderRepository;
use crate::infrastructure::repositories::user_repo::PgUserRepository;
use async_trait::async_trait;
use oauth2::client::providers::ft::FtProvider;
use oauth2::client::providers::ProviderKind;
use oauth2::client::{CsrfToken, OAuth2Client, Url};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub pool: Arc<PgPool>,
    pub oauth2_client: Arc<OAuth2Client>,
}

impl AuthServiceImpl {
    pub fn new(pool: Arc<PgPool>, oauth2_client: Arc<OAuth2Client>) -> Self {
        AuthServiceImpl { pool, oauth2_client }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login(&self, email: &str, password: &str) -> Result<(), AuthError> {
        todo!()
    }

    async fn generate_oauth_url(&self, provider: ProviderKind) -> Result<(Url, CsrfToken), AuthError> {
        let (url, csrf_token) = match provider {
            ProviderKind::Ft => self.oauth2_client.authorize::<FtProvider>()?,
        };

        Ok((url, csrf_token))
    }

    async fn oauth_callback(&self, provider: ProviderKind, code: String, state: String) -> Result<User, AuthError> {
        let mut tx = self.pool.begin().await?;

        let oauth_user = match provider {
            // TODO: wrap oauth2 user to a domain user
            ProviderKind::Ft => self.oauth2_client.callback::<FtProvider>(code, state).await?,
        };

        let provider_account =
            PgOAuthAccountRepository::get_by_provider_user_and_provider(&mut *tx, &oauth_user.id.to_string(), provider)
                .await;

        let user = match provider_account {
            Ok(account) => {
                let user = PgUserRepository::get_by_id(&mut *tx, account.user_id).await?;

                if !user.is_active {
                    return Err(AuthError::AccountNotActivated);
                }

                user
            }
            Err(error) => {
                // Handle RowNotFound error by creating a new account
                match error {
                    sqlx::Error::RowNotFound => {
                        let new_user = PgUserRepository::insert(
                            &mut *tx,
                            &UserInsert {
                                email: oauth_user.email.clone(),
                                username: oauth_user.login.clone(),
                                last_name: oauth_user.last_name.clone(),
                                first_name: oauth_user.first_name.clone(),
                                password: None,
                            },
                        )
                        .await?;

                        let provider =
                            PgOAuthProviderRepository::get_by_name(&mut *tx, &ProviderKind::Ft.to_string()).await?;

                        let _ = PgOAuthAccountRepository::insert(
                            &mut *tx,
                            &OAuthAccountInsert {
                                user_id: new_user.id,
                                provider_id: provider.id,
                                provider_user_id: oauth_user.id.to_string(),
                                access_token: None,
                                refresh_token: None,
                                expires_at: None,
                            },
                        )
                        .await?;

                        new_user
                    }
                    _ => return Err(error.into()),
                }
            }
        };

        tx.commit().await?;

        Ok(user)
    }

    async fn register(&self, email: &str, password: &str) -> Result<(), AuthError> {
        todo!()
    }

    async fn logout(&self, email: &str) -> Result<(), AuthError> {
        todo!()
    }
}
