use std::net::IpAddr;

use serde::Deserialize;

use crate::infrastructure::services::iploc::error::IpLocError;

pub mod error;

#[derive(Debug, Deserialize)]
pub struct IpLocation {
    pub country: String,
    pub region: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[tracing::instrument]
pub async fn locate_ip(ip: IpAddr) -> Result<IpLocation, IpLocError> {
    let client = reqwest::Client::new();
    let location = client
        .get(format!("https://ipapi.co/{}/json", ip))
        .send()
        .await?
        .json::<IpLocation>()
        .await?;

    Ok(location)
}
