use apistos::ApiComponent;
use garde::Validate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::infrastructure::models::user::UserInsert;
use crate::shared::utils::validation::{validate_password, ValidatePasswordContext};

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[garde(context(ValidatePasswordContext))]
#[serde(rename(deserialize = "RegisterUser"))]
pub struct RegisterUserDto {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 3, max = 20), pattern("^[a-zA-Z0-9_-]+$"))]
    pub username: String,
    #[garde(length(min = 1, max = 50), pattern("^[a-zA-Z]+$"))]
    pub first_name: String,
    #[garde(length(min = 1, max = 50), pattern("^[a-zA-Z]+$"))]
    pub last_name: String,
    #[garde(custom(validate_password))]
    pub password: String,
    #[garde(matches(password))]
    pub confirm_password: String,
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
    #[garde(skip)]
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

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[serde(rename(deserialize = "RequestResetPassword"))]
pub struct RequestResetPasswordDto {
    #[garde(email)]
    pub email: String,
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema, Validate)]
#[garde(context(ValidatePasswordContext))]
#[serde(rename(deserialize = "ResetPassword"))]
pub struct ResetPasswordDto {
    #[garde(email)]
    pub email: String,
    #[garde(custom(validate_password))]
    pub password: String,
    #[garde(matches(password))]
    pub confirm_password: String,
    #[garde(skip)]
    pub token: String,
}
