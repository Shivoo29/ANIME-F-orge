# AnimaForge CLI

A powerful command-line tool for creating AI-powered animations using Manim.

## Features

- 🎨 **Create animations from text prompts** using LLMs (Ollama, Gemini, Claude)
- 🎬 **Render animations** with quality presets
- 🔧 **Flexible configuration** with multiple LLM backends
- 📦 **Publish to marketplace** for sharing
- 🔍 **Search animations** created by the community
- ✨ **Beautiful CLI** with progress bars and colored output

## Installation

```bash
cd cli
cargo build --release
```

The binary will be available at `target/release/animaforge`.

For easier access, you can install it globally:

```bash
cargo install --path .
```

## Quick Start

### 1. Configure the CLI

First, set up your LLM backend:

```bash
# Using Ollama (default, no API key needed)
animaforge config set --backend ollama --model llama2

# Or using Gemini
animaforge config set --backend gemini --model gemini-pro --api-key YOUR_KEY

# Or using Claude
animaforge config set --backend claude --model claude-3-sonnet --api-key YOUR_KEY
```

View your configuration:

```bash
animaforge config list
```

### 2. Create an Animation

Generate animation code from a text description:

```bash
animaforge create "A blue circle that grows and rotates"
```

With auto-render:

```bash
animaforge create "Pythagorean theorem visualization" --render
```

Specify output location:

```bash
animaforge create "Mathematical spiral" --output ./my_animation.py
```

### 3. Render an Animation

Render an existing animation file:

```bash
animaforge render animation_12345.py
```

With quality settings:

```bash
animaforge render animation.py --quality high
animaforge render animation.py --quality medium --output video.mp4
```

Quality options:
- `low` - 480p @ 15fps
- `medium` - 720p @ 30fps (default)
- `high` - 1080p @ 60fps

### 4. Publish to Marketplace

Share your animation with the community:

```bash
animaforge publish animation.py
```

You'll be prompted for:
- Title
- Description
- Tags

### 5. Search the Marketplace

Find animations created by others:

```bash
animaforge search "circle"
animaforge search "mathematics" --limit 20
```

## Commands Reference

### `animaforge create <prompt>`

Create animation code from a text description.

**Options:**
- `-r, --render` - Automatically render after creation
- `-o, --output <FILE>` - Output file path (default: auto-generated)

**Examples:**
```bash
animaforge create "A rotating cube with changing colors"
animaforge create "Graph of sine wave" --render
animaforge create "DNA helix animation" --output dna.py
```

### `animaforge render <file>`

Render an animation file to video.

**Options:**
- `-q, --quality <LEVEL>` - Quality: low, medium, high (default: medium)
- `-o, --output <FILE>` - Output video path

**Examples:**
```bash
animaforge render animation.py
animaforge render animation.py --quality high
animaforge render animation.py --output final.mp4
```

### `animaforge config`

Manage CLI configuration.

**Subcommands:**

**Set configuration:**
```bash
animaforge config set --backend <ollama|gemini|claude>
animaforge config set --model <model-name>
animaforge config set --api-key <key>
animaforge config set --output-dir <directory>
```

**Get configuration value:**
```bash
animaforge config get backend
animaforge config get model
```

**List all configuration:**
```bash
animaforge config list
```

### `animaforge publish <file>`

Publish animation to the marketplace.

**Example:**
```bash
animaforge publish my_animation.py
```

### `animaforge search <query>`

Search the marketplace for animations.

**Options:**
- `-l, --limit <N>` - Maximum results (default: 10)

**Examples:**
```bash
animaforge search "mathematics"
animaforge search "physics" --limit 5
```

## Configuration

Configuration is stored in `~/.animaforge/config.toml`.

**Example config file:**

```toml
backend = "ollama"
model = "llama2"
output_dir = "./animations"
ollama_endpoint = "http://localhost:11434"

# Optional
api_key = "your_api_key_here"
marketplace_token = "your_token_here"
```

## LLM Backend Setup

### Ollama (Recommended for local use)

1. Install Ollama: https://ollama.ai
2. Pull a model:
   ```bash
   ollama pull llama2
   ```
3. Start Ollama:
   ```bash
   ollama serve
   ```
4. Configure AnimaForge:
   ```bash
   animaforge config set --backend ollama --model llama2
   ```

### Gemini

1. Get API key from Google AI Studio
2. Configure:
   ```bash
   animaforge config set --backend gemini --model gemini-pro --api-key YOUR_KEY
   ```

### Claude

1. Get API key from Anthropic
2. Configure:
   ```bash
   animaforge config set --backend claude --model claude-3-sonnet --api-key YOUR_KEY
   ```

## Requirements

- Rust 1.70+ (for building)
- Python 3.8+
- Manim Community Edition (`pip install manim`)
- Ollama (if using local LLM)

## Troubleshooting

### "Manim not found"

Install Manim:
```bash
pip install manim
```

### "Failed to connect to Ollama"

Make sure Ollama is running:
```bash
ollama serve
```

### Config file location

The config file is stored at `~/.animaforge/config.toml`. You can edit it directly or use the `config set` command.

## Development

Build for development:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Run with debug output:
```bash
RUST_LOG=debug cargo run -- create "test animation"
```

## License

MIT
