use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::HashMap;
use tera::{Context, Tera};

pub use mail_template_macro::*;
pub mod error;
pub use tera;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct MailSchema {
    pub template_name: String,
    pub parameters: Vec<String>,
}

pub struct Rendered {
    subject: String,
    html: String,
    txt: String,
}

impl Rendered {
    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn txt(&self) -> &str {
        &self.txt
    }
}

pub async fn render_email<T: Serialize>(
    data: &T,
    template_subject: &str,
    template_html: &str,
    template_txt: &str,
) -> Result<Rendered, error::Error> {
    let mut tera = Tera::default();

    let data_value = serde_json::to_value(data)
        .map_err(|_| error::Error::SerializationError("Failed to serialize data".to_string()))?;

    let context_map: HashMap<String, String> = match data_value.as_object() {
        Some(obj) => {
            if obj.len() == 1 {
                match obj.values().next() {
                    Some(nested_value) if nested_value.is_object() => nested_value
                        .as_object()
                        .ok_or_else(|| error::Error::SerializationError("Failed to extract nested object".to_string()))?
                        .iter()
                        .map(|(k, v)| (k.clone(), v.to_string().replace('"', "")))
                        .collect(),
                    _ => {
                        return Err(error::Error::SerializationError(
                            "Expected nested object but found something else".to_string(),
                        ));
                    }
                }
            } else {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.to_string().replace('"', "")))
                    .collect()
            }
        }
        None => {
            return Err(error::Error::SerializationError("Data is not an object".to_string()));
        }
    };

    let tera_context = Context::from_serialize(&context_map)?;

    let rendered_html = tera.render_str(template_html, &tera_context)?;
    let rendered_subject = tera.render_str(template_subject, &tera_context)?;
    let rendered_txt = tera.render_str(template_txt, &tera_context)?;

    Ok(Rendered {
        subject: rendered_subject,
        html: rendered_html,
        txt: rendered_txt,
    })
}
