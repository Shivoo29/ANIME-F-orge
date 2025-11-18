"""Tests for the validator module."""

import pytest
from animaforge_engine.validator import (
    validate_manim_code,
    extract_scene_class_name,
    clean_code,
    get_imports,
    validate_and_report,
)


class TestValidateManimCode:
    """Test the validate_manim_code function."""

    def test_valid_simple_code(self):
        """Test validation of valid simple Manim code."""
        code = """
from manim import *

class TestScene(Scene):
    def construct(self):
        circle = Circle()
        self.play(Create(circle))
        self.wait()
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is True
        assert error is None

    def test_missing_import(self):
        """Test validation fails when manim import is missing."""
        code = """
class TestScene(Scene):
    def construct(self):
        self.play(Create(Circle()))
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is False
        assert "import" in error.lower()

    def test_missing_scene_class(self):
        """Test validation fails when Scene class is missing."""
        code = """
from manim import *

def some_function():
    pass
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is False
        assert "scene" in error.lower()

    def test_missing_construct_method(self):
        """Test validation fails when construct method is missing."""
        code = """
from manim import *

class TestScene(Scene):
    def some_other_method(self):
        pass
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is False
        assert "construct" in error.lower()

    def test_syntax_error(self):
        """Test validation fails on syntax errors."""
        code = """
from manim import *

class TestScene(Scene):
    def construct(self)
        self.play(Create(Circle()))
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is False
        assert "syntax" in error.lower()

    def test_empty_code(self):
        """Test validation fails on empty code."""
        is_valid, error = validate_manim_code("")
        assert is_valid is False
        assert "empty" in error.lower()

    def test_no_animations(self):
        """Test validation warns when no animations present."""
        code = """
from manim import *

class TestScene(Scene):
    def construct(self):
        pass
"""
        is_valid, error = validate_manim_code(code)
        assert is_valid is False
        assert "animation" in error.lower()


class TestExtractSceneClassName:
    """Test the extract_scene_class_name function."""

    def test_extract_simple_class_name(self):
        """Test extraction of simple scene class name."""
        code = """
from manim import *

class MyAnimation(Scene):
    def construct(self):
        pass
"""
        name = extract_scene_class_name(code)
        assert name == "MyAnimation"

    def test_extract_none_when_no_scene(self):
        """Test returns None when no Scene class found."""
        code = """
from manim import *

class NotAScene:
    pass
"""
        name = extract_scene_class_name(code)
        assert name is None

    def test_extract_with_invalid_syntax(self):
        """Test returns None with invalid syntax."""
        code = "this is not valid python"
        name = extract_scene_class_name(code)
        assert name is None


class TestCleanCode:
    """Test the clean_code function."""

    def test_remove_python_markdown(self):
        """Test removal of python markdown code blocks."""
        dirty = """```python
from manim import *
```"""
        clean = clean_code(dirty)
        assert clean == "from manim import *"

    def test_remove_generic_markdown(self):
        """Test removal of generic markdown code blocks."""
        dirty = """```
from manim import *
```"""
        clean = clean_code(dirty)
        assert clean == "from manim import *"

    def test_strip_whitespace(self):
        """Test stripping of leading/trailing whitespace."""
        dirty = "\n\n  from manim import *  \n\n"
        clean = clean_code(dirty)
        assert clean == "from manim import *"


class TestGetImports:
    """Test the get_imports function."""

    def test_extract_simple_import(self):
        """Test extraction of simple import."""
        code = "import numpy"
        imports = get_imports(code)
        assert "import numpy" in imports

    def test_extract_from_import(self):
        """Test extraction of from import."""
        code = "from manim import *"
        imports = get_imports(code)
        assert "from manim import *" in imports

    def test_extract_import_as(self):
        """Test extraction of import with alias."""
        code = "import numpy as np"
        imports = get_imports(code)
        assert "import numpy as np" in imports

    def test_extract_multiple_imports(self):
        """Test extraction of multiple imports."""
        code = """
from manim import *
import numpy as np
import math
"""
        imports = get_imports(code)
        assert len(imports) == 3

    def test_extract_from_invalid_code(self):
        """Test extraction from invalid code returns empty list."""
        code = "this is not valid python"
        imports = get_imports(code)
        assert imports == []


class TestValidateAndReport:
    """Test the validate_and_report function."""

    def test_report_on_valid_code(self):
        """Test detailed report on valid code."""
        code = """
from manim import *

class TestScene(Scene):
    def construct(self):
        self.play(Create(Circle()))
        self.wait()
"""
        report = validate_and_report(code)

        assert report['valid'] is True
        assert report['error'] is None
        assert report['scene_class'] == 'TestScene'
        assert len(report['imports']) > 0
        assert isinstance(report['warnings'], list)

    def test_report_on_invalid_code(self):
        """Test detailed report on invalid code."""
        code = "invalid python code"

        report = validate_and_report(code)

        assert report['valid'] is False
        assert report['error'] is not None
        assert report['scene_class'] is None

    def test_report_warnings(self):
        """Test that report includes warnings."""
        code = """
from manim import *

class TestScene(Scene):
    def construct(self):
        self.play(Create(Circle()))
"""
        report = validate_and_report(code)

        # Should warn about missing self.wait()
        assert len(report['warnings']) > 0
