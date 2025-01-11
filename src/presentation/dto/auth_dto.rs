use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Debug, ApiComponent, JsonSchema)]
pub struct OAuthResponse {
    pub url: String,
}
