# AnimaForge API

A complete Rust backend API for AnimaForge - A browser-based animation creation platform.

## Features

- RESTful API with Actix-web
- PostgreSQL database with SQLx
- JWT-based authentication
- CORS support
- Request logging
- Error handling
- Database connection pooling
- Pagination support
- Full-text search

## Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- PostgreSQL 14+
- SQLx CLI (for migrations)

## Installation

1. Install SQLx CLI:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

2. Copy environment variables:
```bash
cp .env.example .env
```

3. Update `.env` with your database credentials and JWT secret

4. Create database:
```bash
createdb animaforge
```

5. Run migrations:
```bash
sqlx migrate run --source src/db/migrations
```

## Running the API

Development mode:
```bash
cargo run
```

Release mode:
```bash
cargo build --release
./target/release/animaforge-api
```

The API will start on `http://localhost:8080` by default.

## API Endpoints

### Health Check
- `GET /api/v1/health` - Health check endpoint

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login user (returns JWT token)
- `POST /api/v1/auth/logout` - Logout user
- `GET /api/v1/auth/me` - Get current user (requires auth)

### Animations
- `GET /api/v1/animations` - List all animations (paginated)
- `POST /api/v1/animations` - Create new animation (requires auth)
- `GET /api/v1/animations/:id` - Get animation details
- `PUT /api/v1/animations/:id` - Update animation (requires auth)
- `DELETE /api/v1/animations/:id` - Delete animation (requires auth)
- `GET /api/v1/animations/featured` - Get featured animations
- `GET /api/v1/animations/trending` - Get trending animations

### Users
- `GET /api/v1/users/:id/profile` - Get user profile
- `GET /api/v1/users/:id/animations` - Get user's animations
- `PUT /api/v1/users/:id/profile` - Update profile (requires auth)

### Search
- `GET /api/v1/search?q=query` - Search animations

## Authentication

Protected endpoints require a JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

Obtain a token by logging in via `/api/v1/auth/login`.

## Pagination

List endpoints support pagination via query parameters:

```
?page=1&limit=20
```

Default: `page=1`, `limit=20`

## Example Requests

### Register User
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

### Create Animation
```bash
curl -X POST http://localhost:8080/api/v1/animations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{
    "title": "My Animation",
    "description": "A cool animation",
    "file_url": "https://example.com/animation.mp4",
    "thumbnail_url": "https://example.com/thumb.jpg"
  }'
```

### Search Animations
```bash
curl "http://localhost:8080/api/v1/search?q=cool&page=1&limit=10"
```

## Development

### Running migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run --source src/db/migrations
```

Revert last migration:
```bash
sqlx migrate revert --source src/db/migrations
```

### Testing

Run tests:
```bash
cargo test
```

### Code formatting

Format code:
```bash
cargo fmt
```

Check formatting:
```bash
cargo fmt -- --check
```

### Linting

Run clippy:
```bash
cargo clippy
```

## Project Structure

```
api/
├── Cargo.toml              # Rust dependencies
├── .env.example            # Environment variables template
├── src/
│   ├── main.rs            # Application entry point
│   ├── routes/            # API route handlers
│   │   ├── mod.rs
│   │   ├── auth.rs        # Authentication endpoints
│   │   ├── animations.rs  # Animation CRUD
│   │   ├── users.rs       # User profiles
│   │   └── search.rs      # Search functionality
│   ├── models/            # Data models
│   │   ├── mod.rs
│   │   ├── user.rs        # User model
│   │   └── animation.rs   # Animation model
│   ├── middleware/        # Middleware
│   │   ├── mod.rs
│   │   ├── auth.rs        # JWT validation
│   │   └── cors.rs        # CORS configuration
│   ├── db/                # Database
│   │   ├── mod.rs
│   │   └── migrations/    # SQL migrations
│   │       ├── 001_users.sql
│   │       └── 002_animations.sql
│   └── utils/             # Utilities
│       ├── mod.rs
│       └── errors.rs      # Error handling
```

## Environment Variables

See `.env.example` for all available configuration options.

Required variables:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT signing

Optional variables:
- `PORT` - Server port (default: 8080)
- `HOST` - Server host (default: 0.0.0.0)
- `RUST_LOG` - Logging level
- `CORS_ALLOWED_ORIGIN` - Allowed CORS origin (default: http://localhost:3000)
- `JWT_EXPIRATION` - JWT expiration in seconds (default: 86400)

## License

MIT
