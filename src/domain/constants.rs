#![allow(dead_code)]

pub const TEMPLATE_DIR: &str = "templates/email";
pub const RESET_PASSWORD_TEMPLATE: &str = "reset_password";
pub const ACCOUNT_CONFIRMATION_TEMPLATE: &str = "account_confirmation";

pub const CDN_BASE_URL: &str = "/v1/cdn";
pub const PROFILE_IMAGES_PATH: &str = "/profile";

pub const MAX_PROFILE_IMAGES: usize = 5;

pub const RESET_PASSWORD_TOKEN_TTL: u64 = 900;

pub const DISLIKED_PROFILE_TTL: u64 = 604800; // 1 week
