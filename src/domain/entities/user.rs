use crate::infrastructure::models::user::UserSqlx;
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Clone)]
pub struct User {
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

impl From<UserSqlx> for User {
    fn from(user: UserSqlx) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            last_name: user.last_name,
            first_name: user.first_name,
            password: user.password,
            is_active: user.is_active,
            activation_token: user.activation_token,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
