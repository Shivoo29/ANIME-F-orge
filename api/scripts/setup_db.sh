#!/bin/bash

# Setup database for AnimaForge API
# This script creates the database and runs migrations

set -e

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

echo "Setting up AnimaForge database..."

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "sqlx-cli is not installed. Installing..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Extract database name from DATABASE_URL
DB_NAME=$(echo $DATABASE_URL | sed -n 's/.*\/\([^?]*\).*/\1/p')

echo "Database: $DB_NAME"

# Create database if it doesn't exist
echo "Creating database if it doesn't exist..."
createdb $DB_NAME 2>/dev/null || echo "Database already exists"

# Run migrations
echo "Running migrations..."
sqlx migrate run --source src/db/migrations

echo "Database setup complete!"
echo "You can now run the API with: cargo run"
