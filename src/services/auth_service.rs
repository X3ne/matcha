use std::sync::Arc;

use crate::domain::constants::RESET_PASSWORD_TOKEN_TTL;
use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use crate::domain::repositories::oauth_account_repo::OAuthAccountRepository;
use crate::domain::repositories::oauth_provider_repo::OAuthProviderRepository;
use crate::domain::repositories::user_repo::UserRepository;
use crate::domain::services::auth_service::AuthService;
#[cfg(feature = "mailing")]
use crate::infrastructure::mailing::sender::Sender;
use crate::infrastructure::models::oauth::OAuthAccountInsert;
use crate::infrastructure::models::user::UserInsert;
use crate::infrastructure::repositories::oauth_account_repo::PgOAuthAccountRepository;
use crate::infrastructure::repositories::oauth_provider_repo::PgOAuthProviderRepository;
use crate::infrastructure::repositories::user_repo::PgUserRepository;
use crate::shared::utils::generate_random_secure_string;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use oauth2::client::providers::ft::FtProvider;
use oauth2::client::providers::ProviderKind;
use oauth2::client::{CsrfToken, OAuth2Client, Url};
use redis::AsyncCommands;
use sqlx::PgPool;
use tracing::Instrument;

#[derive(Clone)]
pub struct AuthServiceImpl {
    pub pool: Arc<PgPool>,
    pub redis: Arc<redis::Client>,
    pub oauth2_client: Arc<OAuth2Client>,
    #[cfg(feature = "mailing")]
    pub mail_sender: Arc<Sender>,
    pub service_base_url: String,
}

impl AuthServiceImpl {
    pub fn new(
        pool: Arc<PgPool>,
        redis: Arc<redis::Client>,
        oauth2_client: Arc<OAuth2Client>,
        #[cfg(feature = "mailing")] mail_sender: Arc<Sender>,
        service_base_url: String,
    ) -> Self {
        AuthServiceImpl {
            pool,
            redis,
            oauth2_client,
            #[cfg(feature = "mailing")]
            mail_sender,
            service_base_url,
        }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn register(&self, user: &mut UserInsert) -> Result<User, AuthError> {
        let password = match &user.password {
            Some(password) => password,
            None => return Err(AuthError::PasswordRequired),
        };

        let mut tx = self.pool.begin().await?;

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                tracing::error!("Error hashing password: {:?}", e);
                AuthError::PasswordHashError
            })?
            .to_string();

        user.password = Some(password_hash);

        let user = PgUserRepository::insert(&mut *tx, user).await?;

        let confirmation_url = format!(
            "{}/v1/auth/activate?token={}",
            self.service_base_url, user.activation_token
        );

        #[cfg(feature = "mailing")]
        self.mail_sender
            .send_confirmation_mail(&user, confirmation_url)
            .await
            .map_err(|e| {
                tracing::error!("Error sending confirmation mail: {:?}", e);
                AuthError::MailError
            })?;

        tx.commit().await?;

        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn login(&self, username: &str, password: &str) -> Result<User, AuthError> {
        let mut tx = self.pool.begin().await?;

        tracing::info!("Logging in user: {}", username);

        let user = PgUserRepository::get_by_username(&mut *tx, username).await?;

        if !user.is_active {
            return Err(AuthError::AccountNotActivated);
        }

        let argon2 = Argon2::default();

        let stored_password_hash = match &user.password {
            Some(hash) => hash,
            None => return Err(AuthError::InvalidCredentials),
        };

        let parsed_hash = PasswordHash::new(stored_password_hash).map_err(|e| {
            tracing::error!("Error parsing password hash: {:?}", e);
            AuthError::PasswordHashError
        })?;

        argon2.verify_password(password.as_bytes(), &parsed_hash).map_err(|e| {
            tracing::warn!("Password verification failed: {:?}", e);
            AuthError::InvalidCredentials
        })?;

        tx.commit().await?;

        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn generate_oauth_url(&self, provider: ProviderKind) -> Result<(Url, CsrfToken), AuthError> {
        let (url, csrf_token) = match provider {
            ProviderKind::Ft => self.oauth2_client.authorize::<FtProvider>()?,
        };

        Ok((url, csrf_token))
    }

    #[tracing::instrument(skip(self))]
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
                return match error {
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

                        Ok(new_user)
                    }
                    _ => return Err(AuthError::DatabaseError),
                };
            }
        };

        let confirmation_url = format!(
            "{}/v1/auth/activate?token={}",
            self.service_base_url, user.activation_token
        );

        #[cfg(feature = "mailing")]
        self.mail_sender
            .send_confirmation_mail(&user, confirmation_url)
            .await
            .map_err(|e| {
                tracing::error!("Error sending confirmation mail: {:?}", e);
                AuthError::MailError
            })?;

        // Send error to client to ask for validation
        tx.commit().await?;
        Err(AuthError::AccountNotActivated)
    }

    #[tracing::instrument(skip(self))]
    async fn activate_account(&self, token: String) -> Result<(), AuthError> {
        let mut tx = self.pool.begin().await?;

        PgUserRepository::activate(&mut *tx, token).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn request_password_reset(&self, email: &str) -> Result<(), AuthError> {
        let reset_token = generate_random_secure_string(32);

        // store to redis
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        let key = format!("password_reset:{}", reset_token);
        conn.set_ex(&key, email, RESET_PASSWORD_TOKEN_TTL).await?;

        #[cfg(feature = "mailing")]
        self.mail_sender
            .send_password_reset_mail(email, &reset_token)
            .await
            .map_err(|e| {
                tracing::error!("Error sending confirmation mail: {:?}", e);
                AuthError::MailError
            })?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn reset_password(&self, token: &str, new_password: &str) -> Result<(), AuthError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        let key = format!("password_reset:{}", token);
        let email: String = conn.get(&key).await?;

        if email.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let mut tx = self.pool.begin().await?;

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| {
                tracing::error!("Error hashing password: {:?}", e);
                AuthError::PasswordHashError
            })?
            .to_string();

        PgUserRepository::update_password(&mut *tx, &email, &password_hash).await?;

        tx.commit().await?;

        Ok(())
    }
}
