use sqlx::FromRow;

use crate::shared::types::snowflake::Snowflake;

#[derive(FromRow, Debug)]
pub struct OAuthProviderSqlx {
    pub id: Snowflake,
    pub name: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct OAuthProviderInsert {
    pub name: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct OAuthProviderUpdate {
    pub name: Option<String>,
    pub active: Option<bool>,
}

#[derive(FromRow, Debug)]
pub struct OAuthAccountSqlx {
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

#[derive(Debug)]
pub struct OAuthAccountInsert {
    pub user_id: Snowflake,
    pub provider_id: Snowflake,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
pub struct OAuthAccountUpdate {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<chrono::NaiveDateTime>,
}
