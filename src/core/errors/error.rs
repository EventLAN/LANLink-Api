use std::fmt;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::core::errors::auth_errors::AuthError;

pub enum AppError {
    IoError(std::io::Error),    // Standard IO error
    DatabaseQueryError(String),
    DatabaseConnectionError(String),
    InvalidInputError(String),
    NotFoundError(String),
    UnauthorizedError(String),
    InternalServerError(String),
    BadRequestError(String),
    UnknownError(String),
}

impl AppError {
    fn format_message(&self) -> String {
        match self {
            AppError::IoError(e) => format!("I/O Error: {}", e),
            AppError::DatabaseQueryError(msg) => format!("Database Query Error: {}", msg),
            AppError::DatabaseConnectionError(msg) => format!("Database Connection Error: {}", msg),
            AppError::InvalidInputError(msg) => format!("Invalid Input Error: {}", msg),
            AppError::NotFoundError(msg) => format!("Not Found Error: {}", msg),
            AppError::UnauthorizedError(msg) => format!("Unauthorized Error: {}", msg),
            AppError::InternalServerError(msg) => format!("Internal Server Error: {}", msg),
            AppError::BadRequestError(msg) => format!("Bad Request Error: {}", msg),
            AppError::UnknownError(msg) => format!("Unknown Error: {}", msg),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_message())
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: {}", self.format_message())
    }
}

impl Error for AppError {}