use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct Tag {
    pub id: Snowflake,
    pub name: String,
}
