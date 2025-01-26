use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use axum::body::Body;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NarodenErrorResponse {
    pub code: String,
    pub description: String,
}

#[derive(Debug)]
pub enum NarodenError {
    AlreadyUsedPhone,
    AlreadyUsedEmail,
    InvalidCredentials,
    MissingAuthorizationHeader,
    GeneralError,
    DatabaseError,
}

impl NarodenError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            NarodenError::AlreadyUsedPhone => StatusCode::BAD_REQUEST, // 400
            NarodenError::AlreadyUsedEmail => StatusCode::BAD_REQUEST, // 400
            NarodenError::InvalidCredentials => StatusCode::UNAUTHORIZED, // 401
            NarodenError::MissingAuthorizationHeader => StatusCode::UNAUTHORIZED, // 401
            NarodenError::GeneralError => StatusCode::INTERNAL_SERVER_ERROR, // 500
            NarodenError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR, // 500

        }
    }

    pub fn message(&self) -> &'static str {
        match *self {
            NarodenError::AlreadyUsedPhone => "Already used phone",
            NarodenError::AlreadyUsedEmail => "Already used email",
            NarodenError::InvalidCredentials => "Login failed",
            NarodenError::MissingAuthorizationHeader => "Missing authorization header",
            NarodenError::GeneralError => "General error",
            NarodenError::DatabaseError => "Database error",
        }
    }

    pub fn code(&self) -> &'static str {
        match *self {
            NarodenError::AlreadyUsedPhone => "4001",
            NarodenError::AlreadyUsedEmail => "4002",
            NarodenError::DatabaseError => "1003",
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
    fn into_response(self) -> Response<Body> {
        let error = NarodenErrorResponse {
            code: self.code().to_string(),
            description: self.message().to_string(),
        };

        Response::builder()
            .status(self.status_code())
            .body(Body::from(serde_json::to_string(&error).unwrap()))
            .unwrap()

    }
}

impl From<surrealdb::Error> for NarodenError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        // TODO: specify errors
        Self::DatabaseError
    }
}

pub fn handle_panic(_: Box<dyn Any + Send + 'static>) -> Response<String> {

    // TODO: safe error
    let error = NarodenErrorResponse {
        code: NarodenError::GeneralError.code().to_string(),
        description: NarodenError::GeneralError.message().to_string(),
    };

    Response::builder()
        .status(NarodenError::GeneralError.status_code())
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&error).unwrap())
        .unwrap()
}