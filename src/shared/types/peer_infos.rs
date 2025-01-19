use actix_web::HttpRequest;
use apistos::ApiHeader;
use schemars::JsonSchema;
use std::net::IpAddr;

#[derive(ApiHeader, JsonSchema, Debug)]
#[openapi_header(name = "X-Product", description = "The product name", required = true)]
pub struct Product(String);

#[derive(ApiHeader, JsonSchema, Debug)]
#[openapi_header(name = "X-Matcha-Version", description = "The version of the product")]
pub struct Version(String);

#[derive(ApiHeader, JsonSchema, Debug)]
#[openapi_header(name = "X-Platform", description = "The platform of the product")]
pub struct Platform(String);

#[derive(Debug)]
pub struct PeerInfos {
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub product: Option<Product>,
    pub version: Option<Version>,
    pub platform: Option<Platform>,
}

impl From<HttpRequest> for PeerInfos {
    fn from(req: HttpRequest) -> Self {
        let ip_address = req.peer_addr().map(|addr| addr.ip());
        let user_agent = req
            .headers()
            .get("User-Agent")
            .map(|ua| ua.to_str().unwrap().to_string());
        let product = req
            .headers()
            .get("X-Product")
            .map(|dt| Product(dt.to_str().unwrap().to_string()));
        let version = req
            .headers()
            .get("X-Matcha-Version")
            .map(|dt| Version(dt.to_str().unwrap().to_string()));
        let platform = req
            .headers()
            .get("X-Platform")
            .map(|dt| Platform(dt.to_str().unwrap().to_string()));

        Self {
            ip_address,
            user_agent,
            product,
            version,
            platform,
        }
    }
}

// impl JsonSchema for crate::presentation::extractors::auth_extractor::Session {
//     fn schema_name() -> String {
//         "Session".to_string()
//     }
//
//     fn json_schema(_: &mut SchemaGenerator) -> Schema {
//         let schema = SchemaObject {
//             instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
//             format: Some("session".to_string()),
//             string: Some(Default::default()),
//             metadata: Some(Box::new(Metadata {
//                 description: Some("The session cookie".to_string()),
//                 examples: vec![Value::String("session=123456".to_string())],
//                 ..Default::default()
//             })),
//             ..Default::default()
//         };
//         schema.into()
//     }
// }
