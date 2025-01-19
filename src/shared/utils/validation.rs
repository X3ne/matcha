use zxcvbn::{zxcvbn, Score};

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

pub fn validate_opt_password(password: &Option<String>, context: &ValidatePasswordContext) -> garde::Result {
    if let Some(password) = password {
        validate_password(password, context)?;
    }

    Ok(())
}
