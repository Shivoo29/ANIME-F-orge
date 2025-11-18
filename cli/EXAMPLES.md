# AnimaForge CLI - Usage Examples

## Basic Examples

### 1. First Time Setup

Configure your LLM backend:

```bash
# Using Ollama (local, free)
animaforge config set --backend ollama --model llama2

# Check configuration
animaforge config list
```

### 2. Create Your First Animation

```bash
# Simple animation
animaforge create "A blue circle that rotates 360 degrees"

# With auto-render
animaforge create "A red square transforming into a circle" --render

# Specify output location
animaforge create "Mathematical sine wave" --output ~/my_animations/sine.py
```

### 3. Render an Animation

```bash
# Default quality (720p @ 30fps)
animaforge render animation_1234567890.py

# High quality
animaforge render animation_1234567890.py --quality high

# Specify output video name
animaforge render my_animation.py --output final_video.mp4
```

### 4. Search Marketplace

```bash
# Basic search
animaforge search "mathematics"

# Limited results
animaforge search "physics" --limit 5

# Search for specific topics
animaforge search "calculus visualization"
```

### 5. Publish to Marketplace

```bash
animaforge publish my_animation.py
```

You'll be prompted for:
- Title
- Description
- Tags

## Advanced Examples

### Complete Workflow

```bash
# 1. Create animation with specific prompt
animaforge create "Pythagorean theorem proof with animated triangles" \
  --output pythagorean.py

# 2. Review the generated code
cat pythagorean.py

# 3. Render in high quality
animaforge render pythagorean.py --quality high --output pythagorean_hq.mp4

# 4. Publish to share with others
animaforge publish pythagorean.py
```

### Using Different LLM Backends

```bash
# Switch to Gemini
animaforge config set --backend gemini
animaforge config set --model gemini-pro
animaforge config set --api-key your_gemini_api_key

# Switch to Claude
animaforge config set --backend claude
animaforge config set --model claude-3-sonnet-20240229
animaforge config set --api-key your_claude_api_key

# Back to Ollama (no API key needed)
animaforge config set --backend ollama
animaforge config set --model llama2
```

### Batch Creation

Create multiple animations:

```bash
# Script to create multiple animations
for prompt in \
  "Circle growing and shrinking" \
  "Square rotating" \
  "Text fading in and out" \
  "Graph of y=x^2"
do
  echo "Creating: $prompt"
  animaforge create "$prompt" --render
done
```

### Configuration Management

```bash
# View specific config value
animaforge config get backend
animaforge config get model

# View all configuration
animaforge config list

# Set output directory for all animations
animaforge config set --output-dir ~/Documents/AnimaForge

# View config file location
cat ~/.animaforge/config.toml
```

## Example Prompts

Here are some great prompts to try:

### Mathematics

```bash
animaforge create "Animate the Fibonacci spiral with golden ratio"
animaforge create "Visualization of the quadratic formula solving x^2 + 2x - 3 = 0"
animaforge create "Taylor series expansion of e^x with animated terms"
animaforge create "3D graph of z = x^2 + y^2 rotating"
```

### Physics

```bash
animaforge create "Simple harmonic motion of a pendulum"
animaforge create "Wave interference pattern between two sources"
animaforge create "Projectile motion with trajectory path"
animaforge create "Electric field lines around positive and negative charges"
```

### Computer Science

```bash
animaforge create "Binary search tree insertion animation"
animaforge create "Bubble sort algorithm visualized with bars"
animaforge create "Graph traversal using breadth-first search"
animaforge create "Recursion tree for Fibonacci sequence"
```

### Abstract/Artistic

```bash
animaforge create "Mandelbrot set zoom animation"
animaforge create "Color gradient morphing from red to blue"
animaforge create "Geometric patterns forming a kaleidoscope"
animaforge create "Particles forming the word ANIMATE"
```

## Quality Comparison

| Quality | Resolution | FPS | Best For | Render Time |
|---------|-----------|-----|----------|-------------|
| low     | 480p      | 15  | Quick previews | Fastest |
| medium  | 720p      | 30  | Standard sharing | Moderate |
| high    | 1080p     | 60  | Professional use | Slower |

## Tips & Tricks

### 1. Iterative Development

```bash
# Create and review code first
animaforge create "Complex animation idea" --output draft.py

# Edit the generated code manually if needed
nano draft.py

# Render with low quality to preview
animaforge render draft.py --quality low

# Once satisfied, render in high quality
animaforge render draft.py --quality high
```

### 2. Using with Python Engine

The CLI integrates with the AnimaForge Python engine:

```bash
# The create command calls the Python engine internally
# But you can also use it directly:
cd ../engine
python -m animaforge_engine.generator "Your prompt here"
```

### 3. Environment Variables

You can also configure via environment variables:

```bash
export ANIMAFORGE_BACKEND=ollama
export ANIMAFORGE_MODEL=llama2
export OLLAMA_ENDPOINT=http://localhost:11434

animaforge create "test animation"
```

### 4. Debugging

```bash
# If rendering fails, check:
manim --version  # Ensure Manim is installed

# If LLM connection fails:
curl http://localhost:11434/api/tags  # For Ollama

# View config
animaforge config list
```

## Common Issues

### Issue: "Manim not found"

**Solution:**
```bash
pip install manim
# or
pip install manimce
```

### Issue: "Failed to connect to Ollama"

**Solution:**
```bash
# Start Ollama server
ollama serve

# In another terminal
animaforge create "test"
```

### Issue: "No Scene class found"

**Solution:** The generated code must contain a class that inherits from `Scene`. Try regenerating with a clearer prompt.

## Integration with Other Tools

### Use with Git

```bash
# Initialize repo for your animations
mkdir my-animations
cd my-animations
git init

# Create animations
animaforge create "intro animation" --output intro.py
animaforge create "outro animation" --output outro.py

# Commit
git add *.py
git commit -m "Add intro and outro animations"
```

### Use in Scripts

```bash
#!/bin/bash
# generate_course_animations.sh

COURSE_TOPICS=(
  "Introduction to Calculus"
  "Derivatives explained"
  "Integration basics"
  "Fundamental theorem"
)

for topic in "${COURSE_TOPICS[@]}"; do
  filename=$(echo "$topic" | tr ' ' '_' | tr '[:upper:]' '[:lower:]').py
  animaforge create "$topic visualization" --output "$filename"
  animaforge render "$filename" --quality high
done
```

## Next Steps

- Read the [README.md](README.md) for installation instructions
- Check the [API documentation](../API.md) for programmatic access
- Explore the [Python engine](../engine/) for advanced customization
- Join the community and share your animations!
