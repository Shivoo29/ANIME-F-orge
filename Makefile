# AnimaForge Makefile
# Simplifies common development tasks

.PHONY: help install test lint format build clean dev docker-up docker-down

# Default target
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Helper function to print colored output
define print_info
	@echo "$(BLUE)ℹ $(1)$(NC)"
endef

define print_success
	@echo "$(GREEN)✓ $(1)$(NC)"
endef

define print_error
	@echo "$(RED)✗ $(1)$(NC)"
endef

##@ General

help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\n$(BLUE)Usage:$(NC)\n  make $(GREEN)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(YELLOW)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Installation

install: install-cli install-api install-engine install-web ## Install all dependencies
	$(call print_success,All dependencies installed!)

install-cli: ## Install CLI dependencies
	$(call print_info,Installing CLI dependencies...)
	cd cli && cargo build
	$(call print_success,CLI dependencies installed)

install-api: ## Install API dependencies
	$(call print_info,Installing API dependencies...)
	cd api && cargo build
	$(call print_success,API dependencies installed)

install-engine: ## Install Python engine
	$(call print_info,Installing Python engine...)
	cd engine && \
		python -m venv venv && \
		. venv/bin/activate && \
		pip install -e ".[dev]"
	$(call print_success,Engine installed)

install-web: ## Install web dependencies
	$(call print_info,Installing web dependencies...)
	cd web && npm install
	$(call print_success,Web dependencies installed)

##@ Development

dev: ## Start all development servers (requires tmux)
	$(call print_info,Starting all development servers...)
	@if command -v tmux >/dev/null 2>&1; then \
		tmux new-session -d -s animaforge \; \
			send-keys 'make dev-api' C-m \; \
			split-window -h \; \
			send-keys 'make dev-web' C-m \; \
			split-window -v \; \
			send-keys 'redis-server' C-m \; \
			select-pane -t 0 \; \
			split-window -v \; \
			send-keys 'psql animaforge_dev' C-m \; \
			attach-session -t animaforge; \
	else \
		echo "$(YELLOW)tmux not found. Starting services sequentially...$(NC)"; \
		make dev-api & make dev-web; \
	fi

dev-cli: ## Run CLI in development mode
	$(call print_info,Running CLI...)
	cd cli && cargo run

dev-api: ## Run API development server
	$(call print_info,Starting API server on :8080...)
	cd api && RUST_LOG=debug cargo run

dev-web: ## Run Next.js development server
	$(call print_info,Starting web server on :3000...)
	cd web && npm run dev

dev-engine: ## Test Python engine interactively
	$(call print_info,Starting Python REPL with engine loaded...)
	cd engine && . venv/bin/activate && python -i -c "from animaforge_engine import *"

##@ Testing

test: test-cli test-api test-engine test-web ## Run all tests
	$(call print_success,All tests passed!)

test-cli: ## Run CLI tests
	$(call print_info,Running CLI tests...)
	cd cli && cargo test

test-api: ## Run API tests
	$(call print_info,Running API tests...)
	cd api && cargo test

test-engine: ## Run Python engine tests
	$(call print_info,Running engine tests...)
	cd engine && . venv/bin/activate && pytest

test-web: ## Run web tests
	$(call print_info,Running web tests...)
	cd web && npm test -- --run

test-integration: ## Run integration tests
	$(call print_info,Running integration tests...)
	./tests/integration/run_all.sh

test-coverage: ## Run tests with coverage
	$(call print_info,Generating coverage reports...)
	cd cli && cargo tarpaulin --out Html
	cd engine && . venv/bin/activate && pytest --cov=animaforge_engine --cov-report=html
	cd web && npm test -- --coverage
	$(call print_success,Coverage reports generated!)

##@ Code Quality

lint: lint-cli lint-api lint-engine lint-web ## Run all linters
	$(call print_success,All linting passed!)

lint-cli: ## Lint Rust CLI
	$(call print_info,Linting CLI...)
	cd cli && cargo clippy -- -D warnings

lint-api: ## Lint Rust API
	$(call print_info,Linting API...)
	cd api && cargo clippy -- -D warnings

lint-engine: ## Lint Python code
	$(call print_info,Linting Python engine...)
	cd engine && . venv/bin/activate && \
		ruff check . && \
		mypy animaforge_engine

