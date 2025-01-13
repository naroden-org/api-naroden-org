use std::fmt::{Debug, Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum NarodenError {
    AlreadyUsedPhone,
    AlreadyUsedEmail,
    InvalidCredentials,
    MissingAuthorizationHeader,
    GeneralError,
}

impl NarodenError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            NarodenError::AlreadyUsedPhone => StatusCode::BAD_REQUEST, // 400
            NarodenError::AlreadyUsedEmail => StatusCode::BAD_REQUEST, // 400
            NarodenError::InvalidCredentials => StatusCode::UNAUTHORIZED, // 401
            NarodenError::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED, // 401
            NarodenError::GeneralError => StatusCode::INTERNAL_SERVER_ERROR, // 500

        }
    }

    pub fn message(&self) -> &'static str {
        match *self {
            NarodenError::AlreadyUsedPhone => "Already used phone",
            NarodenError::AlreadyUsedEmail => "Already used email",
            NarodenError::InvalidCredentials => "Login failed",
            NarodenError::MissingAuthorizationHeader => "Missing authorization header",
            NarodenError::GeneralError => "General error",
        }
    }

    pub fn code(&self) -> &'static str {
        match *self {
            NarodenError::AlreadyUsedPhone => "4001",
            NarodenError::AlreadyUsedEmail => "4002",
            NarodenError::MissingAuthorizationHeader => "1002",
            NarodenError::InvalidCredentials => "1001",
            NarodenError::GeneralError => "1000",
        }
    }
}

impl Display for NarodenError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for NarodenError {

}

impl IntoResponse for NarodenError {
    fn into_response(self) -> Response {
        (self.status_code(), self.message()).into_response()
    }
}