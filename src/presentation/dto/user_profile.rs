use actix_multipart::form::json::Json;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use apistos::ApiComponent;
use garde::Validate;
use geo_types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::constants::MAX_PROFILE_IMAGES;
use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::repository::DEFAULT_LIMIT;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileSortBy};
use crate::shared::types::filtering::SortOrder;
use crate::shared::types::location::Location;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use crate::shared::utils::build_cdn_profile_image_uri;

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "CompleteOnboarding"))]
pub struct CompleteOnboardingDto {
    #[garde(length(min = 1, max = 50), ascii)]
    pub name: String,
    #[garde(length(min = 1, max = 150))]
    pub bio: Option<String>,
    #[garde(range(min = 18, max = 100))]
    pub age: i32,
    #[serde(default = "default_index")]
    #[garde(range(min = 0, max = MAX_PROFILE_IMAGES))]
    pub avatar_index: usize,
    #[garde(skip)]
    pub gender: Gender,
    #[serde(default = "Orientation::default")]
    #[garde(skip)]
    pub sexual_orientation: Orientation,
    #[garde(dive)]
    pub location: Option<Location>,
}

fn default_index() -> usize {
    0
}

#[derive(Debug, MultipartForm)]
#[multipart(duplicate_field = "deny")]
pub struct CompleteOnboardingForm {
    #[multipart(limit = "100MB")]
    pub pictures: Vec<TempFile>,
    pub profile: Json<CompleteOnboardingDto>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "UserProfile"))]
pub struct UserProfileDto {
    pub id: Snowflake,
    pub name: String,
    pub avatar_url: Option<String>,
    pub picture_urls: Vec<String>,
    pub bio: Option<String>,
    pub age: i32,
    pub gender: Gender,
    pub sexual_orientation: Orientation,
    pub rating: i32,
    pub tags: Vec<ProfileTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_distance_km: Option<u64>,
}

impl From<UserProfile> for UserProfileDto {
    fn from(user: UserProfile) -> Self {
        Self {
            id: user.id,
            name: user.name,
            avatar_url: user.avatar_hash.map(|hash| build_cdn_profile_image_uri(&hash)),
            picture_urls: user
                .picture_hashes
                .iter()
                .map(|hash| build_cdn_profile_image_uri(hash))
                .collect(),
            bio: user.bio,
            age: user.age,
            gender: user.gender,
            sexual_orientation: user.sexual_orientation,
            rating: user.rating,
            tags: vec![],
            approx_distance_km: None,
        }
    }
}

impl UserProfileDto {
    pub fn append_tags(&mut self, tags: Vec<ProfileTag>) {
        self.tags = tags;
    }

    pub fn set_approx_distance(&mut self, distance: f64) {
        self.approx_distance_km = Some(distance as u64);
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "UserProfileQueryParams"))]
pub struct UserProfileQueryParamsDto {
    #[garde(range(min = 1, max = 100))]
    #[serde(default = "limit_default")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,

    #[garde(range(min = 18, max = 100))]
    pub min_age: Option<i32>,
    #[garde(custom(validate_age(self.min_age, self.max_age)))]
    pub max_age: Option<i32>,

    #[garde(range(min = 0, max = 100))]
    pub min_fame_rating: Option<i32>,
    #[garde(custom(validate_fame_rating(self.min_fame_rating, self.max_fame_rating)))]
    pub max_fame_rating: Option<i32>,

    #[garde(range(min = -90.0, max = 90.0))]
    pub latitude: Option<f64>,
    #[garde(range(min = -180.0, max = 180.0))]
    pub longitude: Option<f64>,
    #[garde(range(min = 0.0))]
    pub radius_km: Option<f64>,
    #[garde(dive)]
    pub tag_ids: Option<Vec<Snowflake>>,

    #[garde(skip)]
    pub sort_by: Option<UserProfileSortBy>,
    #[garde(skip)]
    pub sort_order: Option<SortOrder>,
}

fn limit_default() -> i64 {
    DEFAULT_LIMIT.unwrap_or(50)
}

fn validate_age(min: Option<i32>, max: Option<i32>) -> impl FnOnce(&Option<i32>, &()) -> garde::Result {
    move |_, _| {
        if min.is_none() || max.is_none() {
            return Ok(());
        }

        if max < min {
            return Err(garde::Error::new("Max age is less than min age"));
        }

        Ok(())
    }
}

fn validate_fame_rating(min: Option<i32>, max: Option<i32>) -> impl FnOnce(&Option<i32>, &()) -> garde::Result {
    move |_, _| {
        if min.is_none() || max.is_none() {
            return Ok(());
        }

        if max < min {
            return Err(garde::Error::new("Max fame rating is less than min fame rating"));
        }

        Ok(())
    }
}

impl Into<UserProfileQueryParams> for UserProfileQueryParamsDto {
    fn into(self) -> UserProfileQueryParams {
        UserProfileQueryParams {
            limit: Some(self.limit),
            offset: Some(self.offset),
            min_age: self.min_age,
            max_age: self.max_age,
            min_fame_rating: self.min_fame_rating,
            max_fame_rating: self.max_fame_rating,
            location: self
                .latitude
                .and_then(|lat| self.longitude.map(|lng| Point::new(lat, lng))),
            radius_km: self.radius_km,
            tag_ids: self.tag_ids,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        }
    }
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "UpdateProfile"))]
pub struct UpdateProfileDto {
    #[garde(length(min = 1, max = 50), ascii)]
    pub name: Option<String>,
    #[garde(length(min = 1, max = 150))]
    pub bio: Option<String>,
    #[garde(range(min = 18, max = 100))]
    pub age: Option<i32>,
    #[garde(range(min = 0, max = MAX_PROFILE_IMAGES))]
    pub avatar_index: Option<usize>,
    #[garde(skip)]
    pub gender: Option<Gender>,
    #[garde(skip)]
    pub sexual_orientation: Option<Orientation>,
    #[garde(dive)]
    pub location: Option<Location>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "UserProfileTagParams"))]
pub struct UserProfileTagParamsDto {
    #[garde(dive)]
    pub tag_id: Snowflake,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "UserProfileBulkTags"))]
pub struct UserProfileBulkTagsDto {
    #[garde(dive)]
    pub tag_ids: Vec<Snowflake>,
}
