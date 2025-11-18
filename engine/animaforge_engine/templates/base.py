"""Pre-built animation templates for common use cases."""

from typing import Dict

# Template 1: Simple Text Animation
SIMPLE_TEXT_TEMPLATE = """from manim import *

class SimpleTextAnimation(Scene):
    def construct(self):
        # Create and display text
        title = Text("{title}", font_size={font_size})
        title.to_edge(UP)

        # Animate text appearing
        self.play(Write(title), run_time=2)
        self.wait()

        # Create subtitle
        subtitle = Text("{subtitle}", font_size={subtitle_size})
        subtitle.next_to(title, DOWN, buff=0.5)

        # Fade in subtitle
        self.play(FadeIn(subtitle), run_time=1.5)
        self.wait(2)

        # Fade out everything
        self.play(FadeOut(title), FadeOut(subtitle))
        self.wait()
"""

# Template 2: Geometric Shapes Animation
GEOMETRIC_SHAPES_TEMPLATE = """from manim import *

class GeometricShapesAnimation(Scene):
    def construct(self):
        # Create various shapes
        circle = Circle(radius=1, color=BLUE)
        circle.shift(LEFT * 3)

        square = Square(side_length=2, color=RED)
        square.shift(RIGHT * 0)

        triangle = Triangle(color=GREEN)
        triangle.shift(RIGHT * 3)

        # Animate shapes appearing
        self.play(
            Create(circle),
            Create(square),
            Create(triangle),
            run_time=2
        )
        self.wait()

        # Rotate shapes
        self.play(
            Rotate(circle, PI),
            Rotate(square, PI/2),
            Rotate(triangle, PI),
            run_time=2
        )
        self.wait()

        # Scale shapes
        self.play(
            circle.animate.scale(1.5),
            square.animate.scale(0.8),
            triangle.animate.scale(1.2),
            run_time=2
        )
        self.wait()

        # Fade out
        self.play(FadeOut(circle), FadeOut(square), FadeOut(triangle))
        self.wait()
"""

# Template 3: Math Equation Animation
MATH_EQUATION_TEMPLATE = """from manim import *

class MathEquationAnimation(Scene):
    def construct(self):
        # Create title
        title = Text("Mathematical Formula", font_size=48)
        title.to_edge(UP)
        self.play(Write(title))
        self.wait()

        # Create equation
        equation = MathTex(
            r"{equation}",
            font_size=60
        )
        equation.move_to(ORIGIN)

        # Write equation
        self.play(Write(equation), run_time=3)
        self.wait(2)

        # Highlight parts of equation
        self.play(equation.animate.set_color(YELLOW), run_time=1)
        self.wait()

        # Transform to simplified form (if provided)
        simplified = MathTex(r"{simplified}", font_size=60)
        simplified.move_to(ORIGIN)

        self.play(Transform(equation, simplified), run_time=2)
        self.wait(2)

        # Fade out
        self.play(FadeOut(title), FadeOut(equation))
        self.wait()
"""

# Template 4: Graph/Chart Animation
GRAPH_CHART_TEMPLATE = """from manim import *

class GraphChartAnimation(Scene):
    def construct(self):
        # Create axes
        axes = Axes(
            x_range=[-3, 3, 1],
            y_range=[-5, 5, 1],
            x_length=10,
            y_length=6,
            axis_config={{"color": BLUE}},
        )

        # Create labels
        x_label = axes.get_x_axis_label("x")
        y_label = axes.get_y_axis_label("y")

        # Animate axes
        self.play(Create(axes), Write(x_label), Write(y_label))
        self.wait()

        # Create graph
        graph = axes.plot(
            lambda x: {function},
            color=YELLOW,
            x_range=[-3, 3]
        )

        # Label for the graph
        graph_label = MathTex(r"{label}").next_to(graph, UP)

        # Animate graph
        self.play(Create(graph), Write(graph_label), run_time=3)
        self.wait(2)

        # Create a dot that moves along the curve
        dot = Dot(color=RED)
        dot.move_to(axes.c2p(-3, {function_at_start}))

        self.play(FadeIn(dot))
        self.play(
            MoveAlongPath(dot, graph),
            run_time=4,
            rate_func=linear
        )
        self.wait()

        # Fade out
        self.play(
            FadeOut(axes),
            FadeOut(x_label),
            FadeOut(y_label),
            FadeOut(graph),
            FadeOut(graph_label),
            FadeOut(dot)
        )
        self.wait()
"""

# Template 5: Transformation Animation
TRANSFORMATION_TEMPLATE = """from manim import *

class TransformationAnimation(Scene):
    def construct(self):
        # Create starting shape
        start_shape = {start_shape}
        start_shape.set_color({start_color})
        start_shape.scale(1.5)

        # Display start shape
        self.play(Create(start_shape), run_time=2)
        self.wait()

        # Create intermediate shape
        mid_shape = {mid_shape}
        mid_shape.set_color({mid_color})
        mid_shape.scale(1.5)

        # Transform to intermediate
        self.play(Transform(start_shape, mid_shape), run_time=2)
        self.wait()

        # Create end shape
        end_shape = {end_shape}
        end_shape.set_color({end_color})
        end_shape.scale(1.5)

        # Transform to end shape
        self.play(Transform(start_shape, end_shape), run_time=2)
        self.wait()

        # Rotate final shape
        self.play(Rotate(start_shape, PI * 2), run_time=3)
        self.wait()

        # Fade out
        self.play(FadeOut(start_shape))
        self.wait()
"""


def get_template(template_name: str, **kwargs: Dict[str, any]) -> str:
    """
    Get a pre-built animation template with parameter substitution.

    Args:
        template_name: Name of the template to retrieve
        **kwargs: Parameters to substitute in the template

    Returns:
        Formatted template code as a string

    Raises:
        ValueError: If template_name is not recognized
    """
    templates = {
        "simple_text": SIMPLE_TEXT_TEMPLATE,
        "geometric_shapes": GEOMETRIC_SHAPES_TEMPLATE,
        "math_equation": MATH_EQUATION_TEMPLATE,
        "graph_chart": GRAPH_CHART_TEMPLATE,
        "transformation": TRANSFORMATION_TEMPLATE,
    }

    if template_name not in templates:
        raise ValueError(
            f"Unknown template: {template_name}. "
            f"Available templates: {', '.join(templates.keys())}"
        )

    template = templates[template_name]

    # Set default values for common parameters
    defaults = {
        "title": "Welcome to Manim",
        "subtitle": "Creating Beautiful Animations",
        "font_size": 48,
        "subtitle_size": 36,
        "equation": r"E = mc^2",
        "simplified": r"E = mc^2",
        "function": "x**2",
        "label": r"y = x^2",
        "function_at_start": "9",
        "start_shape": "Square()",
        "mid_shape": "Circle()",
        "end_shape": "Triangle()",
        "start_color": "BLUE",
        "mid_color": "GREEN",
        "end_color": "RED",
    }

    # Merge defaults with provided kwargs
    params = {**defaults, **kwargs}

    try:
        return template.format(**params)
    except KeyError as e:
        raise ValueError(f"Missing required parameter for template: {e}")


def list_templates() -> list[str]:
    """
    List all available template names.

    Returns:
        List of template names
    """
    return [
        "simple_text",
        "geometric_shapes",
        "math_equation",
        "graph_chart",
        "transformation",
    ]
