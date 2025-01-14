use crate::infrastructure::models::user::UserInsert;
use apistos::ApiComponent;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "RegisterUser"))]
pub struct RegisterUserDto {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 3, max = 20), pattern("^[a-zA-Z0-9_-]+$"))]
    pub username: String,
    #[garde(pattern("^[a-zA-Z]+$"))]
    pub first_name: String,
    #[garde(pattern("^[a-zA-Z]+$"))]
    pub last_name: String,
    #[garde(length(min = 8))]
    pub password: String,
}

impl Into<UserInsert> for RegisterUserDto {
    fn into(self) -> UserInsert {
        UserInsert {
            email: self.email,
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            password: Some(self.password),
        }
    }
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "Login"))]
pub struct LoginDto {
    #[garde(length(min = 3, max = 20), pattern("^[a-zA-Z0-9_-]+$"))]
    pub username: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
#[serde(rename(deserialize = "OAuthCallbackQuery"))]
pub struct OAuthCallbackQueryDto {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Debug, ApiComponent, JsonSchema)]
#[serde(rename(deserialize = "OAuthResponse"))]
pub struct OAuthResponseDto {
    pub url: String,
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
#[serde(rename(deserialize = "ActivateAccount"))]
pub struct ActivateAccountDto {
    pub token: String,
}
