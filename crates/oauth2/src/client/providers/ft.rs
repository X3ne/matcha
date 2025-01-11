use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    http, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::client::providers::{Provider, ProviderKind};
use crate::error::{OAuth2Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtUser {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub login: String,
}

#[derive(Debug, Clone)]
pub struct FtProvider {
    client: BasicClient,
}

impl FtProvider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let auth_url =
            AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string()).expect("Failed to parse auth URL");
        let token_url =
            TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string()).expect("Failed to parse token URL");
        let redirect_url = RedirectUrl::new(redirect_uri).expect("Failed to parse redirect URL");

        let client =
            BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url)).set_redirect_uri(redirect_url);

        FtProvider { client }
    }
}

#[async_trait]
impl Provider for FtProvider {
    type CallbackResponse = FtUser;

    fn get_provider_kind() -> ProviderKind {
        ProviderKind::Ft
    }

    fn authorize(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public".to_string()))
            .url()
    }

    async fn callback(&self, code: String, _state: String) -> Result<FtUser> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| {
                tracing::error!("Failed to exchange code: {:?}", e);
                OAuth2Error::FailedToRequestToken
            })?;

        let mut headers = http::HeaderMap::new();
        headers.insert(
            http::header::AUTHORIZATION,
            http::HeaderValue::from_str(&format!("Bearer {}", token.access_token().secret())).unwrap(),
        );

        let res = async_http_client(oauth2::HttpRequest {
            url: Url::parse("https://api.intra.42.fr/v2/me").unwrap(),
            method: http::method::Method::GET,
            headers,
            body: vec![],
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to request user info: {:?}", e);
            OAuth2Error::FailedToRequestUserInfo
        })?;

        if !res.status_code.is_success() {
            tracing::error!("Failed to request user info: {:?}", res);
            return Err(OAuth2Error::FailedToRequestUserInfo);
        }

        serde_json::from_slice::<FtUser>(&res.body).map_err(|e| {
            tracing::error!("Failed to parse user info: {:?}", e);
            OAuth2Error::FailedToRequestUserInfo
        })
    }
}
