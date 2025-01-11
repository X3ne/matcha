use async_trait::async_trait;
use oauth2::CsrfToken;
use oauth2::url::Url;

use crate::error::Result;

pub mod ft;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ProviderKind {
    Ft,
}

impl std::fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderKind::Ft => write!(f, "ft"),
        }
    }
}

#[async_trait]
pub trait Provider: Send + Sync + Clone {
    type CallbackResponse: std::any::Any + Send;

    fn get_provider_kind() -> ProviderKind;
    fn authorize(&self) -> (Url, CsrfToken);
    async fn callback(&self, code: String, state: String) -> Result<Self::CallbackResponse>;
}

pub(super) trait AnyProvider: std::fmt::Debug + Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn AnyProvider>;
}

impl<T: Provider + std::fmt::Debug + Send + Sync + 'static> AnyProvider for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn AnyProvider> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AnyProvider> {
    fn clone(&self) -> Box<dyn AnyProvider> {
        self.clone_box()
    }
}
