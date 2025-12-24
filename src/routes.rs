//! Application routing configuration.

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::controllers;
use crate::middleware::{admin_middleware, auth_middleware, optional_auth_middleware};
use crate::repositories::{RoleRepository, UserRepository};
use crate::services::{AuthService, CategoryService, PostService, TagService};

/// Application state containing all services.
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub auth_service: AuthService,
    pub post_service: PostService,
    pub category_service: CategoryService,
    pub tag_service: TagService,
    pub user_repo: UserRepository,
    pub role_repo: RoleRepository,
}

// Implement FromRef for extracting individual services from AppState
impl axum::extract::FromRef<AppState> for AuthService {
    fn from_ref(state: &AppState) -> Self {
        state.auth_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for PostService {
    fn from_ref(state: &AppState) -> Self {
        state.post_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for CategoryService {
    fn from_ref(state: &AppState) -> Self {
        state.category_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for TagService {
    fn from_ref(state: &AppState) -> Self {
        state.tag_service.clone()
    }
}

impl axum::extract::FromRef<AppState> for UserRepository {
    fn from_ref(state: &AppState) -> Self {
        state.user_repo.clone()
    }
}

impl axum::extract::FromRef<AppState> for RoleRepository {
    fn from_ref(state: &AppState) -> Self {
        state.role_repo.clone()
    }
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

/// Create the application router with all routes.
pub fn create_router(state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(controllers::health_check))
        .route("/auth/login", post(controllers::login))
        .route("/auth/refresh", post(controllers::refresh_token));

    // Public routes with optional auth (for viewing content)
    let public_view_routes = Router::new()
        .route("/posts", get(controllers::list_posts))
        .route("/posts/slug/{slug}", get(controllers::get_post_by_slug))
        .route("/categories", get(controllers::list_categories))
        .route("/categories/{id}", get(controllers::get_category))
        .route("/tags", get(controllers::list_tags))
        .route("/tags/{id}", get(controllers::get_tag))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            optional_auth_middleware,
        ));

    // Auth-required routes (logout)
    let auth_routes = Router::new()
        .route("/auth/logout", post(controllers::logout))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Admin-only content routes
    let admin_post_routes = Router::new()
        .route("/posts", post(controllers::create_post))
        .route("/posts/{id}", put(controllers::update_post))
        .route("/posts/{id}", delete(controllers::delete_post))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    let admin_category_routes = Router::new()
        .route("/categories", post(controllers::create_category))
        .route("/categories/{id}", put(controllers::update_category))
        .route("/categories/{id}", delete(controllers::delete_category))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    let admin_tag_routes = Router::new()
        .route("/tags", post(controllers::create_tag))
        .route("/tags/{id}", put(controllers::update_tag))
        .route("/tags/{id}", delete(controllers::delete_tag))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    // Admin-only RBAC management routes
    let admin_user_routes = Router::new()
        .route("/users", get(controllers::list_users))
        .route("/users", post(controllers::create_user))
        .route("/users/{id}", get(controllers::get_user))
        .route("/users/{id}", delete(controllers::delete_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    let admin_role_routes = Router::new()
        .route("/roles", get(controllers::list_roles))
        .route("/roles", post(controllers::create_role))
        .route("/roles/{id}", get(controllers::get_role))
        .route("/roles/{id}", put(controllers::update_role))
        .route("/roles/{id}", delete(controllers::delete_role))
        .route(
            "/roles/{id}/permissions",
            get(controllers::get_role_permissions),
        )
        .route(
            "/roles/{id}/permissions",
            post(controllers::assign_permission),
        )
        .route(
            "/roles/{role_id}/permissions/{permission_id}",
            delete(controllers::remove_permission),
        )
        .route("/permissions", get(controllers::list_permissions))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_middleware,
        ));

    // Combine all routes under /api prefix
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", public_view_routes)
        .nest("/api", auth_routes)
        .nest("/api", admin_post_routes)
        .nest("/api", admin_category_routes)
        .nest("/api", admin_tag_routes)
        .nest("/api", admin_user_routes)
        .nest("/api", admin_role_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
