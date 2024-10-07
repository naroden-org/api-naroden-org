use poem_openapi::Object;
use serde::Serialize;

#[derive(Serialize, Object)]
#[oai(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: String,
    pub description: String,
}

pub fn create_error(error: Error) -> ErrorResponse {
    match error {
        Error::GeneralError => ErrorResponse {
            code: String::from("1000"),
            description: String::from("General error"),
        },
        Error::InvalidCredentials => ErrorResponse {
            code: String::from("1001"),
            description: String::from("Invalid credentials."),
        },
    }
}

pub enum Error {
    GeneralError,
    InvalidCredentials,
}
