use crate::config::OAuth2Config;
use crate::domain::services::auth_service::AuthService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::databases::postgresql::connection::connect;
use crate::services::auth_service::AuthServiceImpl;
use crate::services::user_service::UserServiceImpl;
use oauth2::client::providers::ft::FtProvider;
use oauth2::client::OAuth2ClientBuilder;
use std::sync::Arc;

pub struct Container {
    pub user_service: Arc<dyn UserService>,
    pub auth_service: Arc<dyn AuthService>,
    pub pool: Arc<sqlx::PgPool>,
}

impl Container {
    pub async fn new(database_url: &str) -> Self {
        let pool = connect(database_url).await.expect("Failed to connect to database");
        let pool = Arc::new(pool);

        let oauth_config = OAuth2Config::from_env().expect("Failed to load oauth2 configuration");

        let oauth_client = OAuth2ClientBuilder::new()
            .add_provider(FtProvider::new(
                oauth_config.ft_client_id.clone(),
                oauth_config.ft_client_secret.clone(),
                oauth_config.ft_redirect_uri.clone(),
            ))
            .build();
        let oauth_client = Arc::new(oauth_client);

        let user_service = Arc::new(UserServiceImpl {
            pool: Arc::clone(&pool),
        });

        let auth_service = Arc::new(AuthServiceImpl {
            pool: Arc::clone(&pool),
            oauth2_client: Arc::clone(&oauth_client),
        });

        Container {
            user_service,
            auth_service,
            pool,
        }
    }
}
