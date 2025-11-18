-- AnimaForge Sample Data
-- This script inserts demo data for testing and demonstration

-- Insert demo users
-- Password for all demo users: "password123"
-- Bcrypt hash: $2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci

INSERT INTO users (username, email, password_hash, display_name, bio, avatar_url, is_verified) VALUES
('demo_user', 'demo@animaforge.dev', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci', 'Demo User', 'Just testing AnimaForge!', 'https://i.pravatar.cc/150?img=1', true),
('math_teacher', 'math@animaforge.dev', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci', 'Professor Math', 'Creating educational math animations', 'https://i.pravatar.cc/150?img=2', true),
('creative_coder', 'creative@animaforge.dev', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci', 'Creative Coder', 'Exploring generative art with Manim', 'https://i.pravatar.cc/150?img=3', true),
('physics_fan', 'physics@animaforge.dev', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci', 'Physics Enthusiast', 'Visualizing physics concepts', 'https://i.pravatar.cc/150?img=4', true),
('data_viz', 'data@animaforge.dev', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5lY.2E5M8m8Ci', 'Data Visualizer', 'Making data beautiful', 'https://i.pravatar.cc/150?img=5', true)
ON CONFLICT (username) DO NOTHING;

-- Get user IDs for reference
DO $$
DECLARE
    demo_user_id UUID;
    math_teacher_id UUID;
    creative_coder_id UUID;
    physics_fan_id UUID;
    data_viz_id UUID;
BEGIN
    SELECT id INTO demo_user_id FROM users WHERE username = 'demo_user';
    SELECT id INTO math_teacher_id FROM users WHERE username = 'math_teacher';
    SELECT id INTO creative_coder_id FROM users WHERE username = 'creative_coder';
    SELECT id INTO physics_fan_id FROM users WHERE username = 'physics_fan';
    SELECT id INTO data_viz_id FROM users WHERE username = 'data_viz';

    -- Insert sample animations
    INSERT INTO animations (user_id, title, description, file_url, thumbnail_url, source_code, duration, view_count, download_count, like_count, is_public) VALUES
    (
        demo_user_id,
        'Rotating Cube Animation',
        'A simple 3D cube that rotates smoothly with color gradients',
        'https://example.com/animations/rotating-cube.mp4',
        'https://picsum.photos/seed/cube/400/300',
        'from manim import *\n\nclass RotatingCube(Scene):\n    def construct(self):\n        cube = Cube()\n        self.play(Rotate(cube, angle=2*PI))\n        self.wait()',
        5.5,
        1250,
        89,
        45,
        true
    ),
    (
        math_teacher_id,
        'Pythagorean Theorem Proof',
        'Visual proof of the Pythagorean theorem using animated squares',
        'https://example.com/animations/pythagorean.mp4',
        'https://picsum.photos/seed/pyth/400/300',
        'from manim import *\n\nclass PythagoreanTheorem(Scene):\n    def construct(self):\n        # Create right triangle\n        triangle = Polygon([0,0,0], [3,0,0], [3,4,0])\n        self.play(Create(triangle))\n        self.wait()',
        12.0,
        3420,
        234,
        156,
        true
    ),
    (
        math_teacher_id,
        'Fibonacci Spiral',
        'Beautiful visualization of the Fibonacci sequence forming a golden spiral',
        'https://example.com/animations/fibonacci.mp4',
        'https://picsum.photos/seed/fib/400/300',
        'from manim import *\n\nclass FibonacciSpiral(Scene):\n    def construct(self):\n        # Generate Fibonacci squares\n        pass',
        8.2,
        2890,
        178,
        201,
        true
    ),
    (
        creative_coder_id,
        'Particle System Simulation',
        'Mesmerizing particle system with physics-based interactions',
        'https://example.com/animations/particles.mp4',
        'https://picsum.photos/seed/particle/400/300',
        'from manim import *\n\nclass ParticleSystem(Scene):\n    def construct(self):\n        # Create particles\n        pass',
        15.0,
        1678,
        92,
        78,
        true
    ),
    (
        creative_coder_id,
        'Fractal Tree Growth',
        'Watch a fractal tree grow and branch in real-time',
        'https://example.com/animations/fractal-tree.mp4',
        'https://picsum.photos/seed/tree/400/300',
        'from manim import *\n\nclass FractalTree(Scene):\n    def construct(self):\n        # Recursive tree\n        pass',
        10.5,
        2103,
        145,
        189,
        true
    ),
    (
        physics_fan_id,
        'Pendulum Wave Phenomenon',
        'Multiple pendulums create beautiful wave patterns',
        'https://example.com/animations/pendulum.mp4',
        'https://picsum.photos/seed/pendulum/400/300',
        'from manim import *\n\nclass PendulumWave(Scene):\n    def construct(self):\n        # Create pendulums\n        pass',
        20.0,
        4567,
        312,
        298,
        true
    ),
    (
        physics_fan_id,
        'Double Slit Experiment',
        'Quantum mechanics visualization of wave-particle duality',
        'https://example.com/animations/double-slit.mp4',
        'https://picsum.photos/seed/quantum/400/300',
        'from manim import *\n\nclass DoubleSlit(Scene):\n    def construct(self):\n        # Wave interference\n        pass',
        18.3,
        3890,
        267,
        245,
        true
    ),
    (
        data_viz_id,
        'Sorting Algorithms Comparison',
        'Side-by-side comparison of bubble sort, merge sort, and quick sort',
        'https://example.com/animations/sorting.mp4',
        'https://picsum.photos/seed/sort/400/300',
        'from manim import *\n\nclass SortingAlgorithms(Scene):\n    def construct(self):\n        # Sorting visualization\n        pass',
        25.0,
        5234,
        423,
        367,
        true
    ),
    (
        data_viz_id,
        'Stock Market Trends',
        'Animated candlestick chart showing market trends over time',
        'https://example.com/animations/stocks.mp4',
        'https://picsum.photos/seed/stock/400/300',
        'from manim import *\n\nclass StockTrends(Scene):\n    def construct(self):\n        # Chart animation\n        pass',
        14.7,
        2456,
        189,
        134,
        true
    ),
    (
        demo_user_id,
        'Sine Wave Transformation',
        'Morphing between different trigonometric functions',
        'https://example.com/animations/sine-wave.mp4',
        'https://picsum.photos/seed/sine/400/300',
        'from manim import *\n\nclass SineWave(Scene):\n    def construct(self):\n        # Trig functions\n        pass',
        9.0,
        1789,
        98,
        67,
        true
    );

    -- Add tags for animations
    INSERT INTO animation_tags (animation_id, tag)
    SELECT id, unnest(ARRAY['3d', 'geometry', 'beginner']) FROM animations WHERE title = 'Rotating Cube Animation'
    UNION ALL
    SELECT id, unnest(ARRAY['mathematics', 'education', 'geometry']) FROM animations WHERE title = 'Pythagorean Theorem Proof'
    UNION ALL
    SELECT id, unnest(ARRAY['mathematics', 'golden-ratio', 'nature']) FROM animations WHERE title = 'Fibonacci Spiral'
    UNION ALL
    SELECT id, unnest(ARRAY['physics', 'simulation', 'creative']) FROM animations WHERE title = 'Particle System Simulation'
    UNION ALL
    SELECT id, unnest(ARRAY['fractals', 'nature', 'recursive']) FROM animations WHERE title = 'Fractal Tree Growth'
    UNION ALL
    SELECT id, unnest(ARRAY['physics', 'waves', 'demonstration']) FROM animations WHERE title = 'Pendulum Wave Phenomenon'
    UNION ALL
    SELECT id, unnest(ARRAY['physics', 'quantum', 'education']) FROM animations WHERE title = 'Double Slit Experiment'
    UNION ALL
    SELECT id, unnest(ARRAY['computer-science', 'algorithms', 'education']) FROM animations WHERE title = 'Sorting Algorithms Comparison'
    UNION ALL
    SELECT id, unnest(ARRAY['data', 'finance', 'visualization']) FROM animations WHERE title = 'Stock Market Trends'
    UNION ALL
    SELECT id, unnest(ARRAY['mathematics', 'trigonometry', 'waves']) FROM animations WHERE title = 'Sine Wave Transformation'
    ON CONFLICT DO NOTHING;

END $$;

-- Display summary
SELECT
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM animations) as total_animations,
    (SELECT COUNT(DISTINCT tag) FROM animation_tags) as total_tags;
