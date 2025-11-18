# AnimaForge Project Structure 📁

## Complete Directory Layout

```
animaforge/
│
├── 📄 README.md                    # Main project README
├── 📄 LICENSE                      # MIT License
├── 📄 CONTRIBUTING.md              # Contributing guidelines
├── 📄 TODO.md                      # Development roadmap
├── 📄 .gitignore                   # Git ignore rules
├── 📄 Makefile                     # Build automation
├── 📄 docker-compose.yml           # Docker development environment
├── 📄 .env.example                 # Environment variables template
│
├── 📁 cli/                         # Rust CLI Tool
│   ├── 📄 Cargo.toml              # Rust dependencies
│   ├── 📄 Cargo.lock
│   ├── 📄 README.md               # CLI documentation
│   ├── 📄 .env.example
│   ├── 📁 src/
│   │   ├── main.rs                # Entry point
│   │   ├── cli.rs                 # CLI definitions
│   │   ├── 📁 commands/           # Command implementations
│   │   │   ├── create.rs          # `animaforge create`
│   │   │   ├── render.rs          # `animaforge render`
│   │   │   ├── publish.rs         # `animaforge publish`
│   │   │   ├── search.rs          # `animaforge search`
│   │   │   ├── install.rs         # `animaforge install`
│   │   │   ├── config.rs          # `animaforge config`
│   │   │   ├── auth.rs            # `animaforge auth`
│   │   │   └── template.rs        # `animaforge template`
│   │   ├── 📁 config/
│   │   │   ├── mod.rs             # Config management
│   │   │   └── providers.rs      # LLM provider configs
│   │   ├── 📁 llm/                # LLM integrations
│   │   │   ├── mod.rs             # LLM trait
│   │   │   ├── ollama.rs          # Ollama integration
│   │   │   ├── gemini.rs          # Gemini integration
│   │   │   ├── claude.rs          # Claude integration
│   │   │   └── openai.rs          # OpenAI integration
│   │   ├── 📁 engine/
│   │   │   ├── mod.rs             # Engine coordinator
│   │   │   └── manim.rs           # Manim wrapper
│   │   ├── 📁 api/
│   │   │   ├── mod.rs             # API client
│   │   │   └── client.rs          # HTTP client
│   │   └── 📁 utils/
│   │       ├── mod.rs
│   │       ├── progress.rs        # Progress bars
│   │       ├── errors.rs          # Error handling
│   │       └── logger.rs          # Logging setup
│   ├── 📁 tests/
│   │   ├── integration.rs
│   │   └── fixtures/
│   └── 📁 benches/
│       └── generation.rs
│
├── 📁 engine/                      # Python Animation Engine
│   ├── 📄 setup.py                # Package setup
│   ├── 📄 pyproject.toml          # Build config
│   ├── 📄 requirements.txt        # Python dependencies
│   ├── 📄 requirements-dev.txt    # Dev dependencies
│   ├── 📄 README.md
│   ├── 📁 animaforge_engine/
│   │   ├── __init__.py
│   │   ├── generator.py           # Code generation
│   │   ├── validator.py           # Code validation
│   │   ├── renderer.py            # Manim rendering
│   │   ├── optimizer.py           # Code optimization
│   │   ├── quality_scorer.py      # Quality assessment
│   │   ├── 📁 templates/          # Animation templates
│   │   │   ├── __init__.py
│   │   │   ├── base.py            # Base templates
│   │   │   ├── math.py            # Math templates
│   │   │   ├── physics.py         # Physics templates
│   │   │   ├── cs.py              # CS templates
│   │   │   └── business.py        # Business templates
│   │   ├── 📁 prompts/            # Prompt templates
│   │   │   ├── __init__.py
│   │   │   ├── system.py          # System prompts
│   │   │   └── examples.py        # Few-shot examples
│   │   └── 📁 utils/
│   │       ├── __init__.py
│   │       ├── code_parser.py     # AST parsing
│   │       ├── manim_wrapper.py   # Manim helpers
│   │       └── ffmpeg_utils.py    # Video encoding
│   ├── 📁 tests/
│   │   ├── __init__.py
│   │   ├── test_generator.py
│   │   ├── test_validator.py
│   │   ├── test_renderer.py
│   │   └── fixtures/
│   └── 📁 docs/
│       ├── conf.py
│       └── index.rst
│
├── 📁 api/                         # Rust Backend API
│   ├── 📄 Cargo.toml
│   ├── 📄 Cargo.lock
│   ├── 📄 README.md
│   ├── 📄 .env.example
│   ├── 📁 src/
│   │   ├── main.rs
│   │   ├── 📁 routes/             # API endpoints
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs            # /api/v1/auth/*
│   │   │   ├── animations.rs      # /api/v1/animations/*
│   │   │   ├── users.rs           # /api/v1/users/*
│   │   │   ├── marketplace.rs     # /api/v1/marketplace/*
│   │   │   ├── search.rs          # /api/v1/search/*
│   │   │   └── webhooks.rs        # /api/v1/webhooks/*
│   │   ├── 📁 models/             # Data models
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── animation.rs
│   │   │   ├── transaction.rs
│   │   │   └── analytics.rs
│   │   ├── 📁 db/                 # Database
│   │   │   ├── mod.rs
│   │   │   ├── pool.rs            # Connection pooling
│   │   │   └── 📁 migrations/     # SQL migrations
│   │   │       ├── 001_users.sql
│   │   │       ├── 002_animations.sql
│   │   │       └── ...
│   │   ├── 📁 middleware/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs            # JWT validation
│   │   │   ├── rate_limit.rs      # Rate limiting
│   │   │   ├── cors.rs            # CORS handling
│   │   │   └── logging.rs         # Request logging
│   │   ├── 📁 services/           # Business logic
│   │   │   ├── mod.rs
│   │   │   ├── storage.rs         # S3 operations
│   │   │   ├── search.rs          # Meilisearch
│   │   │   ├── payment.rs         # Stripe integration
│   │   │   ├── queue.rs           # Redis queue
│   │   │   └── analytics.rs       # Analytics tracking
│   │   └── 📁 utils/
│   │       ├── mod.rs
│   │       ├── errors.rs          # Error types
│   │       ├── validation.rs      # Input validation
│   │       └── jwt.rs             # JWT helpers
│   ├── 📁 tests/
│   │   ├── integration.rs
│   │   └── api/
│   └── 📁 benches/
│
├── 📁 web/                         # Next.js Frontend
│   ├── 📄 package.json
│   ├── 📄 package-lock.json
│   ├── 📄 tsconfig.json
│   ├── 📄 next.config.js
│   ├── 📄 tailwind.config.ts
│   ├── 📄 .env.example
│   ├── 📄 .env.local
│   ├── 📄 README.md
│   ├── 📁 app/                    # Next.js App Router
│   │   ├── layout.tsx             # Root layout
│   │   ├── page.tsx               # Homepage
│   │   ├── globals.css            # Global styles
│   │   ├── 📁 (auth)/             # Auth routes
│   │   │   ├── login/
│   │   │   │   └── page.tsx
│   │   │   ├── register/
│   │   │   │   └── page.tsx
│   │   │   └── forgot-password/
│   │   │       └── page.tsx
│   │   ├── 📁 (marketplace)/      # Public marketplace
│   │   │   ├── layout.tsx
│   │   │   ├── browse/
│   │   │   │   └── page.tsx
│   │   │   ├── animation/[id]/
│   │   │   │   └── page.tsx
│   │   │   ├── search/
│   │   │   │   └── page.tsx
│   │   │   ├── category/[slug]/
│   │   │   │   └── page.tsx
│   │   │   └── creator/[username]/
│   │   │       └── page.tsx
│   │   ├── 📁 (dashboard)/        # User dashboard
│   │   │   ├── layout.tsx
│   │   │   ├── dashboard/
│   │   │   │   └── page.tsx
│   │   │   ├── my-animations/
│   │   │   │   ├── page.tsx
│   │   │   │   └── [id]/
│   │   │   │       └── page.tsx
│   │   │   ├── analytics/
│   │   │   │   └── page.tsx
│   │   │   ├── purchases/
│   │   │   │   └── page.tsx
│   │   │   ├── earnings/
│   │   │   │   └── page.tsx
│   │   │   └── settings/
│   │   │       └── page.tsx
│   │   └── 📁 api/                # API routes
│   │       ├── auth/[...nextauth]/
│   │       │   └── route.ts
│   │       └── upload/
│   │           └── route.ts
│   ├── 📁 components/
│   │   ├── 📁 ui/                 # Shadcn/UI components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   └── ...
│   │   ├── 📁 marketplace/
│   │   │   ├── AnimationCard.tsx
│   │   │   ├── AnimationGrid.tsx
│   │   │   ├── AnimationPlayer.tsx
│   │   │   ├── SearchBar.tsx
│   │   │   ├── FilterSidebar.tsx
│   │   │   └── FeaturedCarousel.tsx
│   │   ├── 📁 dashboard/
│   │   │   ├── StatsCards.tsx
│   │   │   ├── AnalyticsChart.tsx
│   │   │   ├── RecentActivity.tsx
│   │   │   └── UploadModal.tsx
│   │   └── 📁 layout/
│   │       ├── Header.tsx
│   │       ├── Footer.tsx
│   │       ├── Sidebar.tsx
│   │       └── MobileNav.tsx
│   ├── 📁 lib/
│   │   ├── api.ts                 # API client
│   │   ├── auth.ts                # Auth helpers
│   │   ├── utils.ts               # Utility functions
│   │   ├── constants.ts           # Constants
│   │   └── validations.ts         # Form validations
│   ├── 📁 hooks/
│   │   ├── useAnimation.ts
│   │   ├── useAuth.ts
│   │   ├── useSearch.ts
│   │   ├── useUpload.ts
│   │   └── useAnalytics.ts
│   ├── 📁 stores/
│   │   ├── authStore.ts           # Zustand auth store
│   │   ├── uiStore.ts             # UI state
│   │   └── cartStore.ts           # Shopping cart
│   ├── 📁 types/
│   │   ├── animation.ts
│   │   ├── user.ts
│   │   ├── api.ts
│   │   └── index.ts
│   ├── 📁 styles/
│   │   └── neo-brutalism.css      # Custom styles
│   └── 📁 public/
│       ├── logo.svg
│       ├── favicon.ico
│       └── 📁 images/
│
├── 📁 shared/                      # Shared code/types
│   ├── 📁 types/                  # TypeScript types
│   │   ├── animation.ts
│   │   ├── user.ts
│   │   └── api.ts
│   └── 📁 constants/
│       ├── animation.ts
│       └── marketplace.ts
│
├── 📁 docs/                        # Documentation
│   ├── 📄 README.md
│   ├── 📄 CLI.md                  # CLI documentation
│   ├── 📄 API.md                  # API reference
│   ├── 📄 ARCHITECTURE.md         # Architecture docs
│   ├── 📄 MARKETPLACE.md          # Marketplace guide
│   ├── 📄 PLUGINS.md              # Plugin development
│   ├── 📄 DEV_GUIDE.md            # Development guide
│   └── 📁 tutorials/
│       ├── getting-started.md
│       ├── advanced-usage.md
│       └── deploying.md
│
├── 📁 scripts/                     # Utility scripts
│   ├── setup.sh                   # Initial setup
│   ├── init-db.sql                # Database initialization
│   ├── seed-data.sh               # Seed test data
│   ├── deploy-staging.sh          # Staging deployment
│   ├── deploy-prod.sh             # Production deployment
│   ├── backup-db.sh               # Database backup
│   └── 📁 migrations/
│
├── 📁 tests/                       # Integration tests
│   ├── 📁 integration/
│   │   ├── test_full_workflow.sh
│   │   ├── test_cli.sh
│   │   ├── test_api.sh
│   │   └── test_marketplace.sh
│   ├── 📁 e2e/                    # End-to-end tests
│   │   ├── playwright.config.ts
│   │   └── 📁 specs/
│   │       ├── auth.spec.ts
│   │       ├── upload.spec.ts
│   │       └── purchase.spec.ts
│   └── 📁 fixtures/
│       ├── test_animations/
│       └── test_data.json
│
├── 📁 .github/                     # GitHub configuration
│   ├── 📁 workflows/              # CI/CD pipelines
│   │   ├── ci.yml                 # Continuous integration
│   │   ├── deploy.yml             # Deployment
│   │   ├── security.yml           # Security scans
│   │   └── release.yml            # Release automation
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── dependabot.yml
│
├── 📁 monitoring/                  # Monitoring configs
│   ├── prometheus.yml
│   └── 📁 grafana/
│       ├── 📁 dashboards/
│       └── 📁 datasources/
│
├── 📁 nginx/                       # Nginx configuration
│   ├── nginx.conf
│   └── 📁 ssl/
│
└── 📁 .vscode/                     # VS Code settings
    ├── settings.json
    ├── extensions.json
    └── launch.json
```

