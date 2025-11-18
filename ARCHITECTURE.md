# AnimaForge Architecture 🏗️

## System Overview

AnimaForge is a distributed system designed for high-performance animation generation and marketplace operations. The architecture follows microservices principles with clear separation of concerns.

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER LAYER                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │   CLI    │  │   Web    │  │  Mobile  │  │   API    │      │
│  │  (Rust)  │  │(Next.js) │  │  (RN)    │  │  Clients │      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘      │
└───────┼─────────────┼─────────────┼─────────────┼─────────────┘
        │             │             │             │
        └─────────────┴─────────────┴─────────────┘
                      │
        ┌─────────────▼──────────────────────────────────┐
        │          API GATEWAY / Load Balancer            │
        │           (Nginx / Cloudflare)                  │
        └─────────────┬──────────────────────────────────┘
                      │
        ┌─────────────▼──────────────────────────────────┐
        │            BACKEND SERVICES                     │
        │  ┌──────────────┐  ┌───────────────┐          │
        │  │  API Service │  │ Auth Service  │          │
        │  │ (Rust/Actix) │  │   (OAuth2)    │          │
        │  └──────┬───────┘  └───────┬───────┘          │
        │         │                   │                   │
        │  ┌──────▼──────┐  ┌────────▼──────┐          │
        │  │  Generation │  │   Marketplace │          │
        │  │   Service   │  │    Service    │          │
        │  │  (Python)   │  │  (Rust/TS)    │          │
        │  └──────┬──────┘  └────────┬──────┘          │
        │         │                   │                   │
        │  ┌──────▼──────────────────▼──────┐          │
        │  │      Queue Service              │          │
        │  │    (Redis/BullMQ)               │          │
        │  └─────────────────────────────────┘          │
        └────────────┬─────────────────────────────────┘
                     │
        ┌────────────▼─────────────────────────────────┐
        │            DATA LAYER                         │
        │  ┌──────────┐  ┌────────┐  ┌──────────┐    │
        │  │PostgreSQL│  │  Redis │  │    S3    │    │
        │  │  (Main)  │  │ (Cache)│  │ (Files)  │    │
        │  └──────────┘  └────────┘  └──────────┘    │
        │                                              │
        │  ┌──────────┐  ┌────────────┐              │
        │  │Meilisearch│ │  Analytics │              │
        │  │ (Search) │  │(ClickHouse)│              │
        │  └──────────┘  └────────────┘              │
        └──────────────────────────────────────────────┘

        ┌──────────────────────────────────────────────┐
        │         EXTERNAL SERVICES                     │
        │  ┌──────────┐  ┌────────┐  ┌──────────┐    │
        │  │  Ollama  │  │ Gemini │  │  Claude  │    │
        │  │  (Local) │  │ (API)  │  │  (API)   │    │
        │  └──────────┘  └────────┘  └──────────┘    │
        └──────────────────────────────────────────────┘
```

---

## Core Components

### 1. CLI (Rust)

**Location**: `/cli`

**Responsibilities:**
- User interaction and command parsing
- Local file management
- LLM communication orchestration
- Animation preview and rendering
- Marketplace interaction

**Tech Stack:**
- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` - Serialization/deserialization
- `inquire` - Interactive prompts

**Key Modules:**
```rust
cli/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # Command definitions
│   ├── config/
│   │   ├── mod.rs          # Config management
│   │   └── providers.rs    # LLM provider configs
│   ├── commands/
│   │   ├── create.rs       # Animation creation
│   │   ├── render.rs       # Rendering logic
│   │   ├── publish.rs      # Marketplace upload
│   │   └── search.rs       # Search functionality
│   ├── llm/
│   │   ├── mod.rs          # LLM abstraction
│   │   ├── ollama.rs       # Ollama integration
│   │   ├── gemini.rs       # Gemini integration
│   │   └── claude.rs       # Claude integration
│   ├── engine/
│   │   ├── mod.rs          # Engine coordination
│   │   └── manim.rs        # Manim wrapper
│   └── utils/
│       ├── mod.rs
│       ├── progress.rs     # Progress bars
│       └── errors.rs       # Error handling
└── Cargo.toml
```

**Data Flow:**
```
User Input → CLI Parser → Config Loader → LLM Provider → 
Python Engine → Manim → FFmpeg → Output File
```

---

### 2. Animation Engine (Python)

**Location**: `/engine`

**Responsibilities:**
- Manim code generation and validation
- Animation rendering
- Code optimization
- Quality scoring

**Tech Stack:**
- `manim` - Animation library
- `pydantic` - Data validation
- `ast` - Python code parsing
- `black` - Code formatting

