// use geo_types::Point;
// use sqlx::encode::IsNull;
// use sqlx::error::BoxDynError;
// use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
// use sqlx::{Decode, Encode, Postgres, Type};
//
// #[derive(Debug, PartialEq, Clone)]
// pub struct GeoPoint(Point<f64>);
//
// impl GeoPoint {
//     pub fn new(x: f64, y: f64) -> Self {
//         Self(Point::new(x, y))
//     }
//
//     pub fn x(&self) -> f64 {
//         self.0.x()
//     }
//
//     pub fn y(&self) -> f64 {
//         self.0.y()
//     }
//
//     pub fn into_inner(self) -> Point<f64> {
//         self.0
//     }
//
//     pub fn from_inner(point: Point<f64>) -> Self {
//         Self(point)
//     }
// }
//
// impl Type<Postgres> for GeoPoint {
//     fn type_info() -> PgTypeInfo {
//         PgTypeInfo::with_name("geometry")
//     }
// }
//
// impl<'r> Decode<'r, Postgres> for GeoPoint {
//     fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
//         let wkt: String = Decode::<Postgres>::decode(value)?;
//         let wkt = wkt.trim();
//
//         if !wkt.starts_with("POINT(") || !wkt.ends_with(")") {
//             return Err("Invalid WKT format for POINT".into());
//         }
//
//         let coords = wkt[6..wkt.len() - 1] // Remove "POINT(" and ")"
//             .split_whitespace()
//             .map(str::parse::<f64>)
//             .collect::<Result<Vec<_>, _>>()?;
//
//         if coords.len() != 2 {
//             return Err("Invalid POINT data: expected two coordinates".into());
//         }
//
//         Ok(GeoPoint(Point::new(coords[0], coords[1])))
//     }
// }
//
// impl Encode<'_, Postgres> for GeoPoint {
//     fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
//         let wkt = format!("POINT({} {})", self.0.x(), self.0.y());
//         Encode::<Postgres>::encode_by_ref(&wkt, buf)
//     }
//
//     fn size_hint(&self) -> usize {
//         format!("POINT({} {})", self.0.x(), self.0.y()).len()
//     }
// }
