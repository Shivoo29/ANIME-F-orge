# AnimaForge Development Guide 🛠️

> **Get AnimaForge running locally in 10 minutes**

## Prerequisites

### Required Software
```bash
# Check if you have everything installed:

# Rust (1.70+)
rustc --version

# Python (3.8+)
python --version

# Node.js (18+)
node --version

# PostgreSQL (14+)
psql --version

# Redis (6+)
redis-server --version

# FFmpeg
ffmpeg -version

# LaTeX
latex --version
```

### Installation Instructions

<details>
<summary><b>macOS (Homebrew)</b></summary>

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rust python node postgresql redis ffmpeg
brew install --cask mactex  # LaTeX (large download)

# Or use BasicTeX for smaller install:
brew install --cask basictex
sudo tlmgr update --self
sudo tlmgr install collection-latex
```
</details>

<details>
<summary><b>Ubuntu/Debian</b></summary>

```bash
# Update package list
sudo apt update

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Python
sudo apt install python3 python3-pip python3-venv

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install nodejs

# Install PostgreSQL
sudo apt install postgresql postgresql-contrib

# Install Redis
sudo apt install redis-server

# Install FFmpeg and LaTeX
sudo apt install ffmpeg texlive-full
```
</details>

<details>
<summary><b>Windows (WSL2 Recommended)</b></summary>

```powershell
# Install WSL2
wsl --install

# Then follow Ubuntu instructions inside WSL2

# Or use native Windows:
# - Rust: https://rustup.rs/
# - Python: https://www.python.org/downloads/
# - Node.js: https://nodejs.org/
# - PostgreSQL: https://www.postgresql.org/download/windows/
# - Redis: https://github.com/microsoftarchive/redis/releases
# - FFmpeg: https://ffmpeg.org/download.html
# - MiKTeX (LaTeX): https://miktex.org/download
```
</details>

---

## Quick Start (Docker - Recommended)

### 1. Clone Repository
```bash
git clone https://github.com/yourusername/animaforge.git
cd animaforge
```

### 2. Start All Services
```bash
# Build and start containers
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

### 3. Access Services
- **Web**: http://localhost:3000
- **API**: http://localhost:8080
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379

### 4. Stop Services
```bash
docker-compose down

# Stop and remove volumes (fresh start)
docker-compose down -v
```

---

## Manual Development Setup

### 1. Clone & Setup

```bash
# Clone repository
git clone https://github.com/yourusername/animaforge.git
cd animaforge

# Copy environment files
cp .env.example .env
cp cli/.env.example cli/.env
cp api/.env.example api/.env
cp web/.env.example web/.env.local

# Edit .env files with your settings
```

### 2. Database Setup

```bash
# Start PostgreSQL (if not running)
brew services start postgresql  # macOS
sudo systemctl start postgresql # Linux

# Create database
createdb animaforge_dev

# Or with psql:
psql -U postgres
CREATE DATABASE animaforge_dev;
\q

# Create .env file for database
echo "DATABASE_URL=postgresql://postgres:password@localhost/animaforge_dev" >> api/.env
```

### 3. Start Redis

```bash
# macOS
brew services start redis

# Linux
sudo systemctl start redis

# Or run in foreground
redis-server
```

### 4. Setup Python Engine

```bash
cd engine

# Create virtual environment
python -m venv venv

# Activate it
source venv/bin/activate  # Linux/macOS
# or
.\venv\Scripts\activate   # Windows

# Install dependencies
pip install -e ".[dev]"

# Install Manim
pip install manim

# Test installation
manim --version

# Run tests
pytest

cd ..
```

### 5. Setup Rust CLI

```bash
cd cli

# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .

# Test installation
animaforge --version

cd ..
```

### 6. Setup Rust API

```bash
cd api

# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run

# Build
cargo build

# Run tests
cargo test

# Start server (in background)
cargo run &

# Test API
curl http://localhost:8080/health

cd ..
```

### 7. Setup Next.js Frontend

```bash
cd web

# Install dependencies
npm install

# Generate Prisma client (if using Prisma)
npx prisma generate

# Start development server
npm run dev

# In another terminal, run tests
npm test

# Access at http://localhost:3000
```

