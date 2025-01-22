#[repr(u32)]
#[derive(Debug, Clone)]
pub enum ErrorCode {
    Default = 0,
    UnknownUser = 10001,
    UnknownProfile = 10002,
    UnknownProvider = 10003,
    UnknownSession = 10004,
    UnknownTag = 10005,
    MaxImages = 30001,
    CannotDeleteAvatar = 30002,
    Unauthorized = 40001,
    AccountNotActivated = 40002,
    UserEmailAlreadyExists = 40003,
    UserUsernameAlreadyExists = 40004,
    UserAlreadyHaveProfile = 40005,
    TagAlreadyExists = 40006,
    ProfileAlreadyHasTag = 40007,
    ProfileAlreadyLiked = 40008,
    InvalidFormBody = 50001,
    InvalidCredentials = 50002,
    InvalidMimeType = 50003,
    OnlyImagesAllowed = 50004,
    InvalidImageOffset = 50005,
    InvalidResetToken = 50006,
}

impl ErrorCode {
    pub fn message(&self) -> &str {
        match self {
            ErrorCode::Default => "An error occurred",
            ErrorCode::UnknownUser => "Unknown user",
            ErrorCode::UnknownProfile => "Unknown profile",
            ErrorCode::UnknownProvider => "Unknown provider",
            ErrorCode::UnknownSession => "Unknown session",
            ErrorCode::UnknownTag => "Unknown tag",
            ErrorCode::MaxImages => "Maximum images reached",
            ErrorCode::CannotDeleteAvatar => "Cannot delete avatar",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::AccountNotActivated => "You need to verify your account to perform this action",
            ErrorCode::UserEmailAlreadyExists => "An user with this email already exists",
            ErrorCode::UserUsernameAlreadyExists => "An user with this username already exists",
            ErrorCode::UserAlreadyHaveProfile => "This user already have a profile",
            ErrorCode::ProfileAlreadyHasTag => "Profile already has this tag",
            ErrorCode::ProfileAlreadyLiked => "Profile already liked",
            ErrorCode::TagAlreadyExists => "Tag already exists",
            ErrorCode::InvalidFormBody => "Invalid form body",
            ErrorCode::InvalidCredentials => "Invalid credentials",
            ErrorCode::InvalidMimeType => "Invalid mime type",
            ErrorCode::OnlyImagesAllowed => "Only images are allowed",
            ErrorCode::InvalidImageOffset => "Invalid image offset",
            ErrorCode::InvalidResetToken => "Invalid reset token",
        }
    }
}
