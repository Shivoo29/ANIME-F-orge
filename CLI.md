# AnimaForge CLI Documentation 🎬⚡

> **The power of animation creation in your terminal**

## Table of Contents
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Configuration](#configuration)
- [AI Backends](#ai-backends)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

---

## Installation

### Using Cargo (Recommended)
```bash
cargo install animaforge-cli
```

### Using Homebrew (macOS/Linux)
```bash
brew tap animaforge/tap
brew install animaforge
```

### From Binary
Download the latest release from [GitHub Releases](https://github.com/yourusername/animaforge/releases)

```bash
# Linux/macOS
curl -L https://github.com/animaforge/cli/releases/latest/download/animaforge-$(uname -s)-$(uname -m).tar.gz | tar xz
sudo mv animaforge /usr/local/bin/

# Verify installation
animaforge --version
```

### From Source
```bash
git clone https://github.com/yourusername/animaforge.git
cd animaforge/cli
cargo build --release
sudo cp target/release/animaforge /usr/local/bin/
```

### Dependencies
```bash
# Install required system dependencies

# macOS
brew install python3 ffmpeg texlive

# Ubuntu/Debian
sudo apt update
sudo apt install python3 python3-pip ffmpeg texlive texlive-latex-extra

# Arch Linux
sudo pacman -S python ffmpeg texlive-most

# Then install Manim
pip install manim
```

---

## Quick Start

### 1. Initialize Configuration
```bash
animaforge init

# Interactive setup wizard will guide you through:
# - AI backend selection
# - API key configuration (if using cloud LLMs)
# - Output directory setup
# - Quality preferences
```

### 2. Generate Your First Animation
```bash
# Simple text prompt
animaforge create "A blue circle morphing into a square"

# With options
animaforge create "Pythagorean theorem visualization" \
  --quality high \
  --duration 10 \
  --output ./my_animations/

# Preview before full render
animaforge create "Dancing sine wave" --preview-only
```

### 3. View Generated Animation
```bash
# Open in default video player
animaforge open animations/latest

# Or manually
open ./animations/dancing_sine_wave_001.mp4
```

---

## Commands

### `animaforge init`
Initialize AnimaForge configuration

```bash
animaforge init [OPTIONS]

Options:
  -f, --force           Overwrite existing configuration
  -b, --backend <NAME>  Set AI backend [ollama|gemini|claude|openai]
  -i, --interactive     Use interactive setup wizard
  -h, --help           Print help
```

**Example:**
```bash
# Interactive setup
animaforge init --interactive

# Quick setup with Ollama
animaforge init --backend ollama --model llama3
```

---

### `animaforge create`
Generate animation from text prompt

```bash
animaforge create <PROMPT> [OPTIONS]

Arguments:
  <PROMPT>  Natural language description of animation

Options:
  -q, --quality <LEVEL>      Quality level [low|medium|high|4k] [default: high]
  -d, --duration <SECONDS>   Animation duration [default: 5]
  -f, --fps <FPS>           Frames per second [default: 60]
  -o, --output <PATH>       Output directory [default: ./animations]
  -t, --template <NAME>     Use template as base
  -p, --preview-only        Generate preview only
  -w, --watch               Watch and auto-regenerate on prompt changes
  --background <COLOR>      Background color [default: #1a1a1a]
  --no-cache                Don't use cached results
  -h, --help               Print help
```

**Examples:**
```bash
# Basic usage
animaforge create "Rotating DNA helix"

# High quality 4K with custom duration
animaforge create "Solar system orbits" --quality 4k --duration 15

# Use template
animaforge create "Explain bubble sort" --template algorithm-visualization

# Preview mode (fast)
animaforge create "Graph traversal" --preview-only

# Watch mode (auto-regenerate)
animaforge create "Interactive plot" --watch

# Custom colors
animaforge create "Night sky" --background "#000033"
```

---

### `animaforge render`
Render Manim code to video

```bash
animaforge render <FILE> [OPTIONS]

Arguments:
  <FILE>  Path to Manim Python file

Options:
  -q, --quality <LEVEL>    Quality level [low|medium|high|4k]
  -s, --scene <NAME>       Specific scene to render (if multiple)
  --format <FORMAT>        Output format [mp4|gif|webm|mov] [default: mp4]
  --transparent            Render with transparent background
  --from-frame <N>         Start from frame N
  --to-frame <N>           End at frame N
  -h, --help              Print help
```

**Examples:**
```bash
# Render existing Manim file
animaforge render ./my_animation.py

# Specific scene only
animaforge render ./my_animation.py --scene ConstructScene

# Export as GIF
animaforge render ./my_animation.py --format gif

# Transparent background for overlays
animaforge render ./my_animation.py --transparent
```

---

### `animaforge preview`
Quick preview of animation

```bash
animaforge preview <FILE> [OPTIONS]

Arguments:
  <FILE>  Path to Manim Python file

Options:
  --fps <FPS>              Preview FPS [default: 15]
  --resolution <RES>       Resolution [480p|720p|1080p] [default: 480p]
  --open                   Auto-open in player
  -h, --help              Print help
```

**Examples:**
```bash
# Quick preview
animaforge preview ./animations/my_anim.py

# Higher quality preview
animaforge preview ./animations/my_anim.py --resolution 720p --fps 30

# Preview and open
animaforge preview ./animations/my_anim.py --open
```

---

### `animaforge publish`
Upload animation to marketplace

```bash
animaforge publish <FILE> [OPTIONS]

Arguments:
  <FILE>  Animation file to publish

Options:
  -t, --title <TITLE>          Animation title
  -d, --description <DESC>     Description
  -T, --tags <TAG>...          Tags (comma-separated)
  -l, --license <LICENSE>      License [MIT|CC0|CC-BY|Commercial]
  -p, --price <AMOUNT>         Price (0 for free) [default: 0]
  -c, --category <CAT>         Category
  --thumbnail <PATH>           Custom thumbnail
  --private                    Keep private (don't list publicly)
  -h, --help                  Print help
```

**Examples:**
```bash
# Publish with interactive prompts
animaforge publish ./animations/my_anim.mp4

# Publish with all metadata
animaforge publish ./animations/sorting.mp4 \
  --title "Bubble Sort Visualization" \
  --description "Clean animation showing bubble sort algorithm" \
  --tags "algorithm,sorting,education" \
  --license MIT \
  --price 0 \
  --category "Computer Science"

# Commercial animation
animaforge publish ./animations/product_demo.mp4 \
  --license Commercial \
  --price 29.99 \
  --private
```

---

### `animaforge search`
Search marketplace for animations

```bash
animaforge search <QUERY> [OPTIONS]

Arguments:
  <QUERY>  Search query

Options:
  -c, --category <CAT>     Filter by category
  -l, --license <LICENSE>  Filter by license
  -s, --sort <BY>         Sort by [popular|recent|rating|downloads]
  -n, --limit <N>         Number of results [default: 10]
  --free                  Free animations only
  -h, --help             Print help
```

**Examples:**
```bash
# Basic search
animaforge search "physics"

# Filtered search
animaforge search "machine learning" --category "AI/ML" --free

# Popular recent animations
animaforge search "mathematics" --sort popular --limit 20
```

---

### `animaforge install`
Download animation from marketplace

```bash
animaforge install <ANIMATION_ID> [OPTIONS]

Arguments:
  <ANIMATION_ID>  Animation ID or URL

Options:
  -o, --output <PATH>    Output directory [default: ./animations]
  --format <FORMAT>      Download format [mp4|gif|code]
  --with-source          Include source Manim code
  -h, --help            Print help
```

**Examples:**
```bash
# Install by ID
animaforge install anim-123456

# Install with source code
animaforge install anim-123456 --with-source

# Install to specific location
animaforge install anim-123456 --output ~/Videos/

# Get as GIF
animaforge install anim-123456 --format gif
```

---

### `animaforge config`
Manage configuration

```bash
animaforge config <SUBCOMMAND>

Subcommands:
  get <KEY>              Get configuration value
  set <KEY> <VALUE>      Set configuration value
  list                   List all configuration
  reset                  Reset to defaults
  edit                   Open config in editor
```

**Examples:**
```bash
# View current backend
animaforge config get backend

# Change backend
animaforge config set backend gemini

# Set API key
animaforge config set gemini.api_key "your-api-key"

# View all settings
animaforge config list

# Edit in $EDITOR
animaforge config edit
```

---

### `animaforge template`
Manage templates

```bash
animaforge template <SUBCOMMAND>

Subcommands:
  list                    List available templates
  show <NAME>             Show template details
  create <NAME>           Create new template
  use <NAME> <PROMPT>     Generate from template
  install <ID>            Install from marketplace
```

**Examples:**
```bash
# List all templates
animaforge template list

# View template details
animaforge template show math-equation

# Create animation from template
animaforge template use math-equation "E = mc²"

# Install community template
animaforge template install tmpl-789012
```

---

### `animaforge auth`
Authentication management

```bash
animaforge auth <SUBCOMMAND>

Subcommands:
  login                   Login to AnimaForge
  logout                  Logout
  status                  Show login status
  whoami                  Show current user
```

**Examples:**
```bash
# Login
animaforge auth login

# Check status
animaforge auth status

# Logout
animaforge auth logout
```

---

### `animaforge stats`
View usage statistics

```bash
animaforge stats [OPTIONS]

Options:
  -p, --period <PERIOD>    Time period [day|week|month|all] [default: week]
  --json                   Output as JSON
  -h, --help              Print help
```

**Examples:**
```bash
# This week's stats
animaforge stats

# All-time stats
animaforge stats --period all

# JSON output for scripts
animaforge stats --json
```

---

## Configuration

AnimaForge stores configuration in `~/.config/animaforge/config.toml`

### Configuration File Structure

```toml
[general]
output_dir = "./animations"
default_quality = "high"
default_duration = 5
default_fps = 60

[backend]
provider = "ollama"  # ollama|gemini|claude|openai
model = "llama3"
max_tokens = 4000
temperature = 0.7
streaming = true

[ollama]
host = "http://localhost:11434"
model = "llama3"

[gemini]
api_key = "your-api-key"
model = "gemini-pro"

[claude]
api_key = "your-api-key"
model = "claude-3-sonnet"

[openai]
api_key = "your-api-key"
model = "gpt-4"

[render]
default_resolution = "1080p"
default_format = "mp4"
enable_cache = true
parallel_jobs = 4

[marketplace]
username = "yourname"
auth_token = "your-token"
auto_upload_thumbnail = true

[ui]
color_scheme = "dark"
show_progress = true
verbose = false
```

### Environment Variables

```bash
# Override config file settings
export ANIMAFORGE_BACKEND=ollama
export ANIMAFORGE_MODEL=llama3
export ANIMAFORGE_API_KEY=your-api-key
export ANIMAFORGE_OUTPUT_DIR=/path/to/output
export ANIMAFORGE_QUALITY=high
```

---

## AI Backends

### Ollama (Local, Free)

**Setup:**
```bash
# Install Ollama
curl https://ollama.ai/install.sh | sh

# Pull model
ollama pull llama3

# Configure AnimaForge
animaforge config set backend ollama
animaforge config set ollama.model llama3
```

**Pros:**
- Free and private
- No API limits
- Fast for local execution

**Cons:**
- Requires good hardware (16GB+ RAM)
- Quality depends on model

---

### Gemini (Cloud)

**Setup:**
```bash
# Get API key from https://makersuite.google.com/app/apikey

# Configure
animaforge config set backend gemini
animaforge config set gemini.api_key "your-key"
animaforge config set gemini.model "gemini-1.5-pro"
```

**Pros:**
- High quality outputs
- Large context window
- Multimodal support

**Cons:**
- Requires API key
- Rate limits on free tier

---

### Claude (Cloud)

**Setup:**
```bash
# Get API key from https://console.anthropic.com/

# Configure
animaforge config set backend claude
animaforge config set claude.api_key "your-key"
animaforge config set claude.model "claude-3-sonnet-20240229"
```

**Pros:**
- Excellent code generation
- Long context windows
- Great for complex animations

**Cons:**
- Paid API
- Rate limits

---

### OpenAI (Cloud)

**Setup:**
```bash
# Get API key from https://platform.openai.com/

# Configure
animaforge config set backend openai
animaforge config set openai.api_key "your-key"
animaforge config set openai.model "gpt-4"
```

**Pros:**
- High quality
- Well-documented
- Reliable

**Cons:**
- More expensive
- Rate limits

---

## Advanced Usage

### Batch Processing

```bash
# Process multiple prompts from file
cat prompts.txt | while read prompt; do
  animaforge create "$prompt" --output ./batch_output/
done
```

### Pipeline Integration

```bash
# Generate → Edit → Publish
animaforge create "Cool animation" --output temp.py && \
code temp.py && \
animaforge render temp.py && \
animaforge publish temp.mp4
```

### Custom Prompt Engineering

```bash
# Use prompt file for complex descriptions
animaforge create --prompt-file complex_animation.txt
```

### Scripting

```bash
#!/bin/bash
# Auto-generate educational series

topics=("limits" "derivatives" "integrals" "series")

for topic in "${topics[@]}"; do
  animaforge create "Calculus: $topic explained" \
    --template math-education \
    --output "./calculus_series/" \
    --tags "math,calculus,$topic"
done
```

### Animation Composition

```bash
# Generate multiple scenes and combine
animaforge create "Intro scene" --duration 3 --output intro.py
animaforge create "Main content" --duration 10 --output main.py
animaforge create "Outro" --duration 2 --output outro.py

# Combine (feature coming soon)
animaforge compose intro.py main.py outro.py --output final.mp4
```

---

## Troubleshooting

### Common Issues

**Issue**: `Error: Ollama not running`
```bash
# Solution: Start Ollama
ollama serve
```

**Issue**: `Error: ffmpeg not found`
```bash
# Solution: Install ffmpeg
# macOS
brew install ffmpeg

# Ubuntu
sudo apt install ffmpeg
```

**Issue**: `Manim compilation failed`
```bash
# Solution: Check LaTeX installation
latex --version

# Install if missing
# macOS
brew install texlive

# Ubuntu
sudo apt install texlive-full
```

**Issue**: `Animation quality is poor`
```bash
# Solution: Increase quality and/or change model
animaforge config set default_quality 4k
animaforge config set backend claude  # Better code generation
```

### Debug Mode

```bash
# Enable verbose logging
animaforge --verbose create "test animation"

# Check logs
tail -f ~/.config/animaforge/logs/animaforge.log
```

### Reset Configuration

```bash
# Reset to defaults
animaforge config reset

# Re-initialize
animaforge init --interactive
```

---

## Tips & Tricks

### 1. Better Prompts = Better Animations

```bash
# ❌ Vague
animaforge create "show math"

# ✅ Specific
animaforge create "Animated proof of Pythagorean theorem using colored squares, rotating from a to show a² + b² = c²"
```

### 2. Use Templates

```bash
# Start with template, customize with prompt
animaforge template use physics-simulation "Two planets orbiting with gravitational attraction"
```

### 3. Iterate Quickly

```bash
# Use preview mode for fast iteration
animaforge create "test idea" --preview-only

# Once happy, render full quality
animaforge render ./animations/test_idea.py --quality 4k
```

### 4. Build a Library

```bash
# Create project-specific animations
mkdir my_video_project
cd my_video_project

animaforge create "Intro sequence" --output ./animations/
animaforge create "Transition 1" --output ./animations/
animaforge create "Outro" --output ./animations/
```

---

## Keyboard Shortcuts (Interactive Mode)

When in watch mode (`--watch`), these shortcuts are available:

- `r` - Regenerate animation
- `e` - Edit prompt
- `q` - Change quality
- `p` - Toggle preview mode
- `s` - Save and exit
- `Ctrl+C` - Cancel

---

## Getting Help

```bash
# General help
animaforge --help

# Command-specific help
animaforge create --help

# Show examples
animaforge examples create

# Interactive tutorial
animaforge tutorial
```

---

## Contributing

Found a bug? Have a feature request? Want to contribute?

- [GitHub Issues](https://github.com/yourusername/animaforge/issues)
- [Discord Community](https://discord.gg/animaforge)
- [Contributing Guide](../CONTRIBUTING.md)

---

**Made with ❤️ by the AnimaForge team**

Happy animating! 🎬✨
