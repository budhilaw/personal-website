//! Standardized API response wrapper.
//!
//! All API responses follow the format:
//! ```json
//! {"success": true, "data": {...}, "error": null}
//! ```

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

/// Standardized API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Pagination metadata.
#[derive(Debug, Serialize, Clone)]
pub struct Meta {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl Meta {
    /// Create new pagination metadata.
    pub fn new(page: i64, per_page: i64, total: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response with data.
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: None,
        }
    }

    /// Create a successful response with data and pagination metadata.
    pub fn with_meta(data: T, meta: Meta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }
}

/// Response type alias for handlers.
pub type ApiResult<T> = Result<Json<ApiResponse<T>>, crate::error::AppError>;

/// Helper function to create a success response.
pub fn success<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success(data))
}

/// Helper function to create a paginated response.
pub fn paginated<T: Serialize>(
    data: T,
    page: i64,
    per_page: i64,
    total: i64,
) -> Json<ApiResponse<T>> {
    Json(ApiResponse::with_meta(
        data,
        Meta::new(page, per_page, total),
    ))
}

/// Simple message response for operations that don't return data.
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert!(response.error.is_none());
        assert!(response.meta.is_none());
    }

    #[test]
    fn test_with_meta() {
        let response = ApiResponse::with_meta(vec![1, 2, 3], Meta::new(1, 10, 100));
        assert!(response.success);
        assert!(response.meta.is_some());
        let meta = response.meta.unwrap();
        assert_eq!(meta.page, 1);
        assert_eq!(meta.per_page, 10);
        assert_eq!(meta.total, 100);
        assert_eq!(meta.total_pages, 10);
    }

    #[test]
    fn test_meta_calculation() {
        // Exact division
        let meta = Meta::new(1, 10, 100);
        assert_eq!(meta.total_pages, 10);

        // Remainder
        let meta = Meta::new(1, 10, 95);
        assert_eq!(meta.total_pages, 10);

        // Less than one page
        let meta = Meta::new(1, 10, 5);
        assert_eq!(meta.total_pages, 1);
    }

    #[test]
    fn test_message_response() {
        let msg = MessageResponse::new("Operation successful");
        assert_eq!(msg.message, "Operation successful");
    }

    #[test]
    fn test_health_response_default() {
        let health = HealthResponse::default();
        assert_eq!(health.status, "ok");
    }
}
