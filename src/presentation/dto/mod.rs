use crate::presentation::dto::user_profile::{CompleteOnboardingDto, CompleteOnboardingForm};
use crate::shared::types::location::Location;
use apistos::reference_or::ReferenceOr;
use apistos::{ApiComponent, TypedSchema};
use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec};
use schemars::Map;

pub mod auth_dto;
pub mod user_dto;
pub mod user_profile;

impl TypedSchema for CompleteOnboardingForm {
    fn schema_type() -> InstanceType {
        InstanceType::Object
    }

    fn format() -> Option<String> {
        None
    }
}

impl ApiComponent for CompleteOnboardingForm {
    fn content_type() -> String {
        "multipart/form-data".to_string()
    }

    fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        vec![CompleteOnboardingDto::schema().unwrap(), Location::schema().unwrap()]
    }

    fn schema() -> Option<(String, ReferenceOr<Schema>)> {
        Some((
            "CompleteOnboardingForm".to_string(),
            ReferenceOr::Object(Schema::Object(SchemaObject {
                instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Object))),
                object: Some(Box::new(ObjectValidation {
                    properties: {
                        let mut map = Map::new();
                        map.insert(
                            "pictures".to_string(),
                            Schema::Object(SchemaObject {
                                instance_type: Some(SingleOrVec::Vec(vec![InstanceType::String])),
                                format: Some("binary".to_string()),
                                ..Default::default()
                            }),
                        );
                        map.insert(
                            "profile".to_string(),
                            Schema::new_ref(format!(
                                "#/components/schemas/{}",
                                CompleteOnboardingDto::schema().unwrap().0
                            )),
                        );
                        map
                    },
                    ..Default::default()
                })),
                ..Default::default()
            })),
        ))
    }
}
