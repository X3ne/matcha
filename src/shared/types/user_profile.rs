use apistos::ApiComponent;
use rand_derive2::RandGen;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, sqlx::Type, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema, ApiComponent, RandGen)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "{}", "male"),
            Gender::Female => write!(f, "{}", "female"),
        }
    }
}

#[derive(Debug, sqlx::Type, PartialEq, Eq, Clone, Deserialize, Serialize, JsonSchema, ApiComponent, RandGen)]
#[sqlx(type_name = "sexual_orientation", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    Male,
    Female,
    Bisexual,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Male => write!(f, "{}", "male"),
            Orientation::Female => write!(f, "{}", "female"),
            Orientation::Bisexual => write!(f, "{}", "bisexual"),
        }
    }
}
