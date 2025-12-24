//! Personal Website Backend Library
//!
//! This library exposes all modules for the personal website backend.

pub mod config;
pub mod controllers;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod pkg;
pub mod repositories;
pub mod response;
pub mod routes;
pub mod services;

pub use config::Config;
pub use error::AppError;
pub use routes::{create_router, AppState};
