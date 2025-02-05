use std::sync::Arc;

use oauth2::client::providers::ft::FtProvider;
use oauth2::client::OAuth2ClientBuilder;

#[cfg(feature = "mailing")]
use crate::config::SmtpConfig;
use crate::config::{OAuth2Config, S3Config};
use crate::domain::services::auth_service::AuthService;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::chat_service::ChatService;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::databases::postgresql::connection::connect;
use crate::infrastructure::databases::postgresql::init::create_default_providers;
use crate::infrastructure::gateway::Gateway;
#[cfg(feature = "mailing")]
use crate::infrastructure::mailing::sender::Sender;
use crate::infrastructure::s3::S3Service;
use crate::services::auth_service::AuthServiceImpl;
use crate::services::cdn_service::CdnServiceImpl;
use crate::services::chat_service::ChatServiceImpl;
use crate::services::profile_tag_service::ProfileTagServiceImpl;
use crate::services::user_profile_service::UserProfileServiceImpl;
use crate::services::user_service::UserServiceImpl;

pub struct Container {
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
    pub user_profile_service: Arc<dyn UserProfileService>,
    pub profile_tag_service: Arc<dyn ProfileTagService>,
    pub cdn_service: Arc<dyn CdnService>,
    pub chat_service: Arc<dyn ChatService>,
    pub s3: Arc<S3Service>,
    pub pool: Arc<sqlx::PgPool>,
    pub redis: Arc<redis::Client>,
    pub gateway: Arc<Gateway>,
}

impl Container {
    pub async fn new(database_url: &str, redis_url: &str, service_base_url: &str) -> Self {
        let pool = connect(database_url).await.expect("Failed to connect to database");
        let pool = Arc::new(pool);

        create_default_providers(&pool)
            .await
            .expect("Failed to create default providers");

        // Redis
        let redis = redis::Client::open(redis_url).expect("Failed to connect to redis");
        let redis = Arc::new(redis);

        // OAuth2
        let oauth_config = OAuth2Config::from_env().expect("Failed to load oauth2 configuration");

        let oauth_client = OAuth2ClientBuilder::new()
            .add_provider(FtProvider::new(
                oauth_config.ft_client_id.clone(),
                oauth_config.ft_client_secret.clone(),
                oauth_config.ft_redirect_uri.clone(),
            ))
            .build();
        let oauth_client = Arc::new(oauth_client);

        // Mailing
        #[cfg(feature = "mailing")]
        let smtp_config = SmtpConfig::from_env().expect("Failed to load smtp configuration");
        #[cfg(feature = "mailing")]
        let mail_sender = Arc::new(Sender::new(smtp_config).expect("Failed to create mail sender"));

        // S3
        let s3_config = S3Config::from_env().expect("Failed to load s3 configuration");

        let s3 = Arc::new(
            S3Service::new(s3_config)
                .await
                .expect("Failed to connect to s3 service"),
        );

        // Gateway
        let gateway = Arc::new(Gateway::new());

        // Services
        let auth_service = Arc::new(AuthServiceImpl::new(
            Arc::clone(&pool),
            Arc::clone(&redis),
            Arc::clone(&oauth_client),
            #[cfg(feature = "mailing")]
            Arc::clone(&mail_sender),
            service_base_url.to_string(),
        ));

        let user_service = Arc::new(UserServiceImpl::new(Arc::clone(&pool)));
        let user_profile_service = Arc::new(UserProfileServiceImpl::new(Arc::clone(&pool), Arc::clone(&redis)));
        let profile_tag_service = Arc::new(ProfileTagServiceImpl::new(Arc::clone(&pool)));
        let cdn_service = Arc::new(CdnServiceImpl::new(Arc::clone(&s3), Arc::clone(&pool)));
        let chat_service = Arc::new(ChatServiceImpl::new(Arc::clone(&pool)));

        Container {
            auth_service,
            user_service,
            user_profile_service,
            profile_tag_service,
            cdn_service,
            chat_service,
            s3,
            pool,
            redis,
            gateway,
        }
    }
}
