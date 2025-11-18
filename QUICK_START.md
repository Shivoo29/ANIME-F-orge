# AnimaForge Quick Start Guide 🚀

Get AnimaForge running in 5 minutes!

## Prerequisites

Before you begin, ensure you have:

- ✅ **Rust 1.70+** - [Install from rustup.rs](https://rustup.rs/)
- ✅ **Python 3.8+** - [Download Python](https://www.python.org/downloads/)
- ✅ **Node.js 18+** - [Download Node.js](https://nodejs.org/)
- ✅ **PostgreSQL** - [Download PostgreSQL](https://www.postgresql.org/download/)
- ✅ **FFmpeg** - For video rendering (optional but recommended)

## Automated Setup (Recommended)

Run the automated setup script:

```bash
cd /home/user/ANIME-F-orge
./scripts/setup.sh
```

This script will:
1. ✨ Install all dependencies
2. 🗄️ Create and seed the database
3. 🔨 Build all components
4. ⚙️ Configure environment variables
5. 📁 Create necessary directories

**That's it!** Skip to [Running the Platform](#running-the-platform)

## Manual Setup

If you prefer manual control:

### 1. Setup Database

```bash
# Create database
createdb animaforge_dev

# Initialize schema
psql -d animaforge_dev -f scripts/init-db.sql

# Add sample data
psql -d animaforge_dev -f scripts/seed-data.sql
```

### 2. Setup Python Engine

```bash
cd engine
python3 -m venv venv
source venv/bin/activate
pip install -e .
deactivate
cd ..
```

### 3. Setup Web Frontend

```bash
cd web
npm install
cp .env.example .env
cd ..
```

### 4. Setup Rust CLI

```bash
cd cli
cargo build --release
cd ..
```

### 5. Setup Rust API

```bash
cd api
cp .env.example .env
# Edit .env and set DATABASE_URL
cargo build --release
cd ..
```

## Running the Platform

### Start the Backend API

```bash
cd api
cargo run
# API runs at http://localhost:8080
```

### Start the Frontend (in a new terminal)

```bash
cd web
npm run dev
# Web app runs at http://localhost:3000
```

### Test the CLI

```bash
cd cli
./target/release/animaforge --help
```

## Your First Animation

### Using the Web Interface

1. **Open your browser**: http://localhost:3000
2. **Sign up** for an account or use demo credentials:
   - Email: `demo@animaforge.dev`
   - Password: `password123`
3. **Click "Create Animation"** in the dashboard
4. **Enter a prompt**: "A blue circle that rotates and changes to red"
5. **Wait for generation** (30-60 seconds)
6. **Preview and download** your animation!

### Using the CLI

```bash
cd cli

# Configure Ollama backend (if you have it running)
./target/release/animaforge config set --backend ollama

# Create an animation
./target/release/animaforge create "A spinning cube with rainbow colors"

# The animation code is saved to ./animations/
```

## Demo Accounts

We've created demo accounts for testing:

| Email | Username | Password |
|-------|----------|----------|
| demo@animaforge.dev | demo_user | password123 |
| math@animaforge.dev | math_teacher | password123 |
| creative@animaforge.dev | creative_coder | password123 |

Each account has sample animations you can explore!

## Common Issues

### Port Already in Use

If port 8080 or 3000 is in use:

```bash
# Find and kill the process
lsof -i :8080
kill -9 <PID>
```

### Database Connection Failed

Check your DATABASE_URL in `api/.env`:

```env
DATABASE_URL=postgresql://localhost/animaforge_dev
```

### Python Module Not Found

Make sure the virtual environment is activated:

```bash
cd engine
source venv/bin/activate
pip install -e .
```

### Ollama Not Running

If using the CLI with Ollama:

```bash
# Start Ollama
ollama serve

# Pull a model
ollama pull llama2
```

## Next Steps

- 📖 Read the [User Journey Guide](./USER_JOURNEY.md)
- 🎨 Explore the [Design System](./web/COMPONENTS.md)
- 🔧 Check the [API Documentation](./api/API_QUICK_REFERENCE.md)
- 🐍 Learn about the [Python Engine](./engine/README.md)

## Need Help?

- 🐛 [Report Issues](https://github.com/yourusername/animaforge/issues)
- 💬 Join our [Discord](https://discord.gg/animaforge)
- 📧 Email: hello@animaforge.dev

---

**Now go create something amazing!** 🎬✨
