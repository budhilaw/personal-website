//! External integrations and infrastructure packages.
//!
//! This module contains wrappers for third-party services and external dependencies:
//! - Redis for caching and session storage
//! - Future: WhatsApp OTP, email services, payment gateways, etc.

pub mod redis;

pub use redis::*;
