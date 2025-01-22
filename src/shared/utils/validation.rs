use chrono::Datelike;
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

pub fn validate_birth_date(birth_date: &chrono::NaiveDate, _context: &()) -> garde::Result {
    let now = chrono::Utc::now().naive_utc().date();
    let age = now.year() - birth_date.year();

    if age < 18 {
        return Err(garde::Error::new("user must be at least 18 years old"));
    }

    Ok(())
}
