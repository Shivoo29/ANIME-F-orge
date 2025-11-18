"""Setup configuration for animaforge-engine package."""

from setuptools import setup, find_packages

setup(
    name="animaforge-engine",
    version="0.1.0",
    description="AI-powered animation engine with Manim integration",
    long_description=open("README.md").read() if open("README.md") else "",
    long_description_content_type="text/markdown",
    author="AnimaForge Team",
    author_email="team@animaforge.dev",
    url="https://github.com/animaforge/engine",
    packages=find_packages(),
    python_requires=">=3.9",
    install_requires=[
        "manim>=0.18.0",
        "requests>=2.31.0",
        "python-dotenv>=1.0.0",
        "pydantic>=2.5.0",
        "anthropic>=0.18.0",
        "google-generativeai>=0.3.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.4.0",
            "black>=23.12.0",
            "mypy>=1.7.0",
            "ruff>=0.1.0",
        ],
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
    ],
    keywords="animation manim ai video-generation",
)