### 8. Install Ollama (Optional - for local LLM)

```bash
# macOS/Linux
curl https://ollama.ai/install.sh | sh

# Pull a model
ollama pull llama3

# Start Ollama
ollama serve

# Test it
ollama run llama3 "Hello!"
```

---

## Development Workflow

### Using Make Commands

```bash
# View all available commands
make help

# Install all dependencies
make install

# Run all tests
make test

# Run linters
make lint

# Format code
make format

# Start all services
make dev

# Build production
make build

# Clean build artifacts
make clean
```

### Make Targets

<details>
<summary>View all Make targets</summary>

```makefile
install:          Install all dependencies
install-cli:      Install CLI dependencies
install-api:      Install API dependencies
install-engine:   Install Python engine
install-web:      Install web dependencies

dev:              Start all development servers
dev-cli:          Run CLI in dev mode
dev-api:          Run API dev server
dev-web:          Run Next.js dev server

test:             Run all tests
test-cli:         Run CLI tests
test-api:         Run API tests
test-engine:      Run engine tests
test-web:         Run web tests
test-integration: Run integration tests

lint:             Run all linters
lint-cli:         Lint Rust CLI
lint-api:         Lint Rust API
lint-engine:      Lint Python code
lint-web:         Lint TypeScript

format:           Format all code
format-cli:       Format Rust CLI
format-api:       Format Rust API
format-engine:    Format Python code
format-web:       Format TypeScript

build:            Build production artifacts
build-cli:        Build CLI binary
build-api:        Build API binary
build-web:        Build Next.js production

clean:            Clean build artifacts
clean-cli:        Clean CLI build
clean-api:        Clean API build
clean-engine:     Clean Python cache
clean-web:        Clean Next.js build

docker-up:        Start Docker containers
docker-down:      Stop Docker containers
docker-logs:      View Docker logs
docker-clean:     Remove all containers and volumes

db-migrate:       Run database migrations
db-seed:          Seed database with test data
db-reset:         Reset database

help:             Show this help message
```
</details>

---

## Daily Development Routine

### Morning Setup
```bash
# 1. Pull latest changes
git pull origin main

# 2. Check for dependency updates
make install

# 3. Start all services
make dev

# 4. Run tests to ensure everything works
make test
```

### During Development
```bash
# Auto-format on save (setup in your editor)
# or manually:
make format

# Run tests for what you're working on
cd cli && cargo test
cd engine && pytest
cd web && npm test

# Check linting
make lint
```

### Before Committing
```bash
# 1. Format code
make format

# 2. Run all tests
make test

# 3. Check linting
make lint

# 4. Commit with conventional commits
git add .
git commit -m "feat(cli): add batch processing support"

# 5. Push
git push origin feature/your-feature
```

---

## Editor Setup

### VS Code

**Recommended Extensions:**
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "ms-python.python",
    "ms-python.vscode-pylance",
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "bradlc.vscode-tailwindcss",
    "prisma.prisma"
  ]
}
```

**Settings:**
```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[python]": {
    "editor.defaultFormatter": "ms-python.black-formatter"
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  }
}
```

### Vim/Neovim

```vim
" Install plugins (using vim-plug)
Plug 'rust-lang/rust.vim'
Plug 'neoclide/coc.nvim'
Plug 'python-mode/python-mode'
Plug 'prettier/vim-prettier'

" Rust
let g:rustfmt_autosave = 1

" Python
let g:python_mode_lint = 1
let g:python_mode_lint_on_write = 1
```

---

## Testing

### Run Specific Tests

```bash
# CLI - specific test
cd cli
cargo test test_animation_creation

# Engine - specific test
cd engine
pytest tests/test_generator.py::test_code_generation -v

# Web - specific component
cd web
npm test -- AnimationCard

# Integration - specific workflow
./tests/integration/test_full_workflow.sh
```

### Watch Mode

```bash
# CLI
cd cli
cargo watch -x test

# Engine
cd engine
pytest-watch

