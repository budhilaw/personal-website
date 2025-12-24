//! Personal Website Backend
//!
//! A Rust backend with PostgreSQL, Redis-backed JWT authentication, and Blog CMS features.

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use personal_website::{
    config::Config,
    create_router, db,
    pkg::redis,
    repositories::{
        CategoryRepository, PostRepository, RoleRepository, TagRepository, UserRepository,
    },
    routes::AppState,
    services::{AuthService, CategoryService, PostService, TagService},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "personal_website=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();
    tracing::info!("Starting server on {}", config.server_addr());

    // Create database pool
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    tracing::info!("Connected to PostgreSQL");

    // Create Redis connection
    let redis_conn = redis::create_connection(&config.redis_url)
        .await
        .expect("Failed to connect to Redis");
    tracing::info!("Connected to Redis");

    // Create repositories
    let user_repo = UserRepository::new(db_pool.clone());
    let role_repo = RoleRepository::new(db_pool.clone());
    let post_repo = PostRepository::new(db_pool.clone());
    let category_repo = CategoryRepository::new(db_pool.clone());
    let tag_repo = TagRepository::new(db_pool.clone());

    // Create services
    let auth_service = AuthService::new(
        config.clone(),
        user_repo.clone(),
        role_repo.clone(),
        redis_conn,
    );
    let post_service = PostService::new(
        post_repo,
        user_repo.clone(),
        category_repo.clone(),
        tag_repo.clone(),
    );
    let category_service = CategoryService::new(category_repo);
    let tag_service = TagService::new(tag_repo);

    // Create app state
    let app_state = AppState {
        db_pool,
        auth_service,
        post_service,
        category_service,
        tag_service,
        user_repo,
        role_repo,
    };

    // Create router
    let app = create_router(app_state);

    // Create listener
    let addr: SocketAddr = config.server_addr().parse()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on http://{}", addr);

    // Run server
    axum::serve(listener, app).await?;

    Ok(())
}
