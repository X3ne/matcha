#[repr(u32)]
#[derive(Debug, Clone)]
pub enum ErrorCode {
    Default = 0,
    UnknownUser = 10001,
    UnknownProfile = 10002,
    UnknownProvider = 10003,
    UnknownSession = 10004,
    Unauthorized = 40001,
    AccountNotActivated = 40002,
    UserEmailAlreadyExists = 40003,
    UserUsernameAlreadyExists = 40004,
    InvalidFormBody = 50001,
    InvalidCredentials = 50002,
}

impl ErrorCode {
    pub fn message(&self) -> &str {
        match self {
            ErrorCode::Default => "An error occurred",
            ErrorCode::UnknownUser => "Unknown user",
            ErrorCode::UnknownProfile => "Unknown profile",
            ErrorCode::UnknownProvider => "Unknown provider",
            ErrorCode::UnknownSession => "Unknown session",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::AccountNotActivated => "You need to verify your account to perform this action",
            ErrorCode::UserEmailAlreadyExists => "An user with this email already exists",
            ErrorCode::UserUsernameAlreadyExists => "An user with this username already exists",
            ErrorCode::InvalidFormBody => "Invalid form body",
            ErrorCode::InvalidCredentials => "Invalid credentials",
        }
    }
}
