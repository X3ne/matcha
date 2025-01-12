use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use apistos::ApiHeader;
use futures::future::{ready, Ready};
use schemars::JsonSchema;

use crate::infrastructure::error::ApiError;

#[derive(Debug, Clone, JsonSchema, ApiHeader)]
#[openapi_header(name = "X-Platform", description = "The client platform")]
pub struct PlatformHeader(String);

impl PlatformHeader {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl FromRequest for PlatformHeader {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("X-Platform") {
            Some(header) => ready(Ok(PlatformHeader(header.to_str().unwrap().to_string()))),
            None => ready(Ok(PlatformHeader("unknown".to_string()))),
        }
    }
}

#[derive(Debug, Clone, JsonSchema, ApiHeader)]
#[openapi_header(name = "X-Matcha-Version", description = "The client app version")]
pub struct VersionHeader(String);

impl VersionHeader {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl FromRequest for VersionHeader {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("X-Matcha-Version") {
            Some(header) => ready(Ok(VersionHeader(header.to_str().unwrap().to_string()))),
            None => ready(Ok(VersionHeader("unknown".to_string()))),
        }
    }
}

#[derive(Debug, Clone, JsonSchema, ApiHeader)]
#[openapi_header(name = "X-Product", description = "The client product")]
pub struct ProductHeader(String);

impl ProductHeader {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl FromRequest for ProductHeader {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.headers().get("X-Product") {
            Some(header) => ready(Ok(ProductHeader(header.to_str().unwrap().to_string()))),
            None => ready(Ok(ProductHeader("unknown".to_string()))),
        }
    }
}
