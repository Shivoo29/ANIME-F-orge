#!/usr/bin/env python3
"""
AnimaForge Engine CLI - Command-line interface for animation generation.

Usage:
    python animaforge_cli.py generate "your prompt here" --backend ollama
    python animaforge_cli.py template simple_text --render
    python animaforge_cli.py validate code.py
    python animaforge_cli.py render code.py --quality high
"""

import argparse
import sys
from pathlib import Path

from animaforge_engine import (
    generate_animation_code,
    validate_and_report,
    render_animation,
    get_template,
    list_templates,
    get_info,
)


def cmd_generate(args):
    """Generate animation code from a prompt."""
    print(f"Generating animation from prompt: {args.prompt}")
    print(f"Using backend: {args.backend}")

    try:
        code = generate_animation_code(
            prompt=args.prompt,
            llm_backend=args.backend,
            max_retries=args.retries,
            model=args.model,
        )

        # Save to file if specified
        if args.output:
            output_file = Path(args.output)
            output_file.write_text(code)
            print(f"\n✓ Code saved to: {output_file}")
        else:
            print("\n" + "="*60)
            print("Generated Code:")
            print("="*60)
            print(code)
            print("="*60)

        # Optionally render
        if args.render:
            print("\nRendering animation...")
            render_path = args.render_output or "/tmp/animaforge_output"
            video = render_animation(code, render_path, quality=args.quality)
            print(f"✓ Video rendered: {video}")

        return 0

    except Exception as e:
        print(f"\n✗ Error: {e}", file=sys.stderr)
        return 1


def cmd_template(args):
    """Use a pre-built template."""
    templates = list_templates()

    if args.list:
        print("Available templates:")
        for i, template in enumerate(templates, 1):
            print(f"  {i}. {template}")
        return 0

    if args.name not in templates:
        print(f"✗ Unknown template: {args.name}", file=sys.stderr)
        print(f"Available templates: {', '.join(templates)}")
        return 1

    print(f"Using template: {args.name}")

    try:
        # Parse parameters
        params = {}
        if args.params:
            for param in args.params:
                key, value = param.split("=", 1)
                params[key] = value

        code = get_template(args.name, **params)

        # Save or print
        if args.output:
            output_file = Path(args.output)
            output_file.write_text(code)
            print(f"✓ Template saved to: {output_file}")
        else:
            print("\n" + "="*60)
            print(f"Template: {args.name}")
            print("="*60)
            print(code)
            print("="*60)

        # Optionally render
        if args.render:
            print("\nRendering template...")
            render_path = args.render_output or "/tmp/animaforge_output"
            video = render_animation(code, render_path, quality=args.quality)
            print(f"✓ Video rendered: {video}")

        return 0

    except Exception as e:
        print(f"\n✗ Error: {e}", file=sys.stderr)
        return 1


def cmd_validate(args):
    """Validate Manim code."""
    code_file = Path(args.file)

    if not code_file.exists():
        print(f"✗ File not found: {code_file}", file=sys.stderr)
        return 1

    code = code_file.read_text()

    print(f"Validating: {code_file}")

    report = validate_and_report(code)

    print("\nValidation Report:")
    print("="*60)
    print(f"Valid: {'✓ Yes' if report['valid'] else '✗ No'}")

    if report['error']:
        print(f"Error: {report['error']}")

    if report['scene_class']:
        print(f"Scene Class: {report['scene_class']}")

    if report['imports']:
        print(f"Imports: {len(report['imports'])}")
        for imp in report['imports']:
            print(f"  - {imp}")

    if report['warnings']:
        print(f"\nWarnings:")
        for warning in report['warnings']:
            print(f"  ⚠ {warning}")

    print("="*60)

    return 0 if report['valid'] else 1


