"""Renderer for executing and rendering Manim animations."""

import os
import subprocess
import tempfile
import shutil
from pathlib import Path
from typing import Optional, Literal

from .validator import validate_manim_code, extract_scene_class_name

QualityLevel = Literal["low", "medium", "high", "4k"]


def render_animation(
    code: str,
    output_path: str,
    quality: QualityLevel = "medium",
    format: str = "mp4",
    background_color: Optional[str] = None,
    transparent: bool = False,
    fps: int = 60,
) -> str:
    """
    Render a Manim animation from code.

    Args:
        code: Valid Manim Python code containing a Scene class
        output_path: Directory where the output video will be saved
        quality: Render quality level ('low', 'medium', 'high', '4k')
        format: Output format ('mp4', 'mov', 'gif', 'png')
        background_color: Background color (e.g., 'WHITE', 'BLACK', '#1a1a1a')
        transparent: Render with transparent background
        fps: Frames per second (default: 60)

    Returns:
        Path to the rendered video file

    Raises:
        ValueError: If code is invalid or quality level is not supported
        RuntimeError: If rendering fails

    Example:
        >>> code = '''
        ... from manim import *
        ... class Test(Scene):
        ...     def construct(self):
        ...         circle = Circle()
        ...         self.play(Create(circle))
        ... '''
        >>> video_path = render_animation(code, "/tmp/output", quality="low")
        >>> print(video_path)
        /tmp/output/Test.mp4
    """
    # Validate quality level
    quality_flags = {
        "low": "-ql",  # Low quality (fastest, 480p 15fps)
        "medium": "-qm",  # Medium quality (854x480 30fps)
        "high": "-qh",  # High quality (1080p 60fps)
        "4k": "-qk",  # 4K quality (3840x2160 60fps)
    }

    if quality not in quality_flags:
        raise ValueError(
            f"Invalid quality level: {quality}. "
            f"Must be one of: {', '.join(quality_flags.keys())}"
        )

    # Validate the code first
    is_valid, error = validate_manim_code(code)
    if not is_valid:
        raise ValueError(f"Invalid Manim code: {error}")

    # Extract scene class name
    scene_name = extract_scene_class_name(code)
    if not scene_name:
        raise ValueError("Could not extract Scene class name from code")

    # Create output directory if it doesn't exist
    output_dir = Path(output_path)
    output_dir.mkdir(parents=True, exist_ok=True)

    # Create a temporary directory for the Python file
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)

        # Write code to temporary Python file
        script_file = temp_path / "animation.py"
        with open(script_file, "w") as f:
            f.write(code)

        # Build manim command
        cmd = [
            "manim",
            quality_flags[quality],
            str(script_file),
            scene_name,
            "--output_file",
            scene_name,
            "--media_dir",
            str(output_dir),
        ]

        # Add optional parameters
        if format and format != "mp4":
            cmd.extend(["--format", format])

        if background_color:
            cmd.extend(["--background_color", background_color])

        if transparent:
            cmd.append("--transparent")

        if fps != 60:
            cmd.extend(["--frame_rate", str(fps)])

        # Execute manim command
        try:
            print(f"Rendering animation with command: {' '.join(cmd)}")

            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=300,  # 5 minute timeout
                check=True,
            )

            print("Manim output:", result.stdout)

            # Find the output file
            # Manim creates a structure like: output_dir/videos/animation/quality/SceneName.mp4
            video_dir = output_dir / "videos" / "animation" / quality

            if not video_dir.exists():
                # Try alternate structure: output_dir/videos/quality/
                video_dir = output_dir / "videos" / quality

            # Look for the video file
            possible_extensions = [".mp4", ".mov", ".gif", ".png"]
            output_file = None

            for ext in possible_extensions:
                candidate = video_dir / f"{scene_name}{ext}"
                if candidate.exists():
                    output_file = candidate
                    break

            # Also check in the direct output directory
            if not output_file:
                for ext in possible_extensions:
                    candidate = output_dir / f"{scene_name}{ext}"
                    if candidate.exists():
                        output_file = candidate
                        break

            if not output_file:
                # Search recursively
                for file in output_dir.rglob(f"{scene_name}.*"):
                    if file.suffix in possible_extensions:
                        output_file = file
                        break

            if not output_file or not output_file.exists():
                raise RuntimeError(
                    f"Rendering completed but output file not found. "
                    f"Expected in {video_dir} or {output_dir}"
                )

            return str(output_file.absolute())

        except subprocess.TimeoutExpired:
            raise RuntimeError("Rendering timed out after 5 minutes")

        except subprocess.CalledProcessError as e:
            error_msg = f"Manim rendering failed:\nSTDOUT: {e.stdout}\nSTDERR: {e.stderr}"
            raise RuntimeError(error_msg)

        except Exception as e:
            raise RuntimeError(f"Rendering failed: {str(e)}")


