use rand::rngs::OsRng;
use rand::RngCore;
use zxcvbn::{zxcvbn, Score};

pub fn generate_random_secure_string(length: usize) -> String {
    let mut buffer = vec![0u8; length];
    OsRng.fill_bytes(&mut buffer);
    hex::encode(buffer)
}

#[derive(Default)]
pub struct ValidatePasswordContext {
    pub username: String,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
}

pub fn validate_password(password: &str, context: &ValidatePasswordContext) -> garde::Result {
    let entropy = zxcvbn(
        password,
        &[
            &context.username,
            &context.last_name,
            &context.first_name,
            &context.email,
        ],
    );

    if entropy.score() < Score::Two {
        return Err(garde::Error::new("password is too weak"));
    }

    Ok(())
}
