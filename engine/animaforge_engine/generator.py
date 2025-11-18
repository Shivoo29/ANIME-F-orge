"""Code generator for creating Manim animations from text prompts."""

import os
import re
import json
from typing import Optional, Literal
from dotenv import load_dotenv

from .prompts import (
    MANIM_CODE_GENERATION_PROMPT,
    MANIM_CODE_REFINEMENT_PROMPT,
    CODE_EXTRACTION_PROMPT,
)
from .validator import validate_manim_code, clean_code

# Load environment variables
load_dotenv()

LLMBackend = Literal["ollama", "gemini", "claude"]


def generate_animation_code(
    prompt: str,
    llm_backend: LLMBackend = "ollama",
    max_retries: int = 3,
    model: Optional[str] = None,
) -> str:
    """
    Generate Manim animation code from a text prompt using an LLM.

    Args:
        prompt: Text description of the desired animation
        llm_backend: LLM service to use ('ollama', 'gemini', or 'claude')
        max_retries: Maximum number of retry attempts if generation fails
        model: Specific model to use (optional, uses defaults if not specified)

    Returns:
        Valid Manim Python code as a string

    Raises:
        ValueError: If llm_backend is not supported
        RuntimeError: If code generation fails after max_retries
        EnvironmentError: If required API keys are missing

    Example:
        >>> code = generate_animation_code(
        ...     "Create a blue circle that transforms into a red square",
        ...     llm_backend="ollama"
        ... )
        >>> print(code[:20])
        from manim import *
    """
    if llm_backend not in ["ollama", "gemini", "claude"]:
        raise ValueError(f"Unsupported LLM backend: {llm_backend}")

    system_prompt = MANIM_CODE_GENERATION_PROMPT
    user_prompt = f"Animation request: {prompt}"

    for attempt in range(max_retries):
        try:
            # Generate code using selected backend
            if llm_backend == "ollama":
                raw_code = _generate_with_ollama(system_prompt, user_prompt, model)
            elif llm_backend == "gemini":
                raw_code = _generate_with_gemini(system_prompt, user_prompt, model)
            elif llm_backend == "claude":
                raw_code = _generate_with_claude(system_prompt, user_prompt, model)

            # Clean and extract code
            code = clean_code(raw_code)
            code = _extract_python_code(code)

            # Validate the generated code
            is_valid, error = validate_manim_code(code)

            if is_valid:
                return code

            # If invalid, try to refine it
            if attempt < max_retries - 1:
                print(f"Attempt {attempt + 1} failed validation: {error}")
                print("Attempting to refine code...")
                code = _refine_code(code, error, llm_backend, model)

                # Validate refined code
                is_valid, error = validate_manim_code(code)
                if is_valid:
                    return code

        except Exception as e:
            if attempt == max_retries - 1:
                raise RuntimeError(
                    f"Failed to generate valid code after {max_retries} attempts: {str(e)}"
                )
            print(f"Attempt {attempt + 1} error: {str(e)}")
            continue

    raise RuntimeError(f"Failed to generate valid code after {max_retries} attempts")


def _generate_with_ollama(
    system_prompt: str, user_prompt: str, model: Optional[str] = None
) -> str:
    """Generate code using Ollama local LLM."""
    import requests

    # Default to codellama or llama2 if no model specified
    model_name = model or os.getenv("OLLAMA_MODEL", "codellama")
    ollama_url = os.getenv("OLLAMA_URL", "http://localhost:11434")

    endpoint = f"{ollama_url}/api/generate"

    full_prompt = f"{system_prompt}\n\n{user_prompt}"

    payload = {
        "model": model_name,
        "prompt": full_prompt,
        "stream": False,
        "options": {
            "temperature": 0.7,
            "top_p": 0.9,
        },
    }

    try:
        response = requests.post(endpoint, json=payload, timeout=120)
        response.raise_for_status()
        result = response.json()
        return result.get("response", "")
    except requests.exceptions.ConnectionError:
        raise RuntimeError(
            f"Cannot connect to Ollama at {ollama_url}. "
            "Make sure Ollama is running (ollama serve)"
        )
    except Exception as e:
        raise RuntimeError(f"Ollama generation failed: {str(e)}")


