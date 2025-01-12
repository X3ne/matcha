use geo_types::Point;
use geozero::wkb;
use sqlx::error::BoxDynError;
use sqlx::FromRow;

use crate::domain::entities::user_profile::UserProfile;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::tag::Tag;
use crate::shared::types::user_profile::{Gender, Orientation};

#[derive(FromRow, Debug)]
pub struct UserProfileSqlx {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: wkb::Decode<geo_types::Geometry<f64>>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug)]
pub struct RawProfileWithTag {
    pub profile_id: Snowflake,
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: wkb::Decode<geo_types::Geometry<f64>>,
    pub tag_id: Option<Snowflake>,
    pub tag_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryInto<UserProfile> for Vec<RawProfileWithTag> {
    type Error = sqlx::Error;

    fn try_into(self) -> Result<UserProfile, sqlx::Error> {
        let first_row = self.first().ok_or(sqlx::Error::Decode(
            sqlx::error::Error::Decode(BoxDynError::from("No rows returned")).into(),
        ))?;

        let tags: Vec<Tag> = self
            .iter()
            .filter_map(|row| {
                row.tag_id.map(|id| Tag {
                    id,
                    name: row.tag_name.clone().unwrap(),
                })
            })
            .collect();

        Ok(UserProfile {
            id: first_row.profile_id,
            user_id: first_row.user_id,
            name: first_row.name.clone(),
            avatar_hash: first_row.avatar_hash.clone(),
            bio: first_row.bio.clone(),
            age: first_row.age,
            gender: first_row.gender.clone(),
            sexual_orientation: first_row.sexual_orientation.clone(),
            location: first_row.location.geometry.clone().unwrap(), // TODO: Fix unsafe unwrap
            tags,
            created_at: first_row.created_at,
            updated_at: first_row.updated_at,
        })
    }
}

#[derive(Debug)]
pub struct UserProfileInsert {
    pub user_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub location: Point,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct UserProfileUpdate {
    pub name: Option<String>,
    pub avatar_hash: Option<String>,
    pub bio: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub sexual_orientation: Option<Orientation>,
    pub location: Option<Point>,
}

#[derive(FromRow, Debug)]
pub struct ProfileTagSqlx {
    pub id: Snowflake,
    pub name: String,
}

#[derive(FromRow, Debug)]
pub struct UserProfileTagSqlx {
    pub id: Snowflake,
    pub user_profile_id: Snowflake,
    pub profile_tag_id: Snowflake,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct UserProfileTagInsert {
    pub user_profile_id: Snowflake,
    pub profile_tag_id: Snowflake,
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