---

## Key Files Description

### Root Level

| File | Purpose |
|------|---------|
| `README.md` | Project overview, quick start |
| `CONTRIBUTING.md` | How to contribute |
| `TODO.md` | Development roadmap |
| `Makefile` | Build automation commands |
| `docker-compose.yml` | Development environment |
| `.env.example` | Environment variables template |

### CLI (`/cli`)

**Core Features:**
- User interaction and command parsing
- LLM communication
- Local file management
- Animation preview
- Marketplace interaction

**Key Technologies:**
- Rust with Tokio for async
- Clap for CLI parsing
- Reqwest for HTTP
- Serde for serialization

### Engine (`/engine`)

**Core Features:**
- Manim code generation
- Animation rendering
- Code validation
- Quality scoring
- Template system

**Key Technologies:**
- Python 3.8+
- Manim Community Edition
- Pydantic for validation
- Black for formatting

### API (`/api`)

**Core Features:**
- RESTful API endpoints
- Authentication & authorization
- Database operations
- File storage (S3)
- WebSocket support

**Key Technologies:**
- Rust with Actix-web
- PostgreSQL with SQLx
- Redis for caching
- AWS SDK for S3

### Web (`/web`)

**Core Features:**
- Marketplace interface
- User dashboard
- Animation player
- Upload system
- Real-time updates

