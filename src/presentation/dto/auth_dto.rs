use crate::infrastructure::models::user::UserInsert;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
#[serde(rename(deserialize = "RegisterUser"))]
pub struct RegisterUserDto {
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
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

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
#[serde(rename(deserialize = "Login"))]
pub struct LoginDto {
    pub username: String,
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
