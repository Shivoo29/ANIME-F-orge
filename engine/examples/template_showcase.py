#!/usr/bin/env python3
"""
Template showcase - demonstrates all built-in templates.

This script generates and optionally renders all available templates.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from animaforge_engine import (
    list_templates,
    get_template,
    validate_and_report,
    render_animation,
)


def showcase_template(template_name, output_base="/tmp/animaforge_templates"):
    """Showcase a single template."""
    print(f"\n{'='*60}")
    print(f"Template: {template_name}")
    print('='*60)

    # Define parameters for each template
    params = {
        "simple_text": {
            "title": "AnimaForge Engine",
            "subtitle": "Beautiful Mathematical Animations",
            "font_size": 60,
            "subtitle_size": 40,
        },
        "geometric_shapes": {},  # Uses defaults
        "math_equation": {
            "equation": r"\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}",
            "simplified": r"\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}",
        },
        "graph_chart": {
            "function": "x**2",
            "label": r"y = x^2",
            "function_at_start": "9",
        },
        "transformation": {
            "start_shape": "Square()",
            "mid_shape": "Circle()",
            "end_shape": "Triangle()",
            "start_color": "BLUE",
            "mid_color": "YELLOW",
            "end_color": "RED",
        },
    }

    # Get template code
    code = get_template(template_name, **params.get(template_name, {}))

    # Validate
    report = validate_and_report(code)

    print(f"\nValidation: {'✓ Valid' if report['valid'] else '✗ Invalid'}")
    if report['scene_class']:
        print(f"Scene Class: {report['scene_class']}")

    if report['error']:
        print(f"Error: {report['error']}")
        return None

    # Show first few lines
    lines = code.split('\n')
    print(f"\nCode Preview (first 15 lines):")
    print('-' * 60)
    for i, line in enumerate(lines[:15], 1):
        print(f"{i:3}: {line}")
    if len(lines) > 15:
        print(f"... ({len(lines) - 15} more lines)")
    print('-' * 60)

    return code


def main():
    """Showcase all templates."""
    print("\n")
    print("╔════════════════════════════════════════════════════════════╗")
    print("║            AnimaForge Template Showcase                   ║")
    print("╚════════════════════════════════════════════════════════════╝")

    templates = list_templates()
    print(f"\nFound {len(templates)} templates: {', '.join(templates)}")

    codes = {}

    for template_name in templates:
        try:
            code = showcase_template(template_name)
            if code:
                codes[template_name] = code
        except Exception as e:
            print(f"\n✗ Error with template '{template_name}': {e}")

    print(f"\n{'='*60}")
    print(f"Summary: Successfully loaded {len(codes)}/{len(templates)} templates")
    print('='*60)

    # Ask if user wants to render
    print("\nTo render these templates, use:")
    print("  from animaforge_engine import get_template, render_animation")
    print("  code = get_template('simple_text')")
    print("  render_animation(code, '/tmp/output', quality='low')")

    return codes


if __name__ == "__main__":
    main()
