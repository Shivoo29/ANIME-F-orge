"""Validator for Manim animation code."""

import ast
import re
from typing import Optional, Tuple


def validate_manim_code(code: str) -> Tuple[bool, Optional[str]]:
    """
    Validate Manim animation code for syntax and structure.

    Checks for:
    1. Valid Python syntax
    2. Required imports from manim
    3. Scene class definition
    4. construct() method implementation

    Args:
        code: Python code string to validate

    Returns:
        Tuple of (is_valid, error_message)
        - is_valid: True if code is valid, False otherwise
        - error_message: Description of error if invalid, None if valid

    Example:
        >>> code = '''
        ... from manim import *
        ... class MyScene(Scene):
        ...     def construct(self):
        ...         self.play(Create(Circle()))
        ... '''
        >>> is_valid, error = validate_manim_code(code)
        >>> print(is_valid)
        True
    """
    if not code or not code.strip():
        return False, "Code is empty"

    # Step 1: Check for valid Python syntax using AST
    try:
        tree = ast.parse(code)
    except SyntaxError as e:
        return False, f"Syntax error: {e.msg} at line {e.lineno}"
    except Exception as e:
        return False, f"Failed to parse code: {str(e)}"

    # Step 2: Check for manim import
    has_manim_import = False
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom):
            if node.module == "manim":
                has_manim_import = True
                break
        elif isinstance(node, ast.Import):
            for alias in node.names:
                if alias.name == "manim":
                    has_manim_import = True
                    break

    if not has_manim_import:
        return False, "Missing 'from manim import *' or 'import manim' statement"

    # Step 3: Check for Scene class definition
    has_scene_class = False
    has_construct_method = False
    scene_class_name = None

    for node in ast.walk(tree):
        if isinstance(node, ast.ClassDef):
            # Check if class inherits from Scene
            for base in node.bases:
                base_name = None
                if isinstance(base, ast.Name):
                    base_name = base.id
                elif isinstance(base, ast.Attribute):
                    base_name = base.attr

                if base_name == "Scene" or "Scene" in base_name:
                    has_scene_class = True
                    scene_class_name = node.name

                    # Check for construct method in this class
                    for item in node.body:
                        if isinstance(item, ast.FunctionDef) and item.name == "construct":
                            has_construct_method = True
                            break
                    break

    if not has_scene_class:
        return False, "No Scene class found. Code must contain a class that inherits from Scene"

    if not has_construct_method:
        return (
            False,
            f"Scene class '{scene_class_name}' is missing construct() method",
        )

    # Step 4: Additional validation - check for self.play or self.add usage
    code_lower = code.lower()
    has_animation = "self.play" in code or "self.add" in code

    if not has_animation:
        return (
            False,
            "Scene construct() method should contain at least one animation "
            "(self.play() or self.add())",
        )

    # All checks passed
    return True, None


def extract_scene_class_name(code: str) -> Optional[str]:
    """
    Extract the name of the Scene class from Manim code.

    Args:
        code: Python code string

    Returns:
        Name of the Scene class, or None if not found

    Example:
        >>> code = '''
        ... from manim import *
        ... class MyAnimation(Scene):
        ...     def construct(self):
        ...         pass
        ... '''
        >>> extract_scene_class_name(code)
        'MyAnimation'
    """
    try:
        tree = ast.parse(code)
    except:
        return None

    for node in ast.walk(tree):
        if isinstance(node, ast.ClassDef):
            for base in node.bases:
                base_name = None
                if isinstance(base, ast.Name):
                    base_name = base.id
                elif isinstance(base, ast.Attribute):
                    base_name = base.attr

                if base_name == "Scene" or "Scene" in base_name:
                    return node.name

    return None


def clean_code(code: str) -> str:
    """
    Clean code by removing markdown formatting and extra whitespace.

    Args:
        code: Raw code string that may contain markdown

    Returns:
        Cleaned Python code

    Example:
        >>> dirty = '''```python
        ... from manim import *
        ... ```'''
        >>> clean_code(dirty)
        'from manim import *'
    """
    # Remove markdown code blocks
    code = re.sub(r"```python\s*\n", "", code)
    code = re.sub(r"```\s*\n", "", code)
    code = re.sub(r"```", "", code)

    # Remove leading/trailing whitespace
    code = code.strip()

    return code


def get_imports(code: str) -> list[str]:
    """
    Extract all import statements from code.

    Args:
        code: Python code string

    Returns:
        List of import statement strings

    Example:
        >>> code = '''
        ... from manim import *
        ... import numpy as np
        ... '''
        >>> get_imports(code)
        ['from manim import *', 'import numpy as np']
    """
    imports = []

    try:
        tree = ast.parse(code)
    except:
        return imports

    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                if alias.asname:
                    imports.append(f"import {alias.name} as {alias.asname}")
                else:
                    imports.append(f"import {alias.name}")
        elif isinstance(node, ast.ImportFrom):
            module = node.module or ""
            for alias in node.names:
                if alias.name == "*":
                    imports.append(f"from {module} import *")
                elif alias.asname:
                    imports.append(f"from {module} import {alias.name} as {alias.asname}")
                else:
                    imports.append(f"from {module} import {alias.name}")

    return imports


def validate_and_report(code: str) -> dict:
    """
    Validate code and return detailed report.

    Args:
        code: Python code string to validate

    Returns:
        Dictionary containing:
        - valid: boolean
        - error: error message if invalid
        - scene_class: name of Scene class
        - imports: list of import statements
        - warnings: list of warning messages

    Example:
        >>> code = 'from manim import *\\nclass Test(Scene):\\n    def construct(self):\\n        self.play(Create(Circle()))'
        >>> report = validate_and_report(code)
        >>> report['valid']
        True
    """
    report = {
        "valid": False,
        "error": None,
        "scene_class": None,
        "imports": [],
        "warnings": [],
    }

    # Clean the code first
    cleaned_code = clean_code(code)

    # Validate
    is_valid, error = validate_manim_code(cleaned_code)
    report["valid"] = is_valid
    report["error"] = error

    if is_valid:
        report["scene_class"] = extract_scene_class_name(cleaned_code)
        report["imports"] = get_imports(cleaned_code)

        # Add warnings for best practices
        if "self.wait()" not in cleaned_code:
            report["warnings"].append(
                "Consider adding self.wait() calls for better animation timing"
            )

        if len(cleaned_code.split("\n")) < 10:
            report["warnings"].append("Code seems short - consider adding more animations")

    return report
