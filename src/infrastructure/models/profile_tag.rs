use crate::shared::types::snowflake::Snowflake;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ProfileTagSqlx {
    pub id: Snowflake,
    pub name: String,
}

#[derive(Debug)]
pub struct ProfileTagInsert {
    pub name: String,
}

#[derive(FromRow, Debug)]
#[allow(dead_code)]
pub struct UserProfileTagSqlx {
    pub id: Snowflake,
    pub user_profile_id: Snowflake,
    pub profile_tag_id: Snowflake,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UserProfileTagInsert {
    pub user_profile_id: Snowflake,
    pub profile_tag_id: Snowflake,
}