# Web
cd web
npm test -- --watch
```

### Coverage

```bash
# CLI
cd cli
cargo tarpaulin --out Html

# Engine
cd engine
pytest --cov=animaforge_engine --cov-report=html

# Web
cd web
npm test -- --coverage
```

---

## Debugging

### CLI Debugging

```rust
// Add debug prints
dbg!(&animation);

// Use RUST_LOG for detailed logs
RUST_LOG=debug cargo run

// Or in code
env_logger::Builder::from_env(
    env_logger::Env::default().default_filter_or("debug")
).init();
```

### API Debugging

```bash
# Run with debug logs
RUST_LOG=actix_web=debug,animaforge_api=debug cargo run

# Use curl to test endpoints
curl -v http://localhost:8080/v1/animations

# Check database
psql animaforge_dev
SELECT * FROM animations;
```

### Frontend Debugging

```typescript
// Use React DevTools
// Chrome extension: React Developer Tools

// Log in components
console.log('Animation data:', animation);

// Use Next.js built-in debugging
DEBUG=* npm run dev
```

---

## Common Issues & Solutions

### Issue: Port Already in Use

```bash
# Find process using port
lsof -i :8080  # macOS/Linux
netstat -ano | findstr :8080  # Windows

# Kill process
kill -9 <PID>
```

### Issue: Database Connection Failed

```bash
# Check if PostgreSQL is running
pg_isready

# Restart PostgreSQL
brew services restart postgresql  # macOS
sudo systemctl restart postgresql # Linux

# Check connection
psql -U postgres -d animaforge_dev
```

### Issue: Manim Not Found

```bash
# Reinstall Manim
pip uninstall manim
pip install manim

# Check installation
manim --version
which manim
```

### Issue: Cargo Build Failed

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build

# Check for conflicts
cargo tree
```

### Issue: npm Install Failed

```bash
# Clear cache
npm cache clean --force

# Remove node_modules and lock file
rm -rf node_modules package-lock.json

# Reinstall
npm install
```

---

## Performance Tips

### CLI Performance

```bash
# Use release builds for better performance
cargo build --release
./target/release/animaforge create "test"

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph -- create "test"
```

### API Performance

```bash
# Enable connection pooling
# In api/.env
DATABASE_POOL_SIZE=10

# Monitor with metrics
cargo install metrics-exporter-prometheus
```

### Frontend Performance

```bash
# Analyze bundle size
npm run build
npm run analyze

# Use production build for testing
npm run build
npm run start
```

---

## Useful Commands

```bash
# Database
psql animaforge_dev                    # Connect to DB
\dt                                     # List tables
\d animations                           # Describe table
SELECT COUNT(*) FROM animations;        # Query data

# Redis
redis-cli                               # Connect to Redis
KEYS *                                  # List all keys
GET key_name                            # Get value
FLUSHALL                                # Clear all data

# Git
git status                              # Check status
git log --oneline --graph               # View history
git branch                              # List branches
git checkout -b feature/new-feature     # New branch

# System
htop                                    # Monitor system
df -h                                   # Check disk space
du -sh *                                # Folder sizes
```

---

## Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Manim Docs](https://docs.manim.community/)
- [Next.js Docs](https://nextjs.org/docs)
- [Actix Web](https://actix.rs/)

### Community
- [Discord Server](https://discord.gg/animaforge)
- [GitHub Discussions](https://github.com/yourusername/animaforge/discussions)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/animaforge)

### Tools
- [Rust Playground](https://play.rust-lang.org/)
- [TypeScript Playground](https://www.typescriptlang.org/play)
- [Manim Sandbox](https://try.manim.community/)

---

## Next Steps

1. ✅ Get the development environment running
2. 📖 Read the [Architecture docs](./ARCHITECTURE.md)
3. 🐛 Pick a [good first issue](https://github.com/yourusername/animaforge/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
4. 💻 Make your first contribution!
5. 🎉 Join our [Discord](https://discord.gg/animaforge)

---

**Happy coding! 🚀**

Need help? Don't hesitate to ask in Discord or open a discussion on GitHub.
