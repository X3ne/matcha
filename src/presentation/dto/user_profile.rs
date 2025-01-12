use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user_profile::UserProfile;
use crate::shared::types::location::Location;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::tag::Tag;
use crate::shared::types::user_profile::{Gender, Orientation};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "CompleteOnboarding"))]
pub struct CompleteOnboardingDto {
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: Option<Location>,
    #[serde(default)]
    pub tag_ids: Vec<Snowflake>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "UserProfile"))]
pub struct UserProfileDto {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub tags: Vec<Tag>,
}

impl From<UserProfile> for UserProfileDto {
    fn from(user: UserProfile) -> Self {
        Self {
            id: user.id,
            user_id: user.user_id,
            name: user.name,
            avatar_hash: user.avatar_hash,
            bio: user.bio,
            age: user.age,
            gender: user.gender,
            sexual_orientation: user.sexual_orientation,
            tags: user.tags,
        }
    }
}
