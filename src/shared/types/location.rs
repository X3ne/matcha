use apistos::ApiComponent;
use garde::Validate;
use geo_types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
pub struct Location {
    #[garde(range(min = -90.0, max = 90.0))]
    pub latitude: f64,
    #[garde(range(min = -180.0, max = 180.0))]
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

impl Into<Point> for Location {
    fn into(self) -> Point<f64> {
        Point::new(self.longitude, self.latitude)
    }
}
