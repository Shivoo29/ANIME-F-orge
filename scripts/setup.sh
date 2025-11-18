#!/bin/bash

# AnimaForge Complete Setup Script
# This script sets up the entire AnimaForge platform for local development

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
print_info() {
    echo -e "${BLUE}ℹ ${1}${NC}"
}

print_success() {
    echo -e "${GREEN}✓ ${1}${NC}"
}

print_error() {
    echo -e "${RED}✗ ${1}${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ ${1}${NC}"
}

print_header() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

print_header "AnimaForge Platform Setup"

# Check prerequisites
print_info "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi
print_success "Rust found: $(cargo --version)"

if ! command_exists python3; then
    print_error "Python 3 is not installed. Please install Python 3.8+"
    exit 1
fi
print_success "Python found: $(python3 --version)"

if ! command_exists node; then
    print_error "Node.js is not installed. Please install Node.js 18+"
    exit 1
fi
print_success "Node.js found: $(node --version)"

if ! command_exists psql; then
    print_warning "PostgreSQL client not found. Database setup will be skipped."
    SKIP_DB=true
else
    print_success "PostgreSQL found: $(psql --version | head -n1)"
    SKIP_DB=false
fi

# Setup Python engine
print_header "Setting up Python Animation Engine"

cd engine

if [ ! -d "venv" ]; then
    print_info "Creating Python virtual environment..."
    python3 -m venv venv
fi

print_info "Activating virtual environment and installing dependencies..."
source venv/bin/activate
pip install --upgrade pip > /dev/null 2>&1
pip install -e . > /dev/null 2>&1

print_success "Python engine installed"
deactivate

cd ..

# Setup Web frontend
print_header "Setting up Next.js Frontend"

cd web

if [ ! -d "node_modules" ]; then
    print_info "Installing npm dependencies..."
    npm install > /dev/null 2>&1
else
    print_info "npm dependencies already installed"
fi

if [ ! -f ".env" ]; then
    print_info "Creating .env file from template..."
    cp .env.example .env
    print_success ".env file created"
else
    print_info ".env file already exists"
fi

print_success "Frontend setup complete"

cd ..

# Setup Rust CLI
print_header "Setting up Rust CLI"

cd cli

if [ ! -f ".env" ]; then
    print_info "Creating .env file..."
    cp .env.example .env
fi

print_info "Building CLI (this may take a few minutes)..."
cargo build --release > /dev/null 2>&1

print_success "CLI built successfully at cli/target/release/animaforge"

cd ..

# Setup Rust API
print_header "Setting up Rust API"

cd api

if [ ! -f ".env" ]; then
    print_info "Creating .env file..."
    cp .env.example .env
fi

print_info "Building API (this may take a few minutes)..."
cargo build --release > /dev/null 2>&1

print_success "API built successfully"

cd ..

# Database setup
if [ "$SKIP_DB" = false ]; then
    print_header "Setting up Database"

    print_info "Database name: animaforge_dev"
    print_info "Attempting to create database..."

    # Try to create database
    createdb animaforge_dev 2>/dev/null || print_warning "Database may already exist"

    print_info "Running schema initialization..."
    psql -d animaforge_dev -f scripts/init-db.sql > /dev/null 2>&1
    print_success "Database schema created"

    print_info "Seeding sample data..."
    psql -d animaforge_dev -f scripts/seed-data.sql > /dev/null 2>&1
    print_success "Sample data inserted"

    print_success "Database setup complete"
else
    print_warning "Skipping database setup"
fi

# Create necessary directories
print_header "Creating directories"

mkdir -p animations
mkdir -p ~/.animaforge

print_success "Directories created"

# Final summary
print_header "Setup Complete!"

echo ""
echo -e "${GREEN}✨ AnimaForge is ready to use! ✨${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo ""
echo "1. Start the backend API:"
echo -e "   ${YELLOW}cd api && cargo run${NC}"
echo ""
echo "2. Start the frontend (in another terminal):"
echo -e "   ${YELLOW}cd web && npm run dev${NC}"
echo ""
echo "3. Try the CLI tool:"
echo -e "   ${YELLOW}cd cli && ./target/release/animaforge --help${NC}"
echo ""
echo "4. Access the web interface:"
echo -e "   ${YELLOW}http://localhost:3000${NC}"
echo ""
echo "5. API is available at:"
echo -e "   ${YELLOW}http://localhost:8080${NC}"
echo ""
echo -e "${BLUE}Demo accounts (password: password123):${NC}"
echo "  • demo@animaforge.dev"
echo "  • math@animaforge.dev"
echo "  • creative@animaforge.dev"
echo ""
echo -e "${GREEN}Happy animating! 🎬${NC}"
echo ""
