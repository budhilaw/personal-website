//! Repository modules for data access.

pub mod category_repo;
pub mod post_repo;
pub mod role_repo;
pub mod tag_repo;
pub mod user_repo;

pub use category_repo::CategoryRepository;
pub use post_repo::PostRepository;
pub use role_repo::RoleRepository;
pub use tag_repo::TagRepository;
pub use user_repo::UserRepository;
