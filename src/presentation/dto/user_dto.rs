use apistos::ApiComponent;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;
use crate::infrastructure::models::user::UserUpdate;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::validation::{validate_password, ValidatePasswordContext};

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

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "UpdateUser"))]
pub struct UpdateUserDto {
    #[garde(email)]
    pub email: Option<String>,
    #[garde(length(min = 3, max = 20), pattern("^[a-zA-Z0-9_-]+$"))]
    pub username: Option<String>,
    #[garde(length(min = 1, max = 50), pattern("^[a-zA-Z]+$"))]
    pub first_name: Option<String>,
    #[garde(length(min = 1, max = 50), pattern("^[a-zA-Z]+$"))]
    pub last_name: Option<String>,
}

impl Into<UserUpdate> for UpdateUserDto {
    fn into(self) -> UserUpdate {
        UserUpdate {
            email: self.email,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[garde(context(ValidatePasswordContext))]
#[serde(rename(deserialize = "ResetPassword"))]
pub struct ResetPasswordDto {
    #[garde(custom(validate_password))]
    pub password: String,
    #[garde(skip)]
    pub token: String,
}
