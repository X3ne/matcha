use std::sync::Arc;

use oauth2::client::providers::ft::FtProvider;
use oauth2::client::OAuth2ClientBuilder;

use crate::config::{OAuth2Config, SmtpConfig};
use crate::domain::services::auth_service::AuthService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::databases::postgresql::connection::connect;
use crate::infrastructure::databases::postgresql::init::create_default_providers;
use crate::infrastructure::mailing::sender::Sender;
use crate::services::auth_service::AuthServiceImpl;
use crate::services::user_profile_service::UserProfileServiceImpl;
use crate::services::user_service::UserServiceImpl;

pub struct Container {
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
    pub user_profile_service: Arc<dyn UserProfileService>,
    pub pool: Arc<sqlx::PgPool>,
}

impl Container {
    pub async fn new(database_url: &str) -> Self {
        let pool = connect(database_url).await.expect("Failed to connect to database");
        let pool = Arc::new(pool);

        create_default_providers(&pool)
            .await
            .expect("Failed to create default providers");

        let oauth_config = OAuth2Config::from_env().expect("Failed to load oauth2 configuration");

        let oauth_client = OAuth2ClientBuilder::new()
            .add_provider(FtProvider::new(
                oauth_config.ft_client_id.clone(),
                oauth_config.ft_client_secret.clone(),
                oauth_config.ft_redirect_uri.clone(),
            ))
            .build();
        let oauth_client = Arc::new(oauth_client);

        let smtp_config = SmtpConfig::from_env().expect("Failed to load smtp configuration");
        let mail_sender = Arc::new(Sender::new(smtp_config).expect("Failed to create mail sender"));

        let auth_service = Arc::new(AuthServiceImpl {
            pool: Arc::clone(&pool),
            oauth2_client: Arc::clone(&oauth_client),
            mail_sender: Arc::clone(&mail_sender),
        });

        let user_service = Arc::new(UserServiceImpl {
            pool: Arc::clone(&pool),
        });

        let user_profile_service = Arc::new(UserProfileServiceImpl {
            pool: Arc::clone(&pool),
        });

        Container {
            auth_service,
            user_service,
            user_profile_service,
            pool,
        }
    }
}
