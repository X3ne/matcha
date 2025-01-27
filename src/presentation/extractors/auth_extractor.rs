use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use actix_session::{Session as ActixSession, SessionExt};
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use apistos::ApiSecurity;
use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::error::ApiError;

const ALLOWED_PATHS: [&str; 2] = ["/v1/auth/oauth2/42/callback", "/v1/auth/login"];

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(name = "session", api_key_in = "cookie"))))]
pub struct Session {
    inner: ActixSession,
    user: Option<User>,
}

#[allow(dead_code)]
impl Session {
    pub fn inner(&self) -> &ActixSession {
        &self.inner
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn authenticated_user(&self) -> Result<&User, ApiError> {
        self.user.as_ref().ok_or(AuthError::Unauthorized.into())
    }
}

#[async_trait(?Send)]
impl FromRequest for Session {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        if ALLOWED_PATHS.contains(&req.path()) {
            // TODO: find a better way to handle this
            return Box::pin(async move {
                Ok(Session {
                    inner: session,
                    user: None,
                })
            });
        }

        session.renew();

        let user_service = match req.app_data::<web::Data<Arc<dyn UserService>>>() {
            Some(state) => state.clone(),
            None => {
                tracing::error!("No application state found in request");
                return Box::pin(async move { Err(ApiError::InternalServerError) });
            }
        };

        Box::pin(async move {
            let user_id = match session.get("user_id")? {
                Some(user_id) => user_id,
                None => {
                    return Err(AuthError::Unauthorized.into());
                }
            };

            let user = user_service.get_by_id(user_id).await?;

            if user.is_active == false {
                return Err(AuthError::AccountNotActivated.into());
            }

            Ok(Session {
                inner: session,
                user: Some(user),
            })
        })
    }
}
