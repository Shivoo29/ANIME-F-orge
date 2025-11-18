# Contributing to AnimaForge 🤝

First off, thank you for considering contributing to AnimaForge! It's people like you that make AnimaForge such a great tool.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Style Guidelines](#style-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## Code of Conduct

### Our Pledge
We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity, level of experience, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards
**Positive behavior includes:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable behavior includes:**
- Trolling, insulting/derogatory comments, and personal attacks
- Public or private harassment
- Publishing others' private information without permission
- Other conduct which could reasonably be considered inappropriate

---

## Getting Started

### Ways to Contribute
- 🐛 **Report bugs** - Found a bug? Let us know!
- ✨ **Suggest features** - Have an idea? Share it!
- 📝 **Improve documentation** - Help others understand AnimaForge
- 🎨 **Design improvements** - Make AnimaForge more beautiful
- 💻 **Code contributions** - Fix bugs or implement features
- 🎬 **Create templates** - Share your animation templates
- 🌍 **Translations** - Help make AnimaForge multilingual

### What to Work On
- Check out [Good First Issues](https://github.com/yourusername/animaforge/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
- Browse [Help Wanted](https://github.com/yourusername/animaforge/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22)
- Look at the [Project Roadmap](./TODO.md)

---

## Development Setup

### Prerequisites
```bash
# Required
- Rust 1.70+
- Python 3.8+
- Node.js 18+
- PostgreSQL 14+
- Redis 6+

# Recommended
- Docker & Docker Compose
- Make
- Git
```

### Clone Repository
```bash
git clone https://github.com/yourusername/animaforge.git
cd animaforge
```

### Using Docker (Recommended)
```bash
# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Manual Setup

#### 1. Database Setup
```bash
# Start PostgreSQL
brew services start postgresql  # macOS
sudo systemctl start postgresql # Linux

# Create database
createdb animaforge_dev

# Run migrations
cd api
sqlx migrate run
```

#### 2. Backend API
```bash
cd api

# Install dependencies
cargo build

# Setup environment
cp .env.example .env
# Edit .env with your settings

# Run tests
cargo test

# Start server
cargo run
# API will be at http://localhost:8080
```

#### 3. Python Engine
```bash
cd engine

# Create virtual environment
python -m venv venv
source venv/bin/activate  # Linux/macOS
# or
.\venv\Scripts\activate   # Windows

# Install dependencies
pip install -e ".[dev]"

# Run tests
pytest

# The engine is imported by CLI, not run standalone
```

#### 4. CLI
```bash
cd cli

# Build
cargo build

# Run
cargo run -- --help

# Install locally
cargo install --path .
```

#### 5. Frontend
```bash
cd web

# Install dependencies
npm install

# Setup environment
cp .env.example .env.local
# Edit .env.local with your settings

# Run development server
npm run dev
# Web will be at http://localhost:3000

# Run tests
npm test
```

### Verify Setup
```bash
# Test CLI
animaforge --version

# Test API
curl http://localhost:8080/health

# Test Frontend
open http://localhost:3000
```

---

## How to Contribute

### Reporting Bugs

**Before submitting a bug report:**
1. Check existing issues to avoid duplicates
2. Update to the latest version
3. Check if it's actually a bug (not a config issue)

**When reporting a bug, include:**
```markdown
**Describe the bug**
A clear and concise description.

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
 - OS: [e.g. macOS 13.0]
 - AnimaForge Version: [e.g. 1.0.0]
 - Rust/Python/Node Version:
 - LLM Backend: [e.g. Ollama/Gemini]

**Additional context**
Any other relevant information.
```

### Suggesting Features

**Before suggesting a feature:**
1. Check if it's already planned in the [Roadmap](./TODO.md)
2. Search existing feature requests
3. Consider if it aligns with project goals

**When suggesting a feature:**
```markdown
**Problem Statement**
What problem does this solve?

**Proposed Solution**
Describe your solution.

**Alternatives Considered**
What other approaches did you think about?

**Additional Context**
Mockups, examples, etc.

**Would you like to implement this?**
Yes/No - We love community contributions!
```

### Contributing Code

#### 1. Fork & Clone
```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/animaforge.git
cd animaforge
git remote add upstream https://github.com/yourusername/animaforge.git
```

#### 2. Create a Branch
```bash
# Branch naming convention:
# - feature/description
# - fix/description
# - docs/description

git checkout -b feature/add-template-system
```

#### 3. Make Changes
```bash
# Write code
# Add tests
# Update documentation

# Run tests
make test

# Run linters
make lint

# Format code
make format
```

#### 4. Commit Changes
```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: add template system for common animations"

# See commit guidelines below
```

#### 5. Push & Create PR
```bash
# Push to your fork
git push origin feature/add-template-system

# Go to GitHub and create Pull Request
```

---

## Style Guidelines

### Rust Code Style

```rust
// Use rustfmt (runs automatically with `make format`)

// Naming conventions
pub struct AnimationConfig {  // PascalCase for types
    duration: f64,            // snake_case for fields
}

pub fn create_animation() {}  // snake_case for functions

const MAX_DURATION: f64 = 60.0;  // SCREAMING_SNAKE for constants

// Documentation
/// Creates a new animation from prompt.
///
/// # Arguments
/// * `prompt` - Natural language description
/// * `config` - Animation configuration
///
/// # Returns
/// Generated animation or error
///
/// # Examples
/// ```
/// let anim = create_animation("rotating cube", config)?;
/// ```
pub fn create_animation(prompt: &str, config: &Config) -> Result<Animation> {
    // Implementation
}

// Error handling - always use Result/Option
pub fn might_fail() -> Result<String, Error> {
    // Don't panic! Return errors
}

// Prefer iterators over loops
let squares: Vec<i32> = numbers.iter()
    .map(|x| x * x)
    .collect();
```

### Python Code Style

```python
# Use Black for formatting (runs with `make format`)
# Follow PEP 8

# Naming conventions
class AnimationGenerator:  # PascalCase for classes
    pass

def create_animation():  # snake_case for functions
    pass

FRAME_RATE = 60  # SCREAMING_SNAKE for constants

# Type hints always
def generate_code(prompt: str, config: Config) -> str:
    """Generate Manim code from prompt.
    
    Args:
        prompt: Natural language description
        config: Generation configuration
    
    Returns:
        Generated Manim code
    
    Raises:
        ValidationError: If prompt is invalid
    
    Example:
        >>> code = generate_code("rotating cube", config)
        >>> print(code)
    """
    pass

# Use pathlib, not os.path
from pathlib import Path
output_dir = Path("animations")
output_file = output_dir / "my_anim.mp4"

# Context managers for resources
with open(file_path) as f:
    content = f.read()
```

### TypeScript Code Style

```typescript
// Use ESLint + Prettier (runs with `make format`)

// Naming conventions
interface AnimationData {}  // PascalCase for interfaces
type AnimationType = "math" | "physics";  // PascalCase for types

const createAnimation = () => {};  // camelCase for functions
const MAX_DURATION = 60;  // SCREAMING_SNAKE for constants

// Always use const/let, never var
const name = "AnimaForge";
let count = 0;

// Type everything
function generateAnimation(
  prompt: string,
  config: AnimationConfig
): Promise<Animation> {
  // Implementation
}

// Prefer functional components (React)
export const AnimationCard: React.FC<Props> = ({ animation }) => {
  const [isPlaying, setIsPlaying] = useState(false);
  
  return (
    <div className="animation-card">
      {/* JSX */}
    </div>
  );
};

// Use async/await, not .then()
const fetchAnimation = async (id: string) => {
  try {
    const response = await api.get(`/animations/${id}`);
    return response.data;
  } catch (error) {
    console.error("Failed to fetch:", error);
    throw error;
  }
};
```

### CSS/Tailwind Style

```typescript
// Prefer Tailwind utilities over custom CSS

// ❌ Don't
<div style={{ padding: "20px", backgroundColor: "#fff" }}>

// ✅ Do
<div className="p-5 bg-white">

// For neo-brutalism, use consistent patterns
<div className="border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]">

// Custom colors in tailwind.config.ts
export default {
  theme: {
    extend: {
      colors: {
        'brand-orange': '#FF6B35',
        'brand-blue': '#004E89',
      }
    }
  }
}
```

---

## Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation only
- **style**: Code style (formatting, no logic change)
- **refactor**: Code refactoring
- **perf**: Performance improvement
- **test**: Adding tests
- **chore**: Maintenance tasks
- **ci**: CI/CD changes

### Examples

```bash
# Simple feature
git commit -m "feat(cli): add batch processing support"

# Bug fix with details
git commit -m "fix(api): resolve race condition in upload handler

The upload handler was not properly locking resources,
causing occasional corruption when multiple uploads happened
simultaneously.

Fixes #123"

# Breaking change
git commit -m "feat(engine)!: change animation generation API

BREAKING CHANGE: The generate() function now returns
Result<Animation> instead of Option<Animation>"

# Multiple changes
git commit -m "feat(web): add dark mode support

- Add theme toggle component
- Update all pages for dark mode
- Persist preference in localStorage

Closes #456"
```

### Subject Line Rules
- Use imperative mood ("add" not "adds" or "added")
- Don't capitalize first letter
- No period at the end
- Keep under 72 characters

---

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code follows style guidelines
- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] No merge conflicts
- [ ] Commit messages follow guidelines
- [ ] PR description is complete

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (fix or feature that breaks existing functionality)
- [ ] Documentation update

## How Has This Been Tested?
Describe tests you ran

## Screenshots (if applicable)
Add screenshots of UI changes

## Checklist
- [ ] My code follows the style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes

## Related Issues
Closes #123
Related to #456
```

### Review Process

1. **Automated Checks** - CI must pass
2. **Code Review** - 1+ maintainer approval required
3. **Testing** - Reviewer tests changes
4. **Merge** - Squash and merge by maintainer

### After Merge

```bash
# Sync your fork
git checkout main
git pull upstream main
git push origin main

# Delete feature branch
git branch -d feature/your-feature
git push origin --delete feature/your-feature
```

---

## Testing Guidelines

### Unit Tests

```rust
// Rust - use #[test]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let config = Config::default();
        let result = create_animation("test", &config);
        assert!(result.is_ok());
    }
}
```

```python
# Python - use pytest
def test_code_generation():
    generator = CodeGenerator()
    code = generator.generate("rotating cube")
    assert "class" in code
    assert "def construct" in code
```

```typescript
// TypeScript - use Jest
describe('AnimationCard', () => {
  it('renders animation title', () => {
    render(<AnimationCard animation={mockAnimation} />);
    expect(screen.getByText('Test Animation')).toBeInTheDocument();
  });
});
```

### Integration Tests

```bash
# Test full CLI workflow
./tests/integration/test_cli_workflow.sh

# Test API endpoints
./tests/integration/test_api_endpoints.sh
```

### Coverage Goals
- Unit tests: >80% coverage
- Integration tests: Critical paths covered
- E2E tests: User workflows covered

---

## Community

### Get Help
- 💬 [Discord Community](https://discord.gg/animaforge)
- 🐛 [GitHub Issues](https://github.com/yourusername/animaforge/issues)
- 📖 [Documentation](https://docs.animaforge.dev)
- 📧 [Email Support](mailto:support@animaforge.dev)

### Stay Updated
- ⭐ Star the repo on GitHub
- 👀 Watch for releases
- 🐦 Follow [@animaforge](https://twitter.com/animaforge)
- 📰 Subscribe to [newsletter](https://animaforge.dev/newsletter)

### Recognition
Contributors are recognized in:
- [Contributors page](https://github.com/yourusername/animaforge/graphs/contributors)
- Monthly newsletter
- Release notes
- Project README

---

## License

By contributing to AnimaForge, you agree that your contributions will be licensed under the MIT License.

---

## Questions?

Don't hesitate to ask! We're here to help:
- Open a [Discussion](https://github.com/yourusername/animaforge/discussions)
- Join our [Discord](https://discord.gg/animaforge)
- Email us at dev@animaforge.dev

---

**Thank you for contributing to AnimaForge! 🎉**

Together, we're democratizing animation creation for everyone.
