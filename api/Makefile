.PHONY: help setup build run dev test clean migrate migrate-create fmt lint check

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the project (install dependencies, create database, run migrations)
	@echo "Installing SQLx CLI..."
	cargo install sqlx-cli --no-default-features --features postgres
	@echo "Setting up database..."
	./scripts/setup_db.sh
	@echo "Setup complete!"

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

run: ## Run the API server
	cargo run

dev: ## Run the API server in development mode with auto-reload
	./scripts/dev.sh

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

migrate: ## Run database migrations
	sqlx migrate run --source src/db/migrations

migrate-revert: ## Revert last database migration
	sqlx migrate revert --source src/db/migrations

migrate-create: ## Create a new migration (usage: make migrate-create NAME=migration_name)
	sqlx migrate add $(NAME) --source src/db/migrations

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

lint: ## Run clippy linter
	cargo clippy -- -D warnings

check: ## Run all checks (format, lint, test)
	make fmt-check
	make lint
	make test

.env: ## Copy .env.example to .env if it doesn't exist
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "Created .env file. Please update with your configuration."; \
	fi
