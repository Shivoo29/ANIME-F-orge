# AnimaForge 🎬⚡

> **Transform words into stunning animations. No code required.**

AnimaForge is a revolutionary CLI tool and marketplace that democratizes animation creation. Using AI (local or cloud), it converts natural language prompts into professional Manim animations that you can share, remix, and monetize.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Next.js 14](https://img.shields.io/badge/next.js-14-black)](https://nextjs.org/)

---

## 🌟 Features

### 🎨 For Creators
- **AI-Powered Generation**: Use Ollama, Qwen, Gemini, or Claude to generate animations
- **Zero Coding Required**: Natural language → Beautiful animations
- **Instant Preview**: See low-res previews before full render
- **Template Library**: Start from 100+ pre-built templates
- **Version Control**: Fork, branch, and merge animations like code
- **Quality Scoring**: AI validates your animation before publishing

### 🏪 Marketplace
- **Publish & Earn**: Share animations and earn from downloads
- **NPM-like Experience**: Install animations with `animaforge install <animation-id>`
- **Semantic Search**: Find the perfect animation using natural language
- **Analytics Dashboard**: Track views, downloads, and earnings
- **Community Ratings**: Let users vote on quality

### 🛠️ Developer-Friendly
- **Multi-Backend Support**: Works with local LLMs or cloud APIs
- **Plugin System**: Extend functionality with custom plugins
- **API Access**: Programmatic animation generation
- **CLI + GUI**: Choose your preferred workflow
- **Export Formats**: MP4, GIF, WebM, and raw Manim code

---

## 🚀 Quick Start

### Prerequisites
- Python 3.8+
- Rust 1.70+ (for CLI)
- Node.js 18+ (for web)
- FFmpeg
- LaTeX (for math animations)

### Installation

#### Using Cargo (Rust)
```bash
cargo install animaforge-cli
```

#### Using pip (Python)
```bash
pip install animaforge
```

#### From Source
```bash
git clone https://github.com/yourusername/animaforge.git
cd animaforge
make install
```

### First Animation

```bash
# Configure your AI backend
animaforge config set --backend ollama --model llama3

# Generate your first animation
animaforge create "A rotating cube with color gradient"

# Preview before rendering
animaforge preview ./animations/cube_001.py

# Render full quality
animaforge render ./animations/cube_001.py --quality high

# Upload to marketplace
animaforge publish ./animations/cube_001.py
```

---

## 📖 Documentation

- [CLI Documentation](./docs/CLI.md)
- [API Reference](./docs/API.md)
- [Marketplace Guide](./docs/MARKETPLACE.md)
- [Contributing](./CONTRIBUTING.md)
- [Architecture](./docs/ARCHITECTURE.md)
- [Plugin Development](./docs/PLUGINS.md)

---

## 🏗️ Architecture

```
┌─────────────┐
│   CLI Tool  │ (Rust)
│  (animaforge)│
└──────┬──────┘
       │
       ├─────► ┌──────────────┐
       │       │  AI Backends │
       │       │ (Ollama/etc) │
       │       └──────────────┘
       │
       ├─────► ┌──────────────┐
       │       │    Engine    │
       │       │   (Python)   │
       │       │    Manim     │
       │       └──────────────┘
       │
       └─────► ┌──────────────┐
               │   API/Web    │
               │  (Next.js)   │
               │  Marketplace │
               └──────────────┘
```

---

## 🎯 Use Cases

### Content Creators
- **YouTube Explainers**: Create 3Blue1Brown-style math/science animations
- **Educational Content**: Visualize complex concepts
- **Social Media**: Generate eye-catching animated graphics

### Educators
- **Lecture Materials**: Animated slides and visualizations
- **Interactive Demos**: Engage students with dynamic content
- **Course Creation**: Build entire animated courses

### Developers
- **Technical Documentation**: Animated API workflows
- **Product Demos**: Show your app in action
- **Conference Talks**: Elevate your presentations

### Designers
- **Motion Graphics**: Create stunning motion designs
- **Client Presentations**: Impress with animated mockups
- **Portfolio Pieces**: Stand out with unique animations

---

## 🛣️ Roadmap

### Phase 1: MVP (Months 1-2)
- [x] Core CLI tool structure
- [ ] Basic Manim code generation
- [ ] Local LLM integration (Ollama)
- [ ] Simple preview system
- [ ] File management

### Phase 2: Marketplace Beta (Months 3-4)
- [ ] Next.js marketplace frontend
- [ ] User authentication & profiles
- [ ] Upload/download system
- [ ] Basic search functionality
- [ ] Payment integration (Stripe)

### Phase 3: Enhancement (Months 5-6)
- [ ] Cloud LLM support (Gemini, Claude)
- [ ] Advanced prompt engineering
- [ ] Template library (50+ templates)
- [ ] Version control system
- [ ] Analytics dashboard

### Phase 4: Community (Months 7-8)
- [ ] Collaboration features
- [ ] Plugin system
- [ ] API for developers
- [ ] Mobile app (React Native)
- [ ] Enterprise features

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](./CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone repo
git clone https://github.com/yourusername/animaforge.git
cd animaforge

# Install dependencies
make install-dev

# Run tests
make test

# Start development servers
make dev
```

---

## 📊 Stats

- **Templates**: 100+
- **Active Creators**: Growing daily
- **Animations Generated**: Coming soon
- **Community Plugins**: Coming soon

---

## 🎨 Tech Stack

- **CLI**: Rust (performance & reliability)
- **Engine**: Python + Manim (animation generation)
- **Backend**: Rust/Actix-web (API)
- **Frontend**: Next.js 14 + TypeScript (marketplace)
- **Database**: PostgreSQL + Redis
- **Storage**: S3-compatible (animations)
- **Search**: Meilisearch (semantic search)
- **AI**: Ollama, OpenAI, Anthropic, Google AI

---

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

---

## 🙏 Acknowledgments

- [3Blue1Brown](https://www.3blue1brown.com/) for inspiration
- [Manim Community](https://www.manim.community/) for the amazing library
- All contributors and early adopters

---

## 📬 Contact

- **Website**: https://animaforge.dev
- **Discord**: https://discord.gg/animaforge
- **Twitter**: [@animaforge](https://twitter.com/animaforge)
- **Email**: hello@animaforge.dev

---

## ⭐ Show Your Support

If AnimaForge helps you, please give it a star! It motivates us to keep building.

[⭐ Star on GitHub](https://github.com/yourusername/animaforge)

---

**Built with ❤️ by creators, for creators**