def _generate_with_gemini(
    system_prompt: str, user_prompt: str, model: Optional[str] = None
) -> str:
    """Generate code using Google Gemini API."""
    import google.generativeai as genai

    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key:
        raise EnvironmentError(
            "GEMINI_API_KEY not found in environment variables. "
            "Please set it in .env file or environment."
        )

    genai.configure(api_key=api_key)

    model_name = model or os.getenv("GEMINI_MODEL", "gemini-pro")

    try:
        model_instance = genai.GenerativeModel(model_name)

        full_prompt = f"{system_prompt}\n\n{user_prompt}"

        response = model_instance.generate_content(
            full_prompt,
            generation_config={
                "temperature": 0.7,
                "top_p": 0.9,
                "max_output_tokens": 2048,
            },
        )

        return response.text
    except Exception as e:
        raise RuntimeError(f"Gemini generation failed: {str(e)}")


def _generate_with_claude(
    system_prompt: str, user_prompt: str, model: Optional[str] = None
) -> str:
    """Generate code using Anthropic Claude API."""
    import anthropic

    api_key = os.getenv("ANTHROPIC_API_KEY")
    if not api_key:
        raise EnvironmentError(
            "ANTHROPIC_API_KEY not found in environment variables. "
            "Please set it in .env file or environment."
        )

    client = anthropic.Anthropic(api_key=api_key)

    model_name = model or os.getenv("CLAUDE_MODEL", "claude-3-5-sonnet-20241022")

    try:
        message = client.messages.create(
            model=model_name,
            max_tokens=2048,
            temperature=0.7,
            system=system_prompt,
            messages=[{"role": "user", "content": user_prompt}],
        )

        return message.content[0].text
    except Exception as e:
        raise RuntimeError(f"Claude generation failed: {str(e)}")


def _extract_python_code(text: str) -> str:
    """
    Extract Python code from text that may contain markdown or explanations.

    Args:
        text: Raw text possibly containing code

    Returns:
        Extracted Python code
    """
    # First try to find code blocks
    code_block_pattern = r"```(?:python)?\s*\n(.*?)```"
    matches = re.findall(code_block_pattern, text, re.DOTALL)

    if matches:
        # Return the first (or largest) code block
        return max(matches, key=len).strip()

    # If no code blocks, look for lines that look like Python
    lines = text.split("\n")
    code_lines = []
    in_code = False

    for line in lines:
        stripped = line.strip()

        # Start of Python code
        if stripped.startswith(("from ", "import ", "class ", "def ", "#")):
            in_code = True

        if in_code:
            code_lines.append(line)

        # Stop if we hit explanatory text after code
        if (
            in_code
            and stripped
            and not stripped.startswith(("#", "from ", "import ", "class ", "def ", " ", "\t"))
            and ":" not in stripped
        ):
            # Might be explanation, but check if it's part of a string
            if not any(
                quote in stripped for quote in ['"', "'"]
            ) and not stripped.startswith(("self.", "return", "if ", "else", "for ", "while ")):
                break

    if code_lines:
        return "\n".join(code_lines).strip()

    # Last resort: return the whole text
    return text.strip()


def _refine_code(
    code: str, error: str, llm_backend: LLMBackend, model: Optional[str] = None
) -> str:
    """
    Attempt to fix invalid code using LLM.

    Args:
        code: Invalid code to fix
        error: Error message from validation
        llm_backend: LLM service to use
        model: Specific model to use

    Returns:
        Refined code
    """
    refinement_prompt = MANIM_CODE_REFINEMENT_PROMPT.format(error=error, code=code)

    try:
        if llm_backend == "ollama":
            refined = _generate_with_ollama("You are a Python code fixer.", refinement_prompt, model)
        elif llm_backend == "gemini":
            refined = _generate_with_gemini("You are a Python code fixer.", refinement_prompt, model)
        elif llm_backend == "claude":
            refined = _generate_with_claude("You are a Python code fixer.", refinement_prompt, model)

        return clean_code(_extract_python_code(refined))
    except Exception as e:
        print(f"Code refinement failed: {str(e)}")
        return code


def batch_generate(
    prompts: list[str], llm_backend: LLMBackend = "ollama", **kwargs
) -> list[str]:
    """
    Generate multiple animations from a list of prompts.

    Args:
        prompts: List of animation prompts
        llm_backend: LLM service to use
        **kwargs: Additional arguments passed to generate_animation_code

    Returns:
        List of generated code strings

    Example:
        >>> prompts = [
        ...     "Create a spinning circle",
        ...     "Show text that fades in and out"
        ... ]
        >>> codes = batch_generate(prompts, llm_backend="ollama")
        >>> len(codes)
        2
    """
    results = []

    for i, prompt in enumerate(prompts):
        print(f"Generating animation {i + 1}/{len(prompts)}: {prompt[:50]}...")
        try:
            code = generate_animation_code(prompt, llm_backend, **kwargs)
            results.append(code)
        except Exception as e:
            print(f"Failed to generate code for prompt {i + 1}: {str(e)}")
            results.append(None)

    return results
