use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<geo_types::Geometry<f64>> for Location {
    fn from(geometry: geo_types::Geometry<f64>) -> Self {
        match geometry {
            geo_types::Geometry::Point(point) => Self {
                latitude: point.clone().y(),
                longitude: point.x(),
            },
            _ => panic!("Only points are supported"), // TODO: handle error
        }
    }
}
