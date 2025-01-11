use sqlx::PgPool;

use crate::domain::repositories::oauth_provider_repo::OAuthProviderRepository;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::oauth::OAuthProviderInsert;
use crate::infrastructure::repositories::oauth_provider_repo::PgOAuthProviderRepository;

pub async fn create_default_providers(pool: &PgPool) -> Result<(), ApiError> {
    let providers = vec![OAuthProviderInsert {
        name: "ft".to_string(),
        active: true,
    }];

    for provider in providers {
        if let Err(err) = PgOAuthProviderRepository::insert(pool, &provider).await {
            match &err {
                sqlx::Error::Database(e) => {
                    if let Some(constraint) = e.constraint() {
                        tracing::debug!("Provider already exists: {}", constraint);
                    } else {
                        tracing::error!("Failed to create default provider: {}", e);
                        return Err(ApiError::DatabaseError(err));
                    }
                }
                _ => {
                    return Err(ApiError::DatabaseError(err));
                }
            }
        }
    }

    Ok(())
}
