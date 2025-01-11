use crate::infrastructure::models::oauth::OAuthAccountSqlx;
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug)]
pub struct OAuthAccount {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub provider_id: Snowflake,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl OAuthAccount {
    pub fn from_db(db: &OAuthAccountSqlx) -> Self {
        Self {
            id: db.id,
            user_id: db.user_id,
            provider_id: db.provider_id,
            provider_user_id: db.provider_user_id.clone(),
            access_token: db.access_token.clone(),
            refresh_token: db.refresh_token.clone(),
            expires_at: db.expires_at,
            created_at: db.created_at,
            updated_at: db.updated_at,
        }
    }
}