lint-web: ## Lint TypeScript
	$(call print_info,Linting web...)
	cd web && npm run lint

format: format-cli format-api format-engine format-web ## Format all code
	$(call print_success,All code formatted!)

format-cli: ## Format Rust CLI
	$(call print_info,Formatting CLI...)
	cd cli && cargo fmt

format-api: ## Format Rust API
	$(call print_info,Formatting API...)
	cd api && cargo fmt

format-engine: ## Format Python code
	$(call print_info,Formatting Python engine...)
	cd engine && . venv/bin/activate && \
		black . && \
		isort .

format-web: ## Format TypeScript
	$(call print_info,Formatting web...)
	cd web && npm run format

check: lint test ## Run linters and tests
	$(call print_success,All checks passed!)

##@ Building

build: build-cli build-api build-web ## Build production artifacts
	$(call print_success,All builds complete!)

build-cli: ## Build CLI binary
	$(call print_info,Building CLI release binary...)
	cd cli && cargo build --release
	$(call print_success,CLI binary: ./cli/target/release/animaforge)

build-api: ## Build API binary
	$(call print_info,Building API release binary...)
	cd api && cargo build --release
	$(call print_success,API binary: ./api/target/release/animaforge-api)

build-engine: ## Build Python wheel
	$(call print_info,Building Python package...)
	cd engine && . venv/bin/activate && python -m build
	$(call print_success,Wheel: ./engine/dist/)

build-web: ## Build Next.js production bundle
	$(call print_info,Building web production bundle...)
	cd web && npm run build
	$(call print_success,Build output: ./web/.next/)

build-docker: ## Build Docker images
	$(call print_info,Building Docker images...)
	docker-compose build
	$(call print_success,Docker images built!)

##@ Cleaning

clean: clean-cli clean-api clean-engine clean-web ## Clean all build artifacts
	$(call print_success,All cleaned!)

clean-cli: ## Clean CLI build artifacts
	$(call print_info,Cleaning CLI...)
	cd cli && cargo clean

clean-api: ## Clean API build artifacts
	$(call print_info,Cleaning API...)
	cd api && cargo clean

clean-engine: ## Clean Python cache and builds
	$(call print_info,Cleaning engine...)
	cd engine && \
		rm -rf build dist *.egg-info && \
		find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true && \
		find . -type f -name "*.pyc" -delete

clean-web: ## Clean Next.js build
	$(call print_info,Cleaning web...)
	cd web && \
		rm -rf .next node_modules/.cache && \
		npm run clean 2>/dev/null || true

clean-all: clean ## Deep clean including dependencies
	$(call print_info,Deep cleaning...)
	cd cli && rm -rf target
	cd api && rm -rf target
	cd engine && rm -rf venv
	cd web && rm -rf node_modules
	$(call print_success,Deep clean complete!)

##@ Database

db-setup: ## Setup database
	$(call print_info,Setting up database...)
	createdb animaforge_dev || true
	cd api && sqlx migrate run
	$(call print_success,Database ready!)

db-migrate: ## Run database migrations
	$(call print_info,Running migrations...)
	cd api && sqlx migrate run

db-migrate-add: ## Create new migration (usage: make db-migrate-add name=your_migration_name)
	$(call print_info,Creating new migration...)
	cd api && sqlx migrate add $(name)

db-rollback: ## Rollback last migration
	$(call print_info,Rolling back last migration...)
	cd api && sqlx migrate revert

db-reset: ## Reset database
	$(call print_info,Resetting database...)
	dropdb animaforge_dev || true
	createdb animaforge_dev
	cd api && sqlx migrate run
	$(call print_success,Database reset complete!)

db-seed: ## Seed database with test data
	$(call print_info,Seeding database...)
	cd api && cargo run --bin seed
	$(call print_success,Database seeded!)

db-console: ## Open database console
	psql animaforge_dev

##@ Docker

docker-up: ## Start Docker containers
	$(call print_info,Starting Docker containers...)
	docker-compose up -d
	$(call print_success,Containers started!)
	@echo "$(YELLOW)Access services:$(NC)"
	@echo "  Web:        http://localhost:3000"
	@echo "  API:        http://localhost:8080"
	@echo "  PostgreSQL: localhost:5432"
	@echo "  Redis:      localhost:6379"

