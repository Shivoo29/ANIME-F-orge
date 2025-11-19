# AnimaForge - Developer Implementation Guide

> **Internal Document for Developers**
> Last Updated: 2025-11-19
> Status: MVP/Beta Ready
> Production Readiness: 5.5/10

---

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Current State](#current-state)
3. [Architecture](#architecture)
4. [Getting Started](#getting-started)
5. [Known Issues & Bugs](#known-issues--bugs)
6. [What Works vs What Doesn't](#what-works-vs-what-doesnt)
7. [Common Errors](#common-errors)
8. [Testing](#testing)
9. [Deployment](#deployment)
10. [Contributing](#contributing)

---

## 📖 Project Overview

### What Is AnimaForge?

AnimaForge is an **AI-powered animation creation platform** that converts natural language descriptions into professional Manim animations. Think of it as "GitHub Copilot for animations."

**Core Value Proposition:**
> "Transform words into stunning animations. No code required."

**Target Users:**
- Educators (math, physics, science)
- Content creators (YouTube, social media)
- Developers (technical documentation)
- Designers (motion graphics)

### Tech Stack

```
Frontend:  Next.js 14 + TypeScript + Tailwind CSS
Backend:   Rust (Actix-web) + PostgreSQL + SQLx
Engine:    Python 3.11 + Manim Community Edition
CLI:       Rust (clap, tokio, reqwest)
AI:        Ollama (local) / Gemini / Claude APIs
Storage:   S3-compatible (MinIO/AWS)
Search:    Meilisearch (planned)
Cache:     Redis (planned)
```

### Repository Structure

```
animaforge/
├── cli/              # Rust CLI tool (1,619 lines)
├── api/              # Rust backend API (1,500+ lines)
├── engine/           # Python animation engine (2,343 lines)
├── web/              # Next.js frontend (3,000+ lines)
├── scripts/          # Setup and database scripts
├── docs/             # (Future) Additional documentation
└── tests/            # (Future) Integration tests
```

---

## 🎯 Current State

### What Stage Are We At?

**Status:** **MVP/Beta Ready** (Not Full Production)

```
Idea → Prototype → MVP → Beta → Production
                     ^      ^
                     |      |
                  Done   Here
```

### Completion Breakdown

| Component | Code Complete | Tested | Production Ready |
|-----------|---------------|--------|------------------|
| CLI | ✅ 100% | ⚠️ Build only | ⚠️ 60% |
| API | ✅ 100% | ⚠️ Build only | ⚠️ 50% |
| Engine | ✅ 100% | ⚠️ Core only | ⚠️ 40% |
| Frontend | ✅ 100% | ⚠️ Build only | ⚠️ 70% |
| Database | ✅ 100% | ✅ Schema tested | ✅ 80% |
| Docs | ✅ 100% | N/A | ✅ 90% |

### What We've Built

✅ **Complete:**
- All 4 main components (CLI, API, Engine, Web)
- Authentication system (JWT + bcrypt)
- Database schema with migrations
- Neo-brutalism design system
- User journey and onboarding flow
- 12 REST API endpoints
- 5 CLI commands
- 10+ documentation files

⚠️ **Partially Complete:**
- LLM integrations (code exists, not tested end-to-end)
- File upload/download (endpoints exist, S3 not tested)
- Search functionality (basic implementation)
- Manim rendering (code exists, needs full deps)

❌ **Not Started:**
- Automated testing (unit, integration, E2E)
- CI/CD pipeline (scaffolded only)
- Monitoring and logging
- Caching layer
- Rate limiting (implemented but not tested)
- Email verification
- Payment integration

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────┐
│                    USER LAYER                            │
│  Browser ──→ Next.js Frontend ──→ React Components      │
│  Terminal ──→ Rust CLI ──→ Config + LLM                 │
└─────────────────────┬───────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                   API LAYER                              │
│  Actix-web ──→ Routes ──→ Middleware ──→ Handlers      │
│     │                         │                           │
│     └── JWT Auth ─────────────┘                          │
└─────────────────────┬───────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
   ┌─────────┐  ┌─────────┐  ┌──────────┐
   │PostgreSQL│  │  Redis  │  │    S3    │
   │   DB     │  │ (Future)│  │ (Future) │
   └─────────┘  └─────────┘  └──────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────┐
│                 WORKER LAYER                             │
│  Python Engine ──→ LLM APIs ──→ Manim ──→ FFmpeg       │
└─────────────────────────────────────────────────────────┘
```

### Data Flow: Creating an Animation

```
1. User enters prompt in Web UI or CLI
   ↓
2. Request sent to API: POST /api/v1/animations
   ↓
3. API validates JWT token
   ↓
4. API calls Python Engine (subprocess or API)
   ↓
5. Engine sends prompt to LLM (Ollama/Gemini/Claude)
   ↓
6. LLM returns Manim code
   ↓
7. Engine validates code (AST parsing)
   ↓
8. Engine renders animation with Manim
   ↓
9. Video file saved to S3
   ↓
10. Metadata saved to PostgreSQL
    ↓
11. Response returned to user
```

### Database Schema

**Tables:**
- `users` - User accounts (auth, profiles)
- `animations` - Animation metadata
- `animation_tags` - Tags for discovery

**Key Relationships:**
```sql
animations.user_id → users.id (CASCADE DELETE)
animation_tags.animation_id → animations.id (CASCADE DELETE)
```

**Indexes:**
- `idx_animations_user` - Fast user queries
- `idx_animations_public` - Public listings
- `idx_tags` - Tag filtering
- `idx_animations_likes` - Sorting by popularity

---

## 🚀 Getting Started

### Prerequisites

**Required:**
- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Python 3.8+ (`python3 --version`)
- Node.js 18+ (`node --version`)
- PostgreSQL 13+ (`psql --version`)

**Optional (for full functionality):**
- FFmpeg (`ffmpeg -version`)
- LaTeX (`pdflatex --version`)
- Ollama (`ollama --version`)
- Redis (`redis-cli --version`)

### Quick Setup (5 minutes)

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/animaforge.git
cd animaforge

# 2. Run automated setup
./scripts/setup.sh

# This will:
# - Install all dependencies
# - Create database and seed data
# - Build all components
# - Create config files
```

### Manual Setup

#### 1. Database Setup

```bash
# Create database
createdb animaforge_dev

# Initialize schema
psql -d animaforge_dev -f scripts/init-db.sql

# Seed sample data (optional but recommended)
psql -d animaforge_dev -f scripts/seed-data.sql

# Verify
psql -d animaforge_dev -c "SELECT COUNT(*) FROM users;"
# Should return 5 users
```

#### 2. Python Engine

```bash
cd engine

# Create virtual environment
python3 -m venv venv

# Activate
source venv/bin/activate

# Install package in development mode
pip install -e .

# Test import
python -c "from animaforge_engine import validator; print('OK')"

# Deactivate when done
deactivate

cd ..
```

**Note:** Full Manim rendering requires system dependencies:
```bash
# Ubuntu/Debian
sudo apt-get install ffmpeg texlive texlive-latex-extra libpango1.0-dev

# macOS
brew install ffmpeg texlive pango
```

#### 3. Rust API

```bash
cd api

# Copy environment template
cp .env.example .env

# Edit .env with your database URL
# DATABASE_URL=postgresql://localhost/animaforge_dev
nano .env

# Build (first time is slow ~2 minutes)
cargo build --release

# Run
cargo run

# API should start at http://localhost:8080

cd ..
```

#### 4. Rust CLI

```bash
cd cli

# Copy environment template
cp .env.example .env

# Build
cargo build --release

# Test
./target/release/animaforge --help

cd ..
```

#### 5. Next.js Frontend

```bash
cd web

# Install dependencies
npm install

# Copy environment template
cp .env.example .env

# Edit .env
# NEXT_PUBLIC_API_URL=http://localhost:8080/v1
nano .env

# Run development server
npm run dev

# Frontend should start at http://localhost:3000

cd ..
```

### Running the Full Stack

**Terminal 1 - API:**
```bash
cd api
cargo run
```

**Terminal 2 - Frontend:**
```bash
cd web
npm run dev
```

**Terminal 3 - CLI (optional):**
```bash
cd cli
./target/release/animaforge config list
```

**Access:**
- Frontend: http://localhost:3000
- API: http://localhost:8080
- API Health: http://localhost:8080/health (when implemented)

### Demo Credentials

```
Email: demo@animaforge.dev
Password: password123

Other accounts:
- math@animaforge.dev
- creative@animaforge.dev
- physics@animaforge.dev
- data@animaforge.dev

All use password: password123
```

---

## 🐛 Known Issues & Bugs

### Critical Issues

#### 1. **Manim Rendering Not Tested End-to-End** 🔴
**Status:** Code exists but not verified

**Issue:**
```python
# This code exists but hasn't been tested with actual Manim
def render_animation(code, output_path, quality='medium'):
    # Subprocess call to Manim
    # No integration test
```

**Impact:** High - Core feature might not work

**Workaround:** Manual testing required

**Fix Priority:** P0 - Must test before launch

#### 2. **LLM APIs Not Integrated** 🔴
**Status:** Code exists but no API keys tested

**Issue:**
```rust
// Ollama, Gemini, Claude integrations written
// But never called with real API keys
```

**Impact:** High - Can't generate animations

**Workaround:** Use templates instead

**Fix Priority:** P0 - Must integrate before launch

#### 3. **S3 Upload/Download Not Tested** 🟡
**Status:** Endpoints exist, no real S3 connection

**Issue:**
```rust
// POST /api/v1/animations
// Code assumes S3, but never tested
```

**Impact:** Medium - Files stored locally for now

**Workaround:** Store files on local filesystem

**Fix Priority:** P1 - Needed for production

### Build Warnings

#### 1. **Frontend: `location is not defined`** 🟡
**File:** `web/app/dashboard/page.tsx`

**Warning:**
```
ReferenceError: location is not defined
    at Dashboard page static generation
```

**Cause:** Using `window.location` during SSR

**Impact:** Low - Build succeeds, just a warning

**Fix:**
```typescript
// Add check
if (typeof window !== 'undefined') {
  window.location.href = '/somewhere';
}
```

**Priority:** P2 - Cosmetic

#### 2. **API: Unnecessary Clone Calls** 🟢
**Files:** `api/src/routes/animations.rs`, `users.rs`

**Warning:**
```rust
warning: call to `.clone()` on a reference in this situation does nothing
```

**Cause:** Claims struct doesn't implement Clone

**Impact:** None - Just inefficient

**Fix:**
```rust
// Option 1: Add derive
#[derive(Clone)]
pub struct Claims { ... }

// Option 2: Remove .clone()
let claims = req.extensions().get::<Claims>()
    .ok_or(...)?;
```

**Priority:** P3 - Optimization

#### 3. **CLI: Unused Functions** 🟢
**File:** `cli/src/utils/progress.rs`

**Warning:**
```
warning: function `create_download_bar` is never used
warning: struct `MultiStepProgress` is never constructed
```

**Cause:** Utility functions written but not called yet

**Impact:** None - Just code bloat

**Fix:** Remove unused code or use it

**Priority:** P3 - Cleanup

### Missing Features

#### 1. **No Rate Limiting** 🟡
**Status:** Middleware exists but not configured

**Issue:**
```rust
// Code for rate limiting exists
// But not enabled in routes
```

**Impact:** Medium - API abuse possible

**Fix:** Enable middleware

**Priority:** P1 - Security

#### 2. **No Email Verification** 🟡
**Status:** Users can register but email not verified

**Impact:** Medium - Spam accounts possible

**Fix:** Add email service integration

**Priority:** P2 - User experience

#### 3. **No Error Boundaries** 🟡
**Status:** Frontend has no error boundaries

**Issue:**
```typescript
// If component crashes, whole app crashes
// No graceful error handling
```

**Impact:** Medium - Poor UX

**Fix:** Add React error boundaries

**Priority:** P1 - User experience

---

## ✅ What Works vs What Doesn't

### ✅ What Actually Works

**Frontend:**
- ✅ Builds successfully (99.2KB bundle)
- ✅ All pages render
- ✅ Tailwind CSS works
- ✅ Components display correctly
- ✅ Navigation works
- ✅ Forms render
- ⚠️ API calls work (if API is running)

**API:**
- ✅ Compiles successfully
- ✅ Serves HTTP requests
- ✅ JWT generation works
- ✅ Password hashing works
- ⚠️ Database queries work (if DB connected)
- ⚠️ Routes return data (mock or DB)

**CLI:**
- ✅ Compiles successfully
- ✅ Config management works
- ✅ Help system works
- ✅ Interactive prompts work
- ⚠️ Can call Python engine (if installed)

**Engine:**
- ✅ Code validation works (AST parsing)
- ✅ Templates work
- ✅ Import structures correct
- ⚠️ Code generation works (if LLM available)
- ⚠️ Rendering works (if Manim installed)

**Database:**
- ✅ Schema is valid
- ✅ Migrations work
- ✅ Sample data loads
- ✅ Queries execute
- ✅ Constraints enforced

### ❌ What Doesn't Work

**Integration Issues:**
- ❌ Full end-to-end animation creation (not tested)
- ❌ Real S3 upload/download (no S3 configured)
- ❌ Real LLM API calls (no keys configured)
- ❌ Email sending (no service configured)
- ❌ Payment processing (not implemented)

**Missing Infrastructure:**
- ❌ Redis caching (not connected)
- ❌ Meilisearch (not set up)
- ❌ Monitoring (Sentry not configured)
- ❌ Logging aggregation (none)
- ❌ CDN (not configured)

**No Automated Tests:**
- ❌ Unit tests
- ❌ Integration tests
- ❌ E2E tests
- ❌ Load tests
- ⚠️ Only build verification tests

**Security Gaps:**
- ❌ Rate limiting not enabled
- ❌ CSRF protection incomplete
- ❌ Input validation partial
- ❌ Security headers not configured
- ❌ API abuse prevention missing

---

## ⚠️ Common Errors

### Error 1: Database Connection Failed

**Error:**
```
Error: Failed to connect to database
    at sqlx::pool::connect
```

**Cause:** PostgreSQL not running or wrong connection string

**Fix:**
```bash
# Check if PostgreSQL is running
pg_isadmin

# Check connection string in .env
cat api/.env | grep DATABASE_URL

# Should be: postgresql://localhost/animaforge_dev

# Create database if missing
createdb animaforge_dev
```

### Error 2: Python Module Not Found

**Error:**
```
ModuleNotFoundError: No module named 'animaforge_engine'
```

**Cause:** Virtual environment not activated or package not installed

**Fix:**
```bash
cd engine
source venv/bin/activate
pip install -e .
python -c "from animaforge_engine import validator; print('OK')"
```

### Error 3: Manim Dependencies Missing

**Error:**
```
Package 'pangocairo' was not found
```

**Cause:** System dependencies not installed

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y ffmpeg texlive texlive-latex-extra libpango1.0-dev

# macOS
brew install ffmpeg texlive pango

# Then retry
pip install manim
```

### Error 4: Port Already in Use

**Error:**
```
Error: Address already in use (os error 48)
```

**Cause:** Another process using port 8080 or 3000

**Fix:**
```bash
# Find process using port
lsof -i :8080
lsof -i :3000

# Kill it
kill -9 <PID>

# Or change port in .env
PORT=8081
```

### Error 5: Cargo Build Fails

**Error:**
```
error: linker `cc` not found
```

**Cause:** C compiler not installed

**Fix:**
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Then retry
cargo build
```

### Error 6: npm Install Fails

**Error:**
```
EACCES: permission denied
```

**Cause:** Node modules directory permission issue

**Fix:**
```bash
# Use correct Node version
node --version  # Should be 18+

# Clean and retry
rm -rf node_modules package-lock.json
npm install

# Or use correct permissions
sudo chown -R $USER ~/.npm
```

### Error 7: JWT Token Invalid

**Error:**
```json
{
  "error": "Unauthorized",
  "message": "Invalid token"
}
```

**Cause:** JWT_SECRET mismatch or expired token

**Fix:**
```bash
# Ensure JWT_SECRET is set in api/.env
JWT_SECRET=your-secret-key-at-least-32-characters-long

# Clear cookies and login again
# Token expires after 24 hours by default
```

### Error 8: CORS Error in Browser

**Error:**
```
Access to fetch at 'http://localhost:8080' from origin 'http://localhost:3000'
has been blocked by CORS policy
```

**Cause:** CORS not properly configured

**Fix:**
```rust
// In api/src/middleware/cors.rs
// Ensure origin is set correctly
.allowed_origin("http://localhost:3000")
```

---

## 🧪 Testing

### Current Testing Status

**Build Tests:** ✅ All pass
- CLI compiles in 10.61s
- API compiles in 1.16s
- Frontend builds in ~45s
- Engine imports work

**Runtime Tests:** ❌ None

**Coverage:** ~0% (no automated tests)

### Running Build Tests

```bash
# Test everything
./scripts/test-all.sh

# Individual components
cd cli && cargo check
cd api && cargo check
cd web && npm run build
cd engine && python -c "from animaforge_engine import validator"
```

### What Needs Testing (TODO)

#### 1. Unit Tests

**CLI:**
```rust
// tests/config_tests.rs
#[test]
fn test_config_set() {
    let config = Config::new();
    config.set("backend", "ollama");
    assert_eq!(config.get("backend"), Some("ollama"));
}
```

**API:**
```rust
// tests/auth_tests.rs
#[actix_web::test]
async fn test_register() {
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}
```

**Engine:**
```python
# tests/test_generator.py
def test_code_generation():
    code = generate_animation_code("rotating cube", "ollama")
    assert "class" in code
    assert "Scene" in code
```

**Frontend:**
```typescript
// __tests__/Button.test.tsx
test('renders button', () => {
  render(<Button>Click me</Button>);
  expect(screen.getByText('Click me')).toBeInTheDocument();
});
```

#### 2. Integration Tests

```bash
# tests/integration/test_full_flow.sh
# 1. Start API
# 2. Call register endpoint
# 3. Get JWT token
# 4. Create animation
# 5. Verify in database
# 6. Download animation
```

#### 3. E2E Tests

```typescript
// tests/e2e/auth.spec.ts
test('user can sign up and login', async ({ page }) => {
  await page.goto('http://localhost:3000');
  await page.click('text=Sign Up');
  await page.fill('[name=email]', 'test@test.com');
  await page.fill('[name=password]', 'password123');
  await page.click('button:has-text("Create Account")');
  await expect(page).toHaveURL('/dashboard');
});
```

### Testing Checklist

- [ ] Add pytest to engine
- [ ] Add cargo test to CLI
- [ ] Add cargo test to API
- [ ] Add jest to frontend
- [ ] Add integration tests
- [ ] Add E2E tests (Playwright)
- [ ] Add load tests (k6)
- [ ] Setup test database
- [ ] Add CI/CD for tests
- [ ] Achieve >80% coverage

---

## 🚀 Deployment

### Current Deployment Status

**Status:** ❌ Not deployed anywhere

**Infrastructure:**
- No CI/CD pipeline (GitHub Actions scaffolded)
- No Docker images built
- No staging environment
- No production environment

### What You Need to Deploy

#### Minimum Deployment (Beta)

```
✅ PostgreSQL database (managed)
✅ Rust API binary (single server)
✅ Next.js frontend (Vercel/Netlify)
⚠️ One LLM provider (Ollama or API key)
```

**Estimated Cost:** $50-100/month

#### Full Production Deployment

```
✅ PostgreSQL (RDS/managed)
✅ Redis (ElastiCache/managed)
✅ S3 bucket (AWS/DigitalOcean)
✅ API servers (2+ instances, load balanced)
✅ Frontend (CDN)
✅ Worker nodes (Python/Manim)
✅ Monitoring (Sentry, Datadog)
✅ Logging (ELK/Loki)
✅ LLM providers (multiple)
```

**Estimated Cost:** $300-500/month

### Deployment Steps

See `DEPLOYMENT.md` for full guide.

**Quick Deploy to Vercel (Frontend only):**
```bash
cd web
vercel --prod
```

**Deploy API to DigitalOcean:**
```bash
# Build binary
cd api && cargo build --release

# Upload to server
scp target/release/animaforge-api user@server:/opt/animaforge/

# Run with systemd
sudo systemctl start animaforge-api
```

### Pre-Deployment Checklist

- [ ] All secrets in environment variables (not code)
- [ ] Database backups configured
- [ ] Monitoring enabled
- [ ] Logging configured
- [ ] Health checks added
- [ ] Rate limiting enabled
- [ ] HTTPS configured
- [ ] CORS properly set
- [ ] Error tracking (Sentry)
- [ ] Load tested
- [ ] Security audit done
- [ ] Documentation updated
- [ ] Runbooks written

---

## 👥 Contributing

### Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/your-feature-name

# 2. Make changes
# ... code ...

# 3. Test locally
./scripts/test-all.sh

# 4. Format code
cd cli && cargo fmt
cd api && cargo fmt
cd engine && black . && isort .
cd web && npm run format

# 5. Commit
git add .
git commit -m "feat: your feature description"

# 6. Push
git push origin feature/your-feature-name

# 7. Create PR on GitHub
```

### Code Style

**Rust:**
- Use `cargo fmt` (rustfmt)
- Use `cargo clippy` for linting
- Follow Rust API guidelines

**Python:**
- Use `black` for formatting
- Use `isort` for imports
- Use `mypy` for type checking
- Use `ruff` for linting

**TypeScript:**
- Use `prettier` for formatting
- Use `eslint` for linting
- Follow Airbnb style guide

### Commit Message Format

```
type(scope): subject

body

footer
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

**Examples:**
```
feat(api): add rate limiting middleware
fix(frontend): resolve SSR location error
docs(readme): update installation steps
test(engine): add validation tests
```

### Pull Request Process

1. Update documentation
2. Add tests for new features
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Request review
6. Address feedback
7. Squash commits if needed
8. Merge when approved

### Getting Help

**Questions?**
- Check this document first
- Check `QUICK_START.md`
- Check `USER_JOURNEY.md`
- Ask in GitHub Discussions
- Create an issue

**Found a bug?**
- Check "Known Issues" above
- Search existing issues
- Create new issue with:
  - Steps to reproduce
  - Expected behavior
  - Actual behavior
  - Environment details

---

## 📚 Additional Resources

### Documentation
- `README.md` - Project overview
- `QUICK_START.md` - 5-minute setup
- `USER_JOURNEY.md` - User experience flow
- `DEPLOYMENT.md` - Production deployment
- `TESTING_RESULTS.md` - Build test results
- `WILL_IT_WORK.md` - Production readiness assessment

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix Web Docs](https://actix.rs/)
- [Next.js Docs](https://nextjs.org/docs)
- [Manim Docs](https://docs.manim.community/)
- [PostgreSQL Manual](https://www.postgresql.org/docs/)

---

## 🎯 Priority Action Items

### Week 1: Critical Path
1. ⚠️ **Add integration tests** - Test full flow end-to-end
2. ⚠️ **Test LLM integration** - Actually call APIs
3. ⚠️ **Test Manim rendering** - Verify video generation
4. ⚠️ **Fix SSR warning** - Dashboard location issue
5. ⚠️ **Add error boundaries** - Frontend crash handling

### Week 2: Hardening
1. ⚠️ **Enable rate limiting** - Prevent API abuse
2. ⚠️ **Add health checks** - API monitoring
3. ⚠️ **Setup logging** - Structured logs
4. ⚠️ **Add validation** - Input sanitization
5. ⚠️ **Configure S3** - Real file storage

### Week 3: Testing
1. ⚠️ **Write unit tests** - All components
2. ⚠️ **Add E2E tests** - Critical user flows
3. ⚠️ **Load testing** - Performance verification
4. ⚠️ **Security audit** - OWASP top 10
5. ⚠️ **Coverage report** - Aim for 80%+

### Week 4: Production Prep
1. ⚠️ **Setup CI/CD** - Automated deployments
2. ⚠️ **Configure monitoring** - Sentry, metrics
3. ⚠️ **Add alerting** - Error notifications
4. ⚠️ **Write runbooks** - Incident response
5. ⚠️ **Beta test** - 10-50 users

---

## 📞 Support

**Internal Team:**
- Lead: [Your Name]
- Backend: [Backend Dev]
- Frontend: [Frontend Dev]
- DevOps: [DevOps Engineer]

**External:**
- GitHub Issues: For bugs and features
- Discussions: For questions
- Email: dev@animaforge.dev

---

**Last Updated:** 2025-11-19
**Next Review:** When hitting Week 4 milestones
**Maintained By:** Development Team

---

**Remember:** This is an MVP. It works, but it's not bulletproof. Test thoroughly before showing to real users.
