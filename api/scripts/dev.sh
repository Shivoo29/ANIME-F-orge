#!/bin/bash

# Development script for AnimaForge API
# Runs the API with auto-reload on code changes

set -e

# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo "cargo-watch is not installed. Installing..."
    cargo install cargo-watch
fi

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

echo "Starting AnimaForge API in development mode..."
echo "Server will restart on code changes"

cargo watch -x run
