use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user::{PartialUser, User};
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "User"))]
pub struct UserDto {
    pub id: Snowflake,
    pub email: String,
    pub username: String,
    pub last_name: String,
    pub first_name: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            last_name: user.last_name,
            first_name: user.first_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "PartialUser"))]
pub struct PartialUserDto {
    pub id: Snowflake,
    pub email: String,
}

impl PartialUserDto {
    pub fn from_user(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }

    pub fn from_partial_user(partial_user: PartialUser) -> Self {
        Self {
            id: partial_user.id,
            email: partial_user.email,
        }
    }
}