**Key Modules:**
```python
engine/
├── animaforge_engine/
│   ├── __init__.py
│   ├── generator.py         # Code generation
│   ├── validator.py         # Code validation
│   ├── renderer.py          # Rendering logic
│   ├── optimizer.py         # Code optimization
│   ├── quality_scorer.py    # Quality assessment
│   ├── templates/
│   │   ├── base.py         # Base templates
│   │   ├── math.py         # Math templates
│   │   └── physics.py      # Physics templates
│   └── utils/
│       ├── code_parser.py  # AST parsing
│       └── manim_wrapper.py
├── tests/
└── setup.py
```

**Rendering Pipeline:**
```
LLM Output → Code Parser → Validator → Optimizer → 
Manim Compiler → FFmpeg Encoder → Video File
```

---

### 3. Backend API (Rust)

**Location**: `/api`

**Responsibilities:**
- RESTful API endpoints
- Authentication & authorization
- Database operations
- File storage management
- WebSocket connections

**Tech Stack:**
- `actix-web` - Web framework
- `sqlx` - Database driver
- `jsonwebtoken` - JWT handling
- `aws-sdk-s3` - S3 integration

**API Structure:**
```rust
api/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth.rs          # /api/v1/auth/*
│   │   ├── animations.rs    # /api/v1/animations/*
│   │   ├── users.rs         # /api/v1/users/*
│   │   ├── marketplace.rs   # /api/v1/marketplace/*
│   │   └── search.rs        # /api/v1/search/*
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── animation.rs
│   │   └── transaction.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── pool.rs
│   │   └── migrations/
│   ├── middleware/
│   │   ├── auth.rs
│   │   ├── rate_limit.rs
│   │   └── cors.rs
│   ├── services/
│   │   ├── storage.rs       # S3 operations
│   │   ├── search.rs        # Meilisearch
│   │   └── payment.rs       # Stripe
│   └── utils/
│       ├── errors.rs
│       └── validation.rs
└── Cargo.toml
```

**API Endpoints:**
```
Authentication:
POST   /api/v1/auth/register
POST   /api/v1/auth/login
POST   /api/v1/auth/logout
GET    /api/v1/auth/me
POST   /api/v1/auth/refresh

Animations:
GET    /api/v1/animations           # List animations
POST   /api/v1/animations           # Upload new
GET    /api/v1/animations/:id       # Get details
PUT    /api/v1/animations/:id       # Update
DELETE /api/v1/animations/:id       # Delete
GET    /api/v1/animations/:id/download
POST   /api/v1/animations/:id/like

Marketplace:
GET    /api/v1/marketplace/featured
GET    /api/v1/marketplace/trending
POST   /api/v1/marketplace/purchase
GET    /api/v1/marketplace/my-purchases

Users:
GET    /api/v1/users/:id/profile
GET    /api/v1/users/:id/animations
POST   /api/v1/users/:id/follow
GET    /api/v1/users/:id/analytics

Search:
GET    /api/v1/search?q=...
GET    /api/v1/search/autocomplete
```

---

### 4. Web Frontend (Next.js)

**Location**: `/web`

**Responsibilities:**
- User interface
- Marketplace display
- User dashboard
- Animation player
- Real-time updates

**Tech Stack:**
- Next.js 14 (App Router)
- TypeScript
- Tailwind CSS
- Zustand (State management)
- React Query (Data fetching)
- Socket.io (WebSocket)

**Directory Structure:**
```
web/
├── app/
│   ├── (auth)/
│   │   ├── login/
│   │   └── register/
│   ├── (marketplace)/
│   │   ├── page.tsx         # Homepage
│   │   ├── browse/
│   │   ├── animation/[id]/
│   │   └── search/
│   ├── (dashboard)/
│   │   ├── dashboard/
│   │   ├── my-animations/
│   │   ├── analytics/
│   │   └── settings/
│   └── api/
│       └── [...]/           # API routes
├── components/
│   ├── ui/                  # Shadcn components
│   ├── marketplace/
│   │   ├── AnimationCard.tsx
│   │   ├── AnimationPlayer.tsx
│   │   └── SearchBar.tsx
│   ├── dashboard/
│   └── layout/
├── lib/
│   ├── api.ts              # API client
│   ├── auth.ts             # Auth helpers
│   └── utils.ts
├── hooks/
│   ├── useAnimation.ts
│   ├── useAuth.ts
│   └── useSearch.ts
├── stores/
│   ├── authStore.ts
│   └── uiStore.ts
└── styles/
    └── globals.css
```

