.PHONY: help build run dev test test-cov clean fmt lint check docker-build docker-up docker-down db-create db-drop db-reset db-migrate
.PHONY: fe-dev fe-build fe-lint fe-preview dev-all build-all

# Default target
help:
	@echo "Personal Website - Available Commands"
	@echo ""
	@echo "Backend (Rust):"
	@echo "  make dev          - Run backend with hot reload"
	@echo "  make run          - Run backend server"
	@echo "  make build        - Build backend release binary"
	@echo "  make test         - Run backend tests"
	@echo "  make fmt          - Format backend code"
	@echo "  make lint         - Run clippy linter"
	@echo ""
	@echo "Frontend (React):"
	@echo "  make fe-dev       - Run frontend dev server"
	@echo "  make fe-build     - Build frontend for production"
	@echo "  make fe-lint      - Lint frontend code"
	@echo "  make fe-preview   - Preview frontend production build"
	@echo ""
	@echo "Full Stack:"
	@echo "  make dev-all      - Run both backend and frontend (requires tmux)"
	@echo "  make build-all    - Build both backend and frontend"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start all services"
	@echo "  make docker-down  - Stop all services"
	@echo ""
	@echo "Database:"
	@echo "  make db-create    - Create database"
	@echo "  make db-drop      - Drop database"
	@echo "  make db-reset     - Drop, recreate and run migrations"
	@echo "  make db-migrate   - Run all migrations"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Clean all build artifacts"

# Backend (Rust)
# ============================================

dev:
	@command -v cargo-watch >/dev/null 2>&1 && cargo watch -x run || cargo run

run:
	cargo run

build:
	cargo build --release

test:
	cargo test

test-cov:
	cargo llvm-cov --html
	@echo "Coverage report generated at target/llvm-cov/html/index.html"

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo check

# ============================================
# Frontend (React)
# ============================================

fe-dev:
	cd frontend && npm run dev

fe-build:
	cd frontend && npm run build

fe-lint:
	cd frontend && npm run lint

fe-preview:
	cd frontend && npm run preview

fe-install:
	cd frontend && npm install

# ============================================
# Full Stack
# ============================================

dev-all:
	@echo "Starting backend and frontend in tmux..."
	@echo "Backend: http://localhost:3000"
	@echo "Frontend: http://localhost:5173"
	@echo ""
	@echo "Tmux shortcuts: Ctrl+b then ← or → to switch panes, Ctrl+b then d to detach"
	@tmux kill-session -t dev 2>/dev/null || true
	@tmux new-session -d -s dev -n main
	@tmux send-keys -t dev 'make dev' Enter
	@tmux split-window -h -t dev
	@tmux send-keys -t dev 'make fe-dev' Enter
	@tmux attach -t dev

build-all: build fe-build
	@echo "Build complete!"
	@echo "Backend binary: target/release/personal-website"
	@echo "Frontend dist: frontend/dist/"

# ============================================
# Docker
# ============================================

docker-build:
	docker build -t personal-website .

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

# ============================================
# Database
# ============================================

DB_CONTAINER ?= postgres
DB_USER ?= postgres
DB_NAME ?= personal_website

db-create:
	docker exec -i $(DB_CONTAINER) psql -U $(DB_USER) -c "CREATE DATABASE $(DB_NAME);" || true

db-drop:
	docker exec -i $(DB_CONTAINER) psql -U $(DB_USER) -c "DROP DATABASE IF EXISTS $(DB_NAME);"

db-migrate:
	@echo "Running migrations..."
	@for f in migrations/*.sql; do \
		echo "-> $$f"; \
		docker exec -i $(DB_CONTAINER) psql -U $(DB_USER) -d $(DB_NAME) < "$$f"; \
	done
	@echo "Migrations complete!"

db-reset: db-drop db-create db-migrate
	@echo "Database reset complete!"

# ============================================
# Maintenance
# ============================================

clean:
	cargo clean
	rm -rf frontend/node_modules frontend/dist
	@echo "Cleaned all build artifacts"

install: fe-install
	@echo "Frontend dependencies installed"