**Key Technologies:**
- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS (Neo-brutalism)
- Zustand for state
- React Query for data

---

## File Size Estimates

```
cli/           ~50MB  (Rust binary)
engine/        ~200MB (Python + Manim + LaTeX)
api/           ~30MB  (Rust binary)
web/           ~100MB (Node modules)
Total:         ~380MB (development)
```

---

## Environment Variables

### CLI (`.env`)
```env
ANIMAFORGE_BACKEND=ollama
ANIMAFORGE_MODEL=llama3
ANIMAFORGE_API_KEY=
ANIMAFORGE_OUTPUT_DIR=./animations
```

### API (`.env`)
```env
DATABASE_URL=postgresql://user:pass@localhost/animaforge_dev
REDIS_URL=redis://localhost:6379
S3_ENDPOINT=http://localhost:9000
JWT_SECRET=your-secret-key
STRIPE_SECRET_KEY=sk_test_...
```

### Web (`.env.local`)
```env
NEXT_PUBLIC_API_URL=http://localhost:8080/v1
NEXT_PUBLIC_WS_URL=ws://localhost:8080
DATABASE_URL=postgresql://user:pass@localhost/animaforge_dev
NEXTAUTH_SECRET=your-nextauth-secret
```

---

## Port Allocations

| Service | Port | Purpose |
|---------|------|---------|
| Web | 3000 | Next.js frontend |
| API | 8080 | Rust backend |
| PostgreSQL | 5432 | Database |
| Redis | 6379 | Cache |
| Meilisearch | 7700 | Search engine |
| MinIO | 9000 | S3-compatible storage |
| MinIO Console | 9001 | Storage admin UI |
| Ollama | 11434 | Local LLM |
| Prometheus | 9090 | Metrics |
| Grafana | 3001 | Dashboards |

---

## Development Workflow

```mermaid
graph LR
    A[Clone Repo] --> B[Install Dependencies]
    B --> C[Setup Database]
    C --> D[Start Services]
    D --> E[Make Changes]
    E --> F[Run Tests]
    F --> G[Commit & Push]
    G --> H[Create PR]
```

---

## Build Output Locations

```
cli/target/release/animaforge           # CLI binary
api/target/release/animaforge-api       # API binary
engine/dist/animaforge_engine-*.whl     # Python package
web/.next/                              # Next.js build
web/out/                                # Static export
```

---

## Testing Structure

```
Unit Tests:
- cli/tests/              Rust unit tests
- engine/tests/           Python unit tests
- web/__tests__/          Jest tests

Integration Tests:
- tests/integration/      Shell scripts

E2E Tests:
- tests/e2e/             Playwright tests
```

---

This structure is designed for:
- ✅ Scalability
- ✅ Maintainability
- ✅ Clear separation of concerns
- ✅ Easy onboarding for new developers
- ✅ Production-ready architecture

---

**Last Updated:** 2025  
**Version:** 1.0.0
