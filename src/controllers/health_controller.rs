//! Health check controller.

use axum::Json;

use crate::response::{ApiResponse, HealthResponse};

/// Health check endpoint.
pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse::success(HealthResponse::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert!(response.0.success);
        assert!(response.0.data.is_some());
        assert_eq!(response.0.data.unwrap().status, "ok");
    }
}
