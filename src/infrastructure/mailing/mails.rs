use mail_template::GenerateMailSchemas;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, GenerateMailSchemas)]
pub enum Mail {
    AccountConfirmation { username: String, confirmation_url: String },
    ResetPassword { reset_url: String },
}
