use actix_web::HttpRequest;
use std::net::IpAddr;

#[derive(Debug)]
pub struct PeerInfos {
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub product: Option<String>,
    pub version: Option<String>,
    pub platform: Option<String>,
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
            .map(|dt| dt.to_str().unwrap().to_string());
        let version = req
            .headers()
            .get("X-Matcha-Version")
            .map(|dt| dt.to_str().unwrap().to_string());
        let platform = req
            .headers()
            .get("X-Platform")
            .map(|dt| dt.to_str().unwrap().to_string());

        Self {
            ip_address,
            user_agent,
            product,
            version,
            platform,
        }
    }
}
