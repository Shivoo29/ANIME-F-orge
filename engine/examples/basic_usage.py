#!/usr/bin/env python3
"""
Basic usage examples for AnimaForge Engine.

This script demonstrates the core functionality of the engine:
1. Generating animation code from prompts
2. Validating generated code
3. Rendering animations
4. Using templates
"""

import os
import sys
from pathlib import Path

# Add parent directory to path to import animaforge_engine
sys.path.insert(0, str(Path(__file__).parent.parent))

from animaforge_engine import (
    generate_animation_code,
    validate_and_report,
    render_animation,
    get_template,
    list_templates,
)


def example_1_generate_code():
    """Example 1: Generate animation code from a text prompt."""
    print("=" * 60)
    print("Example 1: Generate Animation Code")
    print("=" * 60)

    prompt = "Create a blue circle that grows and then transforms into a red square"

    print(f"\nPrompt: {prompt}")
    print("\nGenerating code using Ollama (local LLM)...")

    try:
        code = generate_animation_code(
            prompt=prompt,
            llm_backend="ollama",
            max_retries=2
        )

        print("\n✓ Generated code:")
        print("-" * 60)
        print(code)
        print("-" * 60)

        return code

    except Exception as e:
        print(f"\n✗ Error: {e}")
        print("\nMake sure Ollama is running: ollama serve")
        return None


def example_2_validate_code(code):
    """Example 2: Validate generated code."""
    print("\n" + "=" * 60)
    print("Example 2: Validate Code")
    print("=" * 60)

    if not code:
        print("\nNo code to validate. Skipping.")
        return False

    print("\nValidating code...")

    report = validate_and_report(code)

    print(f"\n✓ Validation Report:")
    print(f"  - Valid: {report['valid']}")
    print(f"  - Scene Class: {report['scene_class']}")
    print(f"  - Imports: {len(report['imports'])}")

    if report['error']:
        print(f"  - Error: {report['error']}")

    if report['warnings']:
        print(f"  - Warnings:")
        for warning in report['warnings']:
            print(f"    • {warning}")

    return report['valid']


def example_3_render_animation(code):
    """Example 3: Render the animation."""
    print("\n" + "=" * 60)
    print("Example 3: Render Animation")
    print("=" * 60)

    if not code:
        print("\nNo code to render. Skipping.")
        return None

    output_path = "/tmp/animaforge_demo"
    print(f"\nRendering animation to: {output_path}")
    print("Quality: low (for fast preview)")

    try:
        video_path = render_animation(
            code=code,
            output_path=output_path,
            quality="low",  # Use low quality for fast rendering
            fps=30
        )

        print(f"\n✓ Animation rendered successfully!")
        print(f"  Video path: {video_path}")
        print(f"  File size: {os.path.getsize(video_path) / 1024:.2f} KB")

        return video_path

    except Exception as e:
        print(f"\n✗ Rendering failed: {e}")
        return None


def example_4_use_template():
    """Example 4: Use a pre-built template."""
    print("\n" + "=" * 60)
    print("Example 4: Use Pre-built Template")
    print("=" * 60)

    # List available templates
    templates = list_templates()
    print(f"\nAvailable templates: {', '.join(templates)}")

    # Get a template
    template_name = "simple_text"
    print(f"\nUsing template: {template_name}")

    code = get_template(
        template_name,
        title="AnimaForge Engine",
        subtitle="AI-Powered Animation Generation",
        font_size=60,
        subtitle_size=40
    )

    print("\n✓ Template code generated:")
    print("-" * 60)
    print(code[:500] + "..." if len(code) > 500 else code)
    print("-" * 60)

    # Validate template
    is_valid = example_2_validate_code(code)

    if is_valid:
        print("\n✓ Template is valid and ready to render!")

    return code


def example_5_complete_workflow():
    """Example 5: Complete workflow from prompt to video."""
    print("\n" + "=" * 60)
    print("Example 5: Complete Workflow")
    print("=" * 60)

    prompt = "Show a mathematical equation E=mc² appearing with a fade in effect"

    print(f"\nPrompt: {prompt}")

    try:
        # Step 1: Generate
        print("\n[Step 1/3] Generating code...")
        code = generate_animation_code(prompt, llm_backend="ollama")
        print("✓ Code generated")

        # Step 2: Validate
        print("\n[Step 2/3] Validating code...")
        report = validate_and_report(code)
        if not report['valid']:
            print(f"✗ Validation failed: {report['error']}")
            return
        print("✓ Code validated")

        # Step 3: Render
        print("\n[Step 3/3] Rendering animation...")
        video_path = render_animation(
            code=code,
            output_path="/tmp/animaforge_complete",
            quality="low"
        )
        print(f"✓ Complete! Video: {video_path}")

    except Exception as e:
        print(f"\n✗ Workflow failed: {e}")


def main():
    """Run all examples."""
    print("\n")
    print("╔════════════════════════════════════════════════════════════╗")
    print("║         AnimaForge Engine - Basic Usage Examples          ║")
    print("╚════════════════════════════════════════════════════════════╝")

    # Example 1: Generate code
    code = example_1_generate_code()

    if code:
        # Example 2: Validate
        is_valid = example_2_validate_code(code)

        # Example 3: Render
        if is_valid:
            video_path = example_3_render_animation(code)

    # Example 4: Use template
    template_code = example_4_use_template()

    # Example 5: Complete workflow
    # Uncomment to run:
    # example_5_complete_workflow()

    print("\n" + "=" * 60)
    print("Examples completed!")
    print("=" * 60)
    print("\nNote: Make sure you have:")
    print("  1. Ollama running (ollama serve)")
    print("  2. Manim installed (pip install manim)")
    print("  3. FFmpeg installed")
    print("\n")


if __name__ == "__main__":
    main()
