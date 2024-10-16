use poem_openapi::Object;
use serde::Serialize;

#[derive(Serialize, Object)]
#[oai(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: String,
    pub description: String,
}

pub fn create_error(error: ApiError) -> ErrorResponse {
    match error {
        ApiError::GeneralError => ErrorResponse {
            code: String::from("1000"),
            description: String::from("General error"),
        },
        ApiError::InvalidCredentials => ErrorResponse {
            code: String::from("1001"),
            description: String::from("Invalid credentials"),
        },
        ApiError::AlreadyUsedPhone => ErrorResponse {
            code: String::from("4001"),
            description: String::from("Already used phone"),
        },
        ApiError::AlreadyUsedEmail => ErrorResponse {
            code: String::from("4002"),
            description: String::from("Already used email"),
        },
    }
}

pub enum ApiError {
    GeneralError,
    InvalidCredentials,
    AlreadyUsedPhone,
    AlreadyUsedEmail,
}
