use geo_types::Point;
use geozero::wkb;
use sqlx::FromRow;

use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};

#[derive(FromRow, Debug)]
pub struct UserProfileSqlx {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: wkb::Decode<geo_types::Geometry<f64>>,
    pub rating: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct UserProfileInsert {
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: Point,
}

#[derive(Debug, Default)]
pub struct UserProfileUpdate {
    pub name: Option<String>,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub sexual_orientation: Option<Orientation>,
    pub location: Option<Point>,
    pub rating: Option<i32>,
}

#[derive(FromRow, Debug)]
pub struct ProfileView {
    pub id: Snowflake,
    pub user_profile_id: Snowflake,
    pub viewer_profile_id: Snowflake,
    pub viewed_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct ProfileViewInsert {
    pub user_profile_id: Snowflake,
    pub viewer_profile_id: Snowflake,
}

#[derive(FromRow, Debug)]
pub struct ProfileLike {
    pub id: Snowflake,
    pub user_profile_id: Snowflake,
    pub liker_profile_id: Snowflake,
    pub liked_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct ProfileLikeInsert {
    pub user_profile_id: Snowflake,
    pub liker_profile_id: Snowflake,
}
