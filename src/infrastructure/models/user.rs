use sqlx::FromRow;

use crate::shared::types::snowflake::Snowflake;

#[derive(FromRow, Debug, Clone)]
pub struct UserSqlx {
    pub id: Snowflake,
    pub email: String,
    pub username: String,
    pub last_name: String,
    pub first_name: String,
    pub password: Option<String>,
    pub is_active: bool,
    pub activation_token: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct UserInsert {
    pub email: String,
    pub username: String,
    pub last_name: String,
    pub first_name: String,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub username: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub password: Option<String>,
}
