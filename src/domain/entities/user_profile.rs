use crate::infrastructure::models::user_profile::UserProfileSqlx;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use crate::shared::utils::calculate_age;

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub picture_hashes: Vec<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub birth_date: chrono::NaiveDate,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: geo_types::Geometry<f64>,
    pub rating: i32,
    pub last_active: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<UserProfileSqlx> for UserProfile {
    fn from(profile: UserProfileSqlx) -> Self {
        Self {
            id: profile.id,
            user_id: profile.user_id,
            name: profile.name,
            avatar_hash: profile.avatar_hash,
            picture_hashes: profile.picture_hashes,
            bio: profile.bio,
            age: calculate_age(profile.birth_date),
            birth_date: profile.birth_date,
            gender: profile.gender,
            sexual_orientation: profile.sexual_orientation,
            location: profile.location.geometry.unwrap(), // TODO: Fix unsafe unwrap
            rating: profile.rating,
            last_active: profile.updated_at,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}
