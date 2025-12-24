# Personal Website

A full-stack personal website built as a monorepo.

**Backend:** Rust (Axum, PostgreSQL, Redis) with permission-based RBAC and blog CMS.  
**Frontend:** React (coming soon)

## Getting Started

### Prerequisites

- Rust (stable)
- Docker & Docker Compose
- PostgreSQL and Redis (or use Docker)

### Setup

1. Clone the repo and copy the environment file:
   ```bash
   cp .env.example .env
   ```

2. Start the database services:
   ```bash
   docker-compose up -d postgres redis
   ```

3. Run database migrations:
   ```bash
   make db-reset
   ```

4. Start the server:
   ```bash
   make run
   ```

The API will be available at `http://localhost:3000`.

## Available Commands

```bash
make dev          # Run with hot reload
make run          # Run the server
make test         # Run tests
make fmt          # Format code
make lint         # Run linter
make db-reset     # Reset database and run migrations
```

## Project Structure

```
.
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Environment configuration
│   ├── db.rs                # Database connection pool
│   ├── error.rs             # Error types and handling
│   ├── response.rs          # API response wrapper
│   ├── routes.rs            # Route definitions
│   ├── controllers/
│   │   ├── auth_controller.rs
│   │   ├── post_controller.rs
│   │   ├── category_controller.rs
│   │   ├── tag_controller.rs
│   │   └── health_controller.rs
│   ├── services/
│   │   ├── auth_service.rs      # JWT, password hashing
│   │   ├── post_service.rs
│   │   ├── category_service.rs
│   │   └── tag_service.rs
│   ├── repositories/
│   │   ├── user_repo.rs
│   │   ├── role_repo.rs
│   │   ├── post_repo.rs
│   │   ├── category_repo.rs
│   │   └── tag_repo.rs
│   ├── models/
│   │   ├── user.rs
│   │   ├── role.rs
│   │   ├── permission.rs
│   │   ├── post.rs
│   │   ├── category.rs
│   │   └── tag.rs
│   ├── middleware/
│   │   └── auth.rs              # JWT validation, permission checks
│   └── pkg/
│       └── redis.rs             # Redis connection
├── migrations/
├── frontend/                    # React app (coming soon)
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── Makefile
└── .env.example
```

## API Endpoints

### Public
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| POST | `/api/auth/login` | Login |
| POST | `/api/auth/refresh` | Refresh token |

### Public (Read)
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/posts` | List posts |
| GET | `/api/posts/slug/:slug` | Get post by slug |
| GET | `/api/categories` | List categories |
| GET | `/api/categories/:id` | Get category |
| GET | `/api/tags` | List tags |
| GET | `/api/tags/:id` | Get tag |

### Authenticated
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/logout` | Logout |

### Protected (Permission-based)
| Method | Endpoint | Permission |
|--------|----------|------------|
| POST | `/api/posts` | posts:create |
| PUT | `/api/posts/:id` | posts:update |
| DELETE | `/api/posts/:id` | posts:delete |
| POST | `/api/categories` | categories:create |
| PUT | `/api/categories/:id` | categories:update |
| DELETE | `/api/categories/:id` | categories:delete |
| POST | `/api/tags` | tags:create |
| PUT | `/api/tags/:id` | tags:update |
| DELETE | `/api/tags/:id` | tags:delete |

### Admin Only (RBAC Management)
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | List all users |
| GET | `/api/users/:id` | Get user |
| POST | `/api/users` | Create user |
| DELETE | `/api/users/:id` | Delete user |
| GET | `/api/roles` | List roles |
| GET | `/api/roles/:id` | Get role |
| POST | `/api/roles` | Create role |
| PUT | `/api/roles/:id` | Update role |
| DELETE | `/api/roles/:id` | Delete role |
| GET | `/api/roles/:id/permissions` | Get role permissions |
| POST | `/api/roles/:id/permissions` | Assign permission to role |
| DELETE | `/api/roles/:id/permissions/:permission_id` | Remove permission from role |
| GET | `/api/permissions` | List all permissions |

## Default Users

| Email | Password | Role |
|-------|----------|------|
| admin@example.com | admin123 | Admin |
| editor@example.com | admin123 | Editor |
| writer@example.com | admin123 | Writer |
| viewer@example.com | admin123 | Viewer |

## License

MIT
