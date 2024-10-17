use axum::response::{ IntoResponse, Response };
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

pub enum UserError {
    MissingField,
    UserNotFound,
    UserCreationFailed,
    UserUpdateFailed,
    InvalidParameter,
    UserMismatch,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UserError::MissingField => (StatusCode::BAD_REQUEST, "Missing field"),
            UserError::UserNotFound => (StatusCode::NO_CONTENT, "User not found"),
            UserError::UserCreationFailed => (StatusCode::INTERNAL_SERVER_ERROR, "User creation failed"),
            UserError::UserUpdateFailed => (StatusCode::INTERNAL_SERVER_ERROR, "User update failed"),
            UserError::InvalidParameter => (StatusCode::BAD_REQUEST, "Invalid Parameter"),
            UserError::UserMismatch => (StatusCode::UNAUTHORIZED, "Unorthorized User"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}