**Neo-Brutalism Design System:**
```tsx
// Color Palette
const colors = {
  primary: '#FF6B35',    // Bright orange
  secondary: '#004E89',  // Deep blue
  accent: '#F7B801',     // Yellow
  black: '#000000',
  white: '#FFFFFF',
  gray: '#CCCCCC'
};

// Typography
const typography = {
  heading: 'font-black uppercase tracking-tight',
  body: 'font-bold',
  mono: 'font-mono font-bold'
};

// Borders & Shadows
const effects = {
  border: 'border-4 border-black',
  shadow: 'shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]',
  hoverShadow: 'hover:shadow-[12px_12px_0px_0px_rgba(0,0,0,1)]'
};
```

---

## Database Schema

### PostgreSQL Tables

```sql
-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Animations
CREATE TABLE animations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    duration DECIMAL(5,2),
    fps INTEGER DEFAULT 60,
    resolution VARCHAR(20),
    file_url TEXT NOT NULL,
    thumbnail_url TEXT,
    source_code TEXT,
    license VARCHAR(50) DEFAULT 'MIT',
    price DECIMAL(10,2) DEFAULT 0.00,
    is_public BOOLEAN DEFAULT TRUE,
    view_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Animation Tags
CREATE TABLE animation_tags (
    animation_id UUID REFERENCES animations(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    PRIMARY KEY (animation_id, tag)
);

-- Animation Versions
CREATE TABLE animation_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    animation_id UUID REFERENCES animations(id) ON DELETE CASCADE,
    version_number INTEGER NOT NULL,
    file_url TEXT NOT NULL,
    source_code TEXT,
    changelog TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(animation_id, version_number)
);

-- Purchases
CREATE TABLE purchases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    animation_id UUID REFERENCES animations(id),
    amount DECIMAL(10,2) NOT NULL,
    stripe_transaction_id VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, animation_id)
);

-- Likes
CREATE TABLE likes (
    user_id UUID REFERENCES users(id),
    animation_id UUID REFERENCES animations(id),
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (user_id, animation_id)
);

-- Comments
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    animation_id UUID REFERENCES animations(id),
    parent_id UUID REFERENCES comments(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Followers
CREATE TABLE followers (
    follower_id UUID REFERENCES users(id),
    following_id UUID REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (follower_id, following_id)
);

-- Analytics Events
CREATE TABLE analytics_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    animation_id UUID REFERENCES animations(id),
    user_id UUID REFERENCES users(id),
    event_type VARCHAR(50) NOT NULL, -- view, download, like, share
    metadata JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_animations_user ON animations(user_id);
CREATE INDEX idx_animations_public ON animations(is_public) WHERE is_public = TRUE;
CREATE INDEX idx_animations_created ON animations(created_at DESC);
CREATE INDEX idx_tags ON animation_tags(tag);
CREATE INDEX idx_analytics_animation ON analytics_events(animation_id);
CREATE INDEX idx_analytics_type ON analytics_events(event_type);
```

---

## Data Flow Diagrams

### Animation Generation Flow
```
┌──────────┐
│   User   │
│  Enters  │
│  Prompt  │
└────┬─────┘
     │
     ▼
┌──────────────────┐
│  CLI validates   │
│  & processes     │
│  prompt          │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Send to LLM     │◄──── (Ollama/Gemini/Claude)
│  Get Manim code  │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Python Engine   │
│  validates code  │
└────┬─────────────┘
     │
     ▼
   Valid?
   /    \
  No     Yes
  │       │
  │       ▼
  │   ┌──────────────┐
  │   │ Render with  │
  │   │   Manim      │
  │   └──────┬───────┘
  │          │
  │          ▼
  │   ┌──────────────┐
  │   │  Encode to   │
  │   │  MP4/GIF     │
  │   └──────┬───────┘
  │          │
  └──►Retry◄─┘
         │
         ▼
   ┌─────────────┐
   │ Show to User│
   └─────────────┘
```

### Marketplace Upload Flow
```
┌──────────┐
│   User   │
│  Uploads │
│Animation │
└────┬─────┘
     │
     ▼
┌──────────────────┐
│  API receives    │
│  file & metadata │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Validate file   │
│  (size, format)  │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Generate        │
│  thumbnail       │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Upload to S3    │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Save metadata   │
│  to PostgreSQL   │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Index in        │
│  Meilisearch     │
└────┬─────────────┘
     │
     ▼
┌──────────────────┐
│  Notify user     │
│  (WebSocket)     │
└──────────────────┘
```

---

## Security Architecture

