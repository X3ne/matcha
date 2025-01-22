use geo_types::Point;
use geozero::wkb;
use sqlx::types::BigDecimal;
use sqlx::FromRow;

use crate::domain::entities::user_profile::UserProfile;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use crate::shared::utils::calculate_age;

#[derive(FromRow, Debug)]
pub struct UserProfileSqlx {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub birth_date: chrono::NaiveDate,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: wkb::Decode<geo_types::Geometry<f64>>,
    pub rating: i32,
    pub last_active: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RecommendedUserProfile {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub birth_date: chrono::NaiveDate,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: wkb::Decode<geo_types::Geometry<f64>>,
    pub rating: i32,
    pub last_active: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub distance: Option<f64>,
    pub common_tags_count: Option<i64>,
    pub inactivity_duration: Option<BigDecimal>,
    pub recommendation_score: Option<f64>,
}

impl Into<UserProfile> for RecommendedUserProfile {
    fn into(self) -> UserProfile {
        UserProfile {
            id: self.id,
            user_id: self.user_id,
            name: self.name,
            avatar_hash: self.avatar_hash,
            picture_hashes: self.picture_hashes,
            bio: self.bio,
            age: calculate_age(self.birth_date),
            birth_date: self.birth_date,
            gender: self.gender,
            sexual_orientation: self.sexual_orientation,
            location: self.location.geometry.unwrap(),
            rating: self.rating,
            last_active: self.last_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct UserProfileInsert {
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub birth_date: chrono::NaiveDate,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: Point,
}

#[derive(Debug, Default)]
pub struct UserProfileUpdate {
    pub name: Option<String>,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
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
