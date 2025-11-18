# AnimaForge Engine

AI-powered animation generation engine with real Manim integration.

## Features

- 🤖 **AI Code Generation**: Generate Manim animations from natural language prompts
- ✅ **Code Validation**: AST-based validation for Manim code
- 🎬 **Real Rendering**: Actual video rendering using Manim Community Edition
- 🎨 **Templates**: Pre-built animation templates for common patterns
- 🔌 **Multiple LLM Backends**: Support for Ollama, Gemini, and Claude
- 🚀 **Batch Processing**: Generate and render multiple animations

## Installation

```bash
cd engine
pip install -e .
```

Or install with development dependencies:

```bash
pip install -e ".[dev]"
```

## Quick Start

### 1. Generate Animation Code

```python
from animaforge_engine import generate_animation_code

# Generate Manim code from a text prompt
code = generate_animation_code(
    prompt="Create a blue circle that transforms into a red square",
    llm_backend="ollama"  # or "gemini" or "claude"
)

print(code)
```

### 2. Validate Code

```python
from animaforge_engine import validate_manim_code, validate_and_report

# Simple validation
is_valid, error = validate_manim_code(code)
print(f"Valid: {is_valid}, Error: {error}")

# Detailed validation report
report = validate_and_report(code)
print(report)
```

### 3. Render Animation

```python
from animaforge_engine import render_animation

# Render the animation
video_path = render_animation(
    code=code,
    output_path="/tmp/animations",
    quality="medium"  # low, medium, high, or 4k
)

print(f"Video saved to: {video_path}")
```

### 4. Use Templates

```python
from animaforge_engine import get_template, list_templates

# List available templates
templates = list_templates()
print(templates)

# Get a template with custom parameters
code = get_template(
    "simple_text",
    title="Welcome to AnimaForge",
    subtitle="AI-Powered Animations",
    font_size=60
)

# Render the template
video_path = render_animation(code, "/tmp/output")
```

## Complete Example

```python
from animaforge_engine import (
    generate_animation_code,
    validate_and_report,
    render_animation
)

# 1. Generate code from prompt
prompt = "Show a sine wave being drawn with a moving dot following the curve"
code = generate_animation_code(prompt, llm_backend="ollama")

# 2. Validate the code
report = validate_and_report(code)
if report['valid']:
    print(f"✓ Code is valid! Scene class: {report['scene_class']}")

    # 3. Render the animation
    video_path = render_animation(
        code=code,
        output_path="/tmp/my_animation",
        quality="high",
        fps=60
    )

    print(f"✓ Animation rendered: {video_path}")
else:
    print(f"✗ Validation failed: {report['error']}")
```

## LLM Backend Configuration

### Ollama (Local)

1. Install Ollama: https://ollama.ai
2. Start Ollama server: `ollama serve`
3. Pull a model: `ollama pull codellama`
4. Set environment variables (optional):

```bash
export OLLAMA_URL=http://localhost:11434
export OLLAMA_MODEL=codellama
```

### Gemini (Google)

1. Get API key from: https://makersuite.google.com/app/apikey
2. Set environment variable:

```bash
export GEMINI_API_KEY=your_api_key_here
```

### Claude (Anthropic)

1. Get API key from: https://console.anthropic.com/
2. Set environment variable:

```bash
export ANTHROPIC_API_KEY=your_api_key_here
```

## API Reference

### Generator

#### `generate_animation_code(prompt, llm_backend, max_retries, model)`
Generate Manim code from a text prompt.

**Parameters:**
- `prompt` (str): Text description of the animation
- `llm_backend` (str): LLM service ('ollama', 'gemini', 'claude')
- `max_retries` (int): Maximum retry attempts (default: 3)
- `model` (str, optional): Specific model to use

**Returns:** Valid Manim Python code as string

#### `batch_generate(prompts, llm_backend, **kwargs)`
Generate multiple animations from a list of prompts.

### Validator

#### `validate_manim_code(code)`
Validate Manim code for syntax and structure.

**Parameters:**
- `code` (str): Python code to validate

**Returns:** Tuple of (is_valid, error_message)

#### `validate_and_report(code)`
Get detailed validation report.

**Returns:** Dictionary with validation details

### Renderer

#### `render_animation(code, output_path, quality, format, ...)`
Render a Manim animation from code.

**Parameters:**
- `code` (str): Valid Manim code
- `output_path` (str): Output directory
- `quality` (str): 'low', 'medium', 'high', or '4k' (default: 'medium')
- `format` (str): 'mp4', 'mov', 'gif', 'png' (default: 'mp4')
- `background_color` (str, optional): Background color
- `transparent` (bool): Transparent background (default: False)
- `fps` (int): Frames per second (default: 60)

**Returns:** Path to rendered video file

#### `render_preview(code, output_path, show)`
Render a quick low-quality preview.

#### `render_batch(codes, output_path, quality, **kwargs)`
Render multiple animations in batch.

### Templates

#### `get_template(template_name, **kwargs)`
Get a pre-built template with parameter substitution.

**Available Templates:**
- `simple_text`: Text animations with title and subtitle
- `geometric_shapes`: Circle, square, triangle animations
- `math_equation`: Mathematical formula animations
- `graph_chart`: Function graphs with moving dots
- `transformation`: Shape transformation sequences

#### `list_templates()`
List all available template names.

## Quality Levels

| Quality | Resolution | FPS | Speed | Use Case |
|---------|-----------|-----|-------|----------|
| low     | 480p      | 15  | Fast  | Preview, testing |
| medium  | 854x480   | 30  | Medium| Development |
| high    | 1080p     | 60  | Slow  | Production |
| 4k      | 3840x2160 | 60  | Slowest | High-quality final output |

## Project Structure

```
engine/
├── setup.py                      # Package setup
├── pyproject.toml                # Modern Python packaging config
├── requirements.txt              # Dependencies
├── README.md                     # This file
└── animaforge_engine/
    ├── __init__.py              # Package exports
    ├── generator.py             # Code generation from prompts
    ├── validator.py             # Validate Manim code
    ├── renderer.py              # Render animations
    ├── templates/
    │   ├── __init__.py
    │   └── base.py              # Base animation templates
    └── prompts/
        ├── __init__.py
        └── system.py            # System prompts for LLMs
```

## Development

### Run Tests

```bash
pytest
```

### Format Code

```bash
black animaforge_engine/
```

### Type Checking

```bash
mypy animaforge_engine/
```

### Linting

```bash
ruff check animaforge_engine/
```

## Requirements

- Python 3.9+
- Manim Community Edition 0.18.0+
- FFmpeg (for video rendering)
- LaTeX (for math equations)

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