### Authentication Flow
```
1. User submits credentials
2. API validates against DB
3. Generate JWT with claims:
   {
     "sub": "user_id",
     "exp": timestamp,
     "roles": ["user" | "creator" | "admin"]
   }
4. Return token + refresh token
5. Client stores in httpOnly cookie
6. Subsequent requests include token in header
7. Middleware validates token on each request
```

### Authorization Levels
- **Public**: View marketplace, search
- **User**: Download free animations, like, comment
- **Creator**: Upload animations, earn money
- **Pro**: Unlimited uploads, analytics, API access
- **Admin**: Moderation, user management

### Security Measures
- Rate limiting (100 req/min per IP)
- SQL injection prevention (parameterized queries)
- XSS protection (Content Security Policy)
- CSRF tokens for state-changing operations
- Input validation on all endpoints
- File upload scanning (ClamAV)
- Encrypted passwords (Argon2)
- HTTPS only in production

---

## Scalability Considerations

### Horizontal Scaling
- **API**: Stateless design allows multiple instances behind load balancer
- **Rendering**: Queue-based system enables distributed workers
- **Database**: Read replicas for queries, master for writes
- **Storage**: CDN for static assets

### Caching Strategy
```
Level 1 (Browser):     Cache-Control headers
Level 2 (CDN):         Cloudflare cache
Level 3 (Redis):       Hot data (user sessions, popular animations)
Level 4 (DB):          Query result caching
```

### Performance Targets
- API response time: <100ms (p95)
- Page load time: <2s (First Contentful Paint)
- Animation generation: <30s (standard quality)
- Search latency: <50ms
- Concurrent users: 10,000+

---

## Monitoring & Observability

### Metrics to Track
- API latency (p50, p95, p99)
- Error rates by endpoint
- Animation generation success rate
- User engagement (DAU, MAU)
- Revenue metrics
- Storage costs
- LLM API costs

### Logging Strategy
- Application logs → Structured JSON
- Access logs → Nginx format
- Error tracking → Sentry
- Metrics → Prometheus
- Visualization → Grafana

### Alerting Rules
- API error rate >5%
- Database connection pool exhausted
- Disk usage >80%
- Animation failure rate >10%
- Payment processing errors

---

## Deployment Architecture

```
┌─────────────────────────────────────────────┐
│              Cloudflare CDN                  │
│         (DDoS protection, caching)           │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│           AWS / DigitalOcean                 │
│                                              │
│  ┌────────────┐  ┌────────────┐            │
│  │   Web      │  │    API     │            │
│  │  Server    │  │   Server   │            │
│  │ (Vercel)   │  │  (EC2/DO)  │            │
│  └────────────┘  └────────────┘            │
│                                              │
│  ┌────────────┐  ┌────────────┐            │
│  │  Render    │  │ PostgreSQL │            │
│  │  Workers   │  │  (RDS/DO)  │            │
│  │  (EC2/DO)  │  └────────────┘            │
│  └────────────┘                             │
│                                              │
│  ┌────────────┐  ┌────────────┐            │
│  │   Redis    │  │     S3     │            │
│  │ (Elasticache)│ (Storage)  │            │
│  └────────────┘  └────────────┘            │
└──────────────────────────────────────────────┘
```

---

## Technology Decisions

### Why Rust for CLI & API?
- **Performance**: Near C++ speeds
- **Safety**: Memory safety without garbage collection
- **Concurrency**: Fearless concurrency
- **Reliability**: Catch bugs at compile time

### Why Python for Animation Engine?
- **Manim**: Native Python library
- **Ecosystem**: Rich scientific computing libraries
- **Flexibility**: Easy to extend and customize

### Why Next.js for Frontend?
- **SSR**: Better SEO and initial load times
- **API Routes**: Unified codebase
- **TypeScript**: Type safety
- **Ecosystem**: Huge library availability

### Why PostgreSQL?
- **ACID**: Strong consistency guarantees
- **JSON support**: Flexible metadata storage
- **Full-text search**: Built-in search capabilities
- **Reliability**: Battle-tested at scale

---

## Future Enhancements

### Phase 2
- GraphQL API
- Real-time collaboration (WebRTC)
- Mobile apps (React Native)
- Plugin marketplace

### Phase 3
- AI animation editing
- Voice-to-animation
- 3D animation support
- VR/AR previews

### Phase 4
- Blockchain-based licensing (NFTs)
- Decentralized storage (IPFS)
- Federated marketplace
- AI model training on user data (opt-in)

---

**Architecture maintained by**: AnimaForge Core Team
**Last Updated**: 2025
**Version**: 1.0.0
