use crate::domain::entities::user::User;
use crate::domain::errors::auth_error::AuthError;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::error::ApiError;
use actix_session::{Session as ActixSession, SessionExt};
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use apistos::ApiCookie;
use async_trait::async_trait;
use schemars::_serde_json::Value;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Metadata, Schema, SchemaObject, SingleOrVec};
use schemars::JsonSchema;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(ApiCookie)]
#[openapi_cookie(name = "session", description = "Session cookie", required = true)]
pub struct Session {
    inner: ActixSession,
    user: Option<User>,
}

impl Session {
    pub fn inner(&self) -> &ActixSession {
        &self.inner
    }

    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
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

        if req.path() == "/v1/auth/oauth2/42/callback" {
            // TODO: find a better way to handle this
            return Box::pin(async move {
                Ok(Session {
                    inner: session,
                    user: None,
                })
            });
        }

        // TODO: query user from db

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

            Ok(Session {
                inner: session,
                user: Some(user),
            })
        })
    }
}

impl JsonSchema for Session {
    fn schema_name() -> String {
        "Session".to_string()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            format: Some("session".to_string()),
            string: Some(Default::default()),
            metadata: Some(Box::new(Metadata {
                description: Some("The session cookie".to_string()),
                examples: vec![Value::String("session=123456".to_string())],
                ..Default::default()
            })),
            ..Default::default()
        };
        schema.into()
    }
}
