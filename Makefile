.PHONY: help build run dev test test-cov clean fmt lint check docker-build docker-up docker-down db-create db-drop db-reset db-migrate

# Default target
help:
	@echo "Personal Website - Available Commands"
	@echo ""
	@echo "Development:"
	@echo "  make dev          - Run in development mode with hot reload"
	@echo "  make run          - Run the server"
	@echo "  make build        - Build release binary"
	@echo ""
	@echo "Testing:"
	@echo "  make test         - Run all tests"
	@echo "  make test-cov     - Run tests with coverage report"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt          - Format code with rustfmt"
	@echo "  make lint         - Run clippy linter"
	@echo "  make check        - Run cargo check"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start all services with docker-compose"
	@echo "  make docker-down  - Stop all services"
	@echo ""
	@echo "Database:"
	@echo "  make db-create    - Create database"
	@echo "  make db-drop      - Drop database"
	@echo "  make db-reset     - Drop and recreate database with migrations"
	@echo "  make db-migrate   - Run all migrations"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Clean build artifacts"

# Development
dev:
	cargo watch -x run

run:
	cargo run

build:
	cargo build --release

# Testing
test:
	cargo test

test-cov:
	cargo llvm-cov --html
	@echo "Coverage report generated at target/llvm-cov/html/index.html"

# Code Quality
fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo check

# Docker
docker-build:
	docker build -t personal-website .

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

# Database
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

# Maintenance
clean:
	cargo clean

