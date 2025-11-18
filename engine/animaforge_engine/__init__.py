"""
AnimaForge Engine - AI-powered animation generation with Manim integration.

This package provides tools for generating, validating, and rendering
mathematical animations using Manim and AI language models.

Main Components:
    - generator: Generate Manim code from text prompts using LLMs
    - validator: Validate and analyze Manim code
    - renderer: Render animations using Manim
    - templates: Pre-built animation templates
    - prompts: System prompts for LLM guidance

Quick Start:
    >>> from animaforge_engine import generate_animation_code, render_animation
    >>>
    >>> # Generate code from a prompt
    >>> code = generate_animation_code(
    ...     "Create a blue circle that transforms into a red square",
    ...     llm_backend="ollama"
    ... )
    >>>
    >>> # Render the animation
    >>> video_path = render_animation(code, "/tmp/output", quality="medium")
    >>> print(f"Video saved to: {video_path}")
"""

__version__ = "0.1.0"
__author__ = "AnimaForge Team"
__license__ = "MIT"

# Import main functions from modules
from .generator import (
    generate_animation_code,
    batch_generate,
)

from .validator import (
    validate_manim_code,
    validate_and_report,
    extract_scene_class_name,
    clean_code,
    get_imports,
)

from .renderer import (
    render_animation,
    render_preview,
    render_batch,
    get_render_info,
    clean_render_cache,
)

from .templates import (
    get_template,
    list_templates,
    SIMPLE_TEXT_TEMPLATE,
    GEOMETRIC_SHAPES_TEMPLATE,
    MATH_EQUATION_TEMPLATE,
    GRAPH_CHART_TEMPLATE,
    TRANSFORMATION_TEMPLATE,
)

from .prompts import (
    MANIM_CODE_GENERATION_PROMPT,
    MANIM_CODE_REFINEMENT_PROMPT,
    CODE_EXTRACTION_PROMPT,
)

# Define public API
__all__ = [
    # Generator functions
    "generate_animation_code",
    "batch_generate",
    # Validator functions
    "validate_manim_code",
    "validate_and_report",
    "extract_scene_class_name",
    "clean_code",
    "get_imports",
    # Renderer functions
    "render_animation",
    "render_preview",
    "render_batch",
    "get_render_info",
    "clean_render_cache",
    # Template functions
    "get_template",
    "list_templates",
    # Templates
    "SIMPLE_TEXT_TEMPLATE",
    "GEOMETRIC_SHAPES_TEMPLATE",
    "MATH_EQUATION_TEMPLATE",
    "GRAPH_CHART_TEMPLATE",
    "TRANSFORMATION_TEMPLATE",
    # Prompts
    "MANIM_CODE_GENERATION_PROMPT",
    "MANIM_CODE_REFINEMENT_PROMPT",
    "CODE_EXTRACTION_PROMPT",
]


# Package metadata
def get_version() -> str:
    """Get the package version."""
    return __version__


def get_info() -> dict:
    """
    Get package information.

    Returns:
        Dictionary containing package metadata
    """
    return {
        "name": "animaforge-engine",
        "version": __version__,
        "author": __author__,
        "license": __license__,
        "description": "AI-powered animation generation with Manim integration",
    }
