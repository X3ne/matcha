use crate::infrastructure::models::user_profile::UserProfileSqlx;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::tag::Tag;
use crate::shared::types::user_profile::{Gender, Orientation};

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: geo_types::Geometry<f64>,
    pub tags: Vec<Tag>,
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
            bio: profile.bio,
            age: profile.age,
            gender: profile.gender,
            sexual_orientation: profile.sexual_orientation,
            location: profile.location.geometry.unwrap(), // TODO: Fix unsafe unwrap
            tags: vec![],
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}