docker-down: ## Stop Docker containers
	$(call print_info,Stopping Docker containers...)
	docker-compose down
	$(call print_success,Containers stopped!)

docker-logs: ## View Docker logs
	docker-compose logs -f

docker-ps: ## Show running containers
	docker-compose ps

docker-clean: ## Remove containers and volumes
	$(call print_info,Cleaning Docker resources...)
	docker-compose down -v
	docker system prune -f
	$(call print_success,Docker cleaned!)

docker-rebuild: ## Rebuild and restart containers
	$(call print_info,Rebuilding containers...)
	docker-compose down
	docker-compose build --no-cache
	docker-compose up -d
	$(call print_success,Containers rebuilt!)

##@ Utilities

watch-cli: ## Watch and auto-recompile CLI
	cd cli && cargo watch -x build

watch-api: ## Watch and auto-restart API
	cd api && cargo watch -x run

watch-engine: ## Watch and auto-test engine
	cd engine && . venv/bin/activate && pytest-watch

watch-web: ## Watch and auto-rebuild web (same as dev-web)
	cd web && npm run dev

deps-update: ## Update all dependencies
	$(call print_info,Updating dependencies...)
	cd cli && cargo update
	cd api && cargo update
	cd engine && . venv/bin/activate && pip install --upgrade -r requirements.txt
	cd web && npm update
	$(call print_success,Dependencies updated!)

deps-check: ## Check for outdated dependencies
	$(call print_info,Checking for outdated dependencies...)
	cd cli && cargo outdated
	cd api && cargo outdated
	cd engine && . venv/bin/activate && pip list --outdated
	cd web && npm outdated

security-audit: ## Run security audits
	$(call print_info,Running security audits...)
	cd cli && cargo audit
	cd api && cargo audit
	cd web && npm audit
	$(call print_success,Security audit complete!)

docs: ## Generate documentation
	$(call print_info,Generating documentation...)
	cd cli && cargo doc --no-deps --open
	cd api && cargo doc --no-deps --open
	cd engine && . venv/bin/activate && cd docs && make html
	$(call print_success,Documentation generated!)

benchmark: ## Run benchmarks
	$(call print_info,Running benchmarks...)
	cd cli && cargo bench
	cd api && cargo bench
	$(call print_success,Benchmarks complete!)

##@ Release

release-check: ## Check if ready for release
	$(call print_info,Running pre-release checks...)
	@make lint
	@make test
	@make build
	$(call print_success,Ready for release!)

release-cli: ## Build and package CLI release
	$(call print_info,Building CLI release...)
	cd cli && cargo build --release
	cd cli/target/release && tar -czf animaforge-cli-$(shell uname -s)-$(shell uname -m).tar.gz animaforge
	$(call print_success,CLI release packaged!)

release-api: ## Build and package API release
	$(call print_info,Building API release...)
	cd api && cargo build --release
	cd api/target/release && tar -czf animaforge-api-$(shell uname -s)-$(shell uname -m).tar.gz animaforge-api
	$(call print_success,API release packaged!)

##@ Deployment

deploy-staging: ## Deploy to staging
	$(call print_info,Deploying to staging...)
	./scripts/deploy-staging.sh
	$(call print_success,Deployed to staging!)

deploy-prod: ## Deploy to production
	$(call print_info,Deploying to production...)
	./scripts/deploy-prod.sh
	$(call print_success,Deployed to production!)

##@ Miscellaneous

init: ## Initialize project (first time setup)
	$(call print_info,Initializing AnimaForge project...)
	@echo "$(YELLOW)This will set up your development environment$(NC)"
	@make install
	@make db-setup
	@echo "$(YELLOW)Starting Redis...$(NC)"
	@redis-server --daemonize yes 2>/dev/null || echo "Redis already running or not installed"
	@make dev
	$(call print_success,Project initialized! Check the running services above.)

fresh: clean-all install db-reset ## Fresh start (clean + install + reset db)
	$(call print_success,Fresh start complete!)

ci: lint test ## Run CI checks
	$(call print_success,CI checks passed!)

.PHONY: $(MAKECMDGOALS)
