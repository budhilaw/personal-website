//! Database connection pool setup.

use sqlx::{postgres::PgPoolOptions, PgPool};

/// Create a new PostgreSQL connection pool.
///
/// # Arguments
/// * `database_url` - PostgreSQL connection URL
///
/// # Returns
/// A configured connection pool ready for use.
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool_invalid_url() {
        let result = create_pool("postgres://invalid:invalid@localhost:9999/nonexistent").await;
        assert!(result.is_err());
    }
}