def cmd_render(args):
    """Render Manim code."""
    code_file = Path(args.file)

    if not code_file.exists():
        print(f"✗ File not found: {code_file}", file=sys.stderr)
        return 1

    code = code_file.read_text()

    print(f"Rendering: {code_file}")
    print(f"Quality: {args.quality}")
    print(f"Output: {args.output}")

    try:
        video_path = render_animation(
            code=code,
            output_path=args.output,
            quality=args.quality,
            format=args.format,
            background_color=args.background,
            transparent=args.transparent,
            fps=args.fps,
        )

        print(f"\n✓ Rendering complete!")
        print(f"  Video: {video_path}")

        return 0

    except Exception as e:
        print(f"\n✗ Rendering failed: {e}", file=sys.stderr)
        return 1


def cmd_info(args):
    """Show package information."""
    info = get_info()

    print("\nAnimaForge Engine")
    print("="*60)
    print(f"Version: {info['version']}")
    print(f"Author: {info['author']}")
    print(f"License: {info['license']}")
    print(f"Description: {info['description']}")
    print("="*60)

    print("\nAvailable Templates:")
    for template in list_templates():
        print(f"  - {template}")

    return 0


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="AnimaForge Engine - AI-powered animation generation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )

    subparsers = parser.add_subparsers(dest="command", help="Available commands")

    # Generate command
    gen_parser = subparsers.add_parser("generate", help="Generate animation from prompt")
    gen_parser.add_argument("prompt", help="Animation description")
    gen_parser.add_argument("--backend", "-b", default="ollama",
                           choices=["ollama", "gemini", "claude"],
                           help="LLM backend (default: ollama)")
    gen_parser.add_argument("--model", "-m", help="Specific model to use")
    gen_parser.add_argument("--output", "-o", help="Output file path")
    gen_parser.add_argument("--retries", "-r", type=int, default=3,
                           help="Max retry attempts (default: 3)")
    gen_parser.add_argument("--render", action="store_true",
                           help="Render after generation")
    gen_parser.add_argument("--render-output", help="Render output directory")
    gen_parser.add_argument("--quality", "-q", default="medium",
                           choices=["low", "medium", "high", "4k"],
                           help="Render quality (default: medium)")
    gen_parser.set_defaults(func=cmd_generate)

    # Template command
    tmpl_parser = subparsers.add_parser("template", help="Use pre-built template")
    tmpl_parser.add_argument("name", nargs="?", help="Template name")
    tmpl_parser.add_argument("--list", "-l", action="store_true",
                            help="List available templates")
    tmpl_parser.add_argument("--params", "-p", nargs="+",
                            help="Template parameters (key=value)")
    tmpl_parser.add_argument("--output", "-o", help="Output file path")
    tmpl_parser.add_argument("--render", action="store_true",
                            help="Render template")
    tmpl_parser.add_argument("--render-output", help="Render output directory")
    tmpl_parser.add_argument("--quality", "-q", default="medium",
                            choices=["low", "medium", "high", "4k"],
                            help="Render quality (default: medium)")
    tmpl_parser.set_defaults(func=cmd_template)

    # Validate command
    val_parser = subparsers.add_parser("validate", help="Validate Manim code")
    val_parser.add_argument("file", help="Python file to validate")
    val_parser.set_defaults(func=cmd_validate)

    # Render command
    render_parser = subparsers.add_parser("render", help="Render Manim code")
    render_parser.add_argument("file", help="Python file to render")
    render_parser.add_argument("--output", "-o", default="/tmp/animaforge_output",
                              help="Output directory (default: /tmp/animaforge_output)")
    render_parser.add_argument("--quality", "-q", default="medium",
                              choices=["low", "medium", "high", "4k"],
                              help="Quality level (default: medium)")
    render_parser.add_argument("--format", "-f", default="mp4",
                              help="Output format (default: mp4)")
    render_parser.add_argument("--background", "-bg",
                              help="Background color")
    render_parser.add_argument("--transparent", "-t", action="store_true",
                              help="Transparent background")
    render_parser.add_argument("--fps", type=int, default=60,
                              help="Frames per second (default: 60)")
    render_parser.set_defaults(func=cmd_render)

    # Info command
    info_parser = subparsers.add_parser("info", help="Show package information")
    info_parser.set_defaults(func=cmd_info)

    # Parse arguments
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 0

    # Execute command
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
