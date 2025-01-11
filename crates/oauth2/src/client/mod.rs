use std::collections::HashMap;

use crate::client::providers::{AnyProvider, Provider, ProviderKind};
use crate::error::{OAuth2Error, Result};

pub mod providers;
pub use oauth2::url::Url;
pub use oauth2::CsrfToken;

#[derive(Debug)]
pub struct OAuth2ClientBuilder {
    providers: HashMap<ProviderKind, Box<dyn AnyProvider>>,
}

impl OAuth2ClientBuilder {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn add_provider<P>(&mut self, provider: P) -> &mut Self
    where
        P: Provider + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        self.providers.insert(P::get_provider_kind(), Box::new(provider));

        self
    }

    pub fn build(&self) -> OAuth2Client {
        OAuth2Client {
            providers: self.providers.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuth2Client {
    providers: HashMap<ProviderKind, Box<dyn AnyProvider>>,
}

impl OAuth2Client {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn get_provider<P>(&self, provider: &ProviderKind) -> Result<&P>
    where
        P: Provider + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        Ok(self
            .providers
            .get(&provider)
            .ok_or(OAuth2Error::InvalidProvider)?
            .as_any()
            .downcast_ref::<P>()
            .ok_or(OAuth2Error::InvalidProvider)?)
    }

    pub fn authorize<P>(&self) -> Result<(Url, CsrfToken)>
    where
        P: Provider + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        let provider = self.get_provider::<P>(&P::get_provider_kind())?;

        let (url, csrf_token) = provider.authorize();

        Ok((url, csrf_token))
    }

    pub async fn callback<P: 'static>(&self, code: String, state: String) -> Result<P::CallbackResponse>
    where
        P: Provider + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        let provider = self.get_provider::<P>(&P::get_provider_kind())?;

        let response = provider.callback(code, state).await?;

        Ok(response)
    }
}
