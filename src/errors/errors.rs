use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::http;
use axum::body::Body;

pub enum AppError {
    InternalServerError,
    NotFound,
    BadRequest,
    Unauthorized,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

use serde_json::json;

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let body = match self {
            AppError::InternalServerError => "Internal Server Error",
            AppError::NotFound => "Not Found",
            AppError::BadRequest => "Bad Request",
            AppError::Unauthorized => "Unauthorized",
        };

        let json = json!({
            "message": body,
        });

        http::Response::builder()
            .status(self.status_code())
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&json).unwrap_or_default()))
            .unwrap_or_default()

    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(_: diesel::result::Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<deadpool_diesel::PoolError> for AppError {
    fn from(_: deadpool_diesel::PoolError) -> Self {
        AppError::InternalServerError
    }
}



impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::InternalServerError
    }
}