def render_preview(
    code: str, output_path: str, show: bool = True
) -> str:
    """
    Render a quick preview of the animation.

    Args:
        code: Valid Manim Python code
        output_path: Directory for output
        show: Whether to open the preview automatically

    Returns:
        Path to preview video

    Example:
        >>> code = 'from manim import *\\nclass Test(Scene):\\n    def construct(self): self.play(Create(Circle()))'
        >>> preview = render_preview(code, "/tmp/preview")
    """
    # Use low quality for fast preview
    video_path = render_animation(code, output_path, quality="low")

    # Optionally open the video
    if show:
        try:
            if os.name == "posix":  # Linux/Mac
                subprocess.run(["xdg-open", video_path], check=False)
            elif os.name == "nt":  # Windows
                os.startfile(video_path)
        except Exception as e:
            print(f"Could not open preview: {e}")

    return video_path


def render_batch(
    codes: list[str],
    output_path: str,
    quality: QualityLevel = "medium",
    **kwargs,
) -> list[str]:
    """
    Render multiple animations in batch.

    Args:
        codes: List of Manim code strings
        output_path: Base output directory
        quality: Render quality
        **kwargs: Additional arguments passed to render_animation

    Returns:
        List of paths to rendered videos

    Example:
        >>> codes = [code1, code2, code3]
        >>> videos = render_batch(codes, "/tmp/batch")
        >>> len(videos)
        3
    """
    output_dir = Path(output_path)
    results = []

    for i, code in enumerate(codes):
        print(f"Rendering animation {i + 1}/{len(codes)}...")

        # Create subdirectory for each animation
        anim_dir = output_dir / f"animation_{i + 1}"

        try:
            video_path = render_animation(code, str(anim_dir), quality, **kwargs)
            results.append(video_path)
            print(f"  ✓ Rendered: {video_path}")
        except Exception as e:
            print(f"  ✗ Failed: {str(e)}")
            results.append(None)

    return results


def get_render_info(output_path: str) -> dict:
    """
    Get information about rendered animations in a directory.

    Args:
        output_path: Directory to inspect

    Returns:
        Dictionary with render information

    Example:
        >>> info = get_render_info("/tmp/output")
        >>> print(info['video_count'])
        5
    """
    output_dir = Path(output_path)

    if not output_dir.exists():
        return {"exists": False, "video_count": 0, "videos": []}

    # Find all video files
    videos = []
    for ext in [".mp4", ".mov", ".gif"]:
        videos.extend(output_dir.rglob(f"*{ext}"))

    return {
        "exists": True,
        "path": str(output_dir.absolute()),
        "video_count": len(videos),
        "videos": [str(v.absolute()) for v in videos],
        "total_size_mb": sum(v.stat().st_size for v in videos) / (1024 * 1024),
    }


def clean_render_cache(output_path: str) -> None:
    """
    Clean up Manim cache and temporary files.

    Args:
        output_path: Directory to clean

    Example:
        >>> clean_render_cache("/tmp/output")
    """
    output_dir = Path(output_path)

    if not output_dir.exists():
        return

    # Remove common cache directories
    cache_dirs = ["Tex", "Text", "partial_movie_files", "videos"]

    for cache_dir in cache_dirs:
        cache_path = output_dir / cache_dir
        if cache_path.exists():
            try:
                shutil.rmtree(cache_path)
                print(f"Cleaned: {cache_path}")
            except Exception as e:
                print(f"Could not clean {cache_path}: {e}")
