# AnimaForge API Quick Start Guide

Get your AnimaForge API up and running in minutes!

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Install PostgreSQL** (if not already installed):
   - macOS: `brew install postgresql@14`
   - Ubuntu/Debian: `sudo apt-get install postgresql postgresql-contrib`
   - Windows: Download from https://www.postgresql.org/download/

3. **Start PostgreSQL**:
   - macOS: `brew services start postgresql@14`
   - Ubuntu/Debian: `sudo systemctl start postgresql`

## Quick Setup (5 minutes)

### Step 1: Environment Setup

Copy the example environment file:
```bash
cd /home/user/ANIME-F-orge/api
cp .env.example .env
```

Edit `.env` and update the database connection string if needed:
```env
DATABASE_URL=postgresql://animaforge:animaforge@localhost:5432/animaforge
JWT_SECRET=change-this-to-a-secure-random-string
```

### Step 2: Database Setup

Run the setup script:
```bash
make setup
```

This will:
- Install SQLx CLI
- Create the database
- Run all migrations

**OR** manually:
```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Create database
createdb animaforge

# Run migrations
sqlx migrate run --source src/db/migrations
```

### Step 3: Run the API

```bash
make run
```

Or:
```bash
cargo run
```

The API will start at `http://localhost:8080`

### Step 4: Test the API

Check if it's running:
```bash
curl http://localhost:8080/api/v1/health
```

You should see:
```json
{
  "status": "healthy",
  "service": "AnimaForge API",
  "version": "0.1.0"
}
```

## Quick Test Workflow

### 1. Register a User

```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

Save the token from the response!

### 2. Login (Alternative)

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

### 3. Create an Animation

Replace `YOUR_TOKEN` with the token from step 1:

```bash
curl -X POST http://localhost:8080/api/v1/animations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "title": "My First Animation",
    "description": "This is a test animation",
    "file_url": "https://example.com/animation.mp4",
    "thumbnail_url": "https://example.com/thumbnail.jpg"
  }'
```

### 4. List Animations

```bash
curl http://localhost:8080/api/v1/animations
```

### 5. Search Animations

```bash
curl "http://localhost:8080/api/v1/search?q=test"
```

## Development Workflow

### Run in Development Mode (with auto-reload)

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
make dev
```

### Run Tests

```bash
make test
```

### Format Code

```bash
make fmt
```

### Lint Code

```bash
make lint
```

### Run All Checks

```bash
make check
```

## Common Commands

```bash
# Build the project
make build

# Build for production
make build-release

# Clean build artifacts
make clean

# Create a new migration
make migrate-create NAME=add_new_table

# Run migrations
make migrate

# Revert last migration
make migrate-revert

# See all available commands
make help
```

## Database Access

### Connect to Database

```bash
psql -d animaforge
```

### Common SQL Queries

```sql
-- List all users
SELECT * FROM users;

-- List all animations
SELECT * FROM animations;

-- Count users
SELECT COUNT(*) FROM users;

-- Get user's animations
SELECT a.* FROM animations a
JOIN users u ON a.user_id = u.id
WHERE u.username = 'testuser';
```

## Troubleshooting

### Database Connection Error

**Error:** `connection to server failed`

**Solution:**
1. Make sure PostgreSQL is running: `pg_isready`
2. Check your DATABASE_URL in `.env`
3. Ensure the database exists: `createdb animaforge`

### SQLx CLI Not Found

**Error:** `sqlx: command not found`

**Solution:**
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Port Already in Use

**Error:** `Address already in use`

**Solution:**
1. Change the PORT in `.env` to something else (e.g., 8081)
2. Or stop the process using port 8080:
   ```bash
   lsof -ti:8080 | xargs kill
   ```

### Compilation Errors

**Solution:**
1. Update Rust: `rustup update`
2. Clean and rebuild: `make clean && make build`

## Next Steps

1. Read the full [API Documentation](API_DOCUMENTATION.md)
2. Explore the [README](README.md) for detailed information
3. Check out the code in `src/` to understand the architecture
4. Modify the code to add your own features!

## Getting Help

- Check the [README](README.md)
- Review the [API Documentation](API_DOCUMENTATION.md)
- Look at the code examples in each route handler
- Check environment variables in `.env.example`

## Production Deployment

Before deploying to production:

1. Change `JWT_SECRET` to a strong random string
2. Use a production-grade PostgreSQL instance
3. Enable SSL for database connections
4. Set appropriate CORS origins
5. Build in release mode: `make build-release`
6. Set `RUST_LOG=info` (not debug)
7. Use environment variables for configuration (not .env file)

---

Happy coding!
