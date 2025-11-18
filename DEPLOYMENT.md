# AnimaForge Deployment Guide 🚀

Complete guide to deploying AnimaForge to production.

## Architecture Overview

```
User → Cloudflare CDN → Load Balancer → [Web, API, Workers]
                                              ↓
                                    [PostgreSQL, Redis, S3]
```

---

## Deployment Options

### Option 1: Docker Compose (Recommended for Testing)

**Best for:** Staging environments, local development teams

```bash
# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Stop all
docker-compose down
```

**Services included:**
- Web frontend (Next.js)
- API backend (Rust)
- PostgreSQL database
- Redis cache
- Meilisearch
- MinIO (S3-compatible storage)
- Worker (Python animation rendering)

---

### Option 2: Cloud Deployment (Production)

**Best for:** Production with scale

#### Infrastructure Stack

**Frontend (Vercel/Netlify):**
- Deploy Next.js to Vercel
- Automatic edge caching
- Global CDN
- Zero-config deployment

**Backend (DigitalOcean/AWS):**
- API on App Platform or EC2
- Auto-scaling enabled
- Load balancer configured

**Database (Managed PostgreSQL):**
- AWS RDS or DigitalOcean Managed Database
- Automated backups
- Read replicas for scale

**Storage (S3):**
- AWS S3 or DigitalOcean Spaces
- CloudFront CDN
- Lifecycle policies

**Cache (Redis):**
- AWS ElastiCache or DigitalOcean Redis
- 2GB+ memory

---

## Step-by-Step Cloud Deployment

### 1. Prepare Environment

**Create `.env.production` files:**

**api/.env.production:**
```env
DATABASE_URL=postgresql://user:pass@db.example.com/animaforge
REDIS_URL=redis://redis.example.com:6379
JWT_SECRET=your-super-secret-jwt-key-min-32-chars
S3_ENDPOINT=https://s3.amazonaws.com
S3_BUCKET=animaforge-production
S3_ACCESS_KEY=your-access-key
S3_SECRET_KEY=your-secret-key
RUST_LOG=info
PORT=8080
```

**web/.env.production:**
```env
NEXT_PUBLIC_API_URL=https://api.animaforge.dev/v1
NEXT_PUBLIC_WS_URL=wss://api.animaforge.dev
DATABASE_URL=postgresql://user:pass@db.example.com/animaforge
NEXTAUTH_SECRET=your-nextauth-secret-min-32-chars
NEXTAUTH_URL=https://animaforge.dev
```

---

### 2. Database Setup

**Create production database:**

```bash
# Connect to your managed database
psql $DATABASE_URL

# Run initialization
\i scripts/init-db.sql

# Optionally seed demo data
\i scripts/seed-data.sql
```

**Enable connection pooling:**
- Use PgBouncer or similar
- Max connections: 100
- Pool mode: Transaction

**Backup strategy:**
- Automated daily backups
- Point-in-time recovery enabled
- Backup retention: 30 days

---

### 3. Deploy Frontend (Vercel)

```bash
# Install Vercel CLI
npm i -g vercel

# Login
vercel login

# Deploy
cd web
vercel --prod

# Set environment variables in Vercel dashboard
# Or via CLI:
vercel env add NEXT_PUBLIC_API_URL production
```

**Vercel configuration** (`vercel.json`):
```json
{
  "build": {
    "env": {
      "NEXT_PUBLIC_API_URL": "@api-url"
    }
  },
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "X-Frame-Options",
          "value": "DENY"
        },
        {
          "key": "X-Content-Type-Options",
          "value": "nosniff"
        }
      ]
    }
  ]
}
```

---

### 4. Deploy API (DigitalOcean App Platform)

**Create `app.yaml`:**
```yaml
name: animaforge-api
services:
  - name: api
    github:
      repo: yourusername/animaforge
      branch: main
      deploy_on_push: true
    source_dir: /api
    build_command: cargo build --release
    run_command: ./target/release/animaforge-api
    envs:
      - key: DATABASE_URL
        scope: RUN_TIME
        value: ${db.DATABASE_URL}
      - key: REDIS_URL
        scope: RUN_TIME
        value: ${redis.REDIS_URL}
      - key: JWT_SECRET
        scope: RUN_TIME
        type: SECRET
    health_check:
      http_path: /health
    instance_count: 2
    instance_size_slug: professional-xs
databases:
  - name: db
    engine: PG
    version: "15"
    size: db-s-1vcpu-1gb
```

**Deploy:**
```bash
doctl apps create --spec app.yaml
```

---

### 5. Deploy Workers (Python Rendering)

**Create worker service:**

**Dockerfile.worker:**
```dockerfile
FROM python:3.11-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    texlive \
    texlive-latex-extra \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY engine/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy engine code
COPY engine /app/engine
WORKDIR /app/engine

# Install engine
RUN pip install -e .

# Run worker
CMD ["python", "-m", "animaforge_worker"]
```

**Deploy to Kubernetes or DigitalOcean:**
```bash
docker build -f Dockerfile.worker -t animaforge-worker .
docker push registry.digitalocean.com/animaforge/worker:latest

# Deploy with kubectl or DO App Platform
```

---

### 6. Configure CDN (Cloudflare)

**Settings:**
- SSL/TLS: Full (strict)
- Always Use HTTPS: On
- Auto Minify: HTML, CSS, JS
- Brotli compression: On
- Caching level: Standard

**Page Rules:**
```
api.animaforge.dev/*
  - Cache Level: Bypass

animaforge.dev/*
  - Cache Level: Cache Everything
  - Edge Cache TTL: 1 day

animaforge.dev/animations/*
  - Cache Level: Cache Everything
  - Edge Cache TTL: 1 month
```

---

### 7. Monitoring Setup

**Application Monitoring (Sentry):**

```bash
# Install Sentry SDK
cd api
cargo add sentry

# Configure in main.rs
let _guard = sentry::init(("YOUR_DSN", sentry::ClientOptions {
    release: sentry::release_name!(),
    ..Default::default()
}));
```

**Infrastructure Monitoring (Datadog/New Relic):**

```yaml
# datadog-agent.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: datadog-agent
data:
  api_key: YOUR_API_KEY
  app_key: YOUR_APP_KEY
```

**Uptime Monitoring (UptimeRobot):**
- Monitor: https://animaforge.dev
- Monitor: https://api.animaforge.dev/health
- Interval: 5 minutes
- Alert: Email, Slack, SMS

---

### 8. SSL Certificates

**Let's Encrypt (Free):**

```bash
# Install certbot
sudo apt-get install certbot

# Generate certificate
sudo certbot certonly --standalone -d api.animaforge.dev

# Auto-renewal cron
0 0 * * * certbot renew --quiet
```

**Or use Cloudflare SSL:**
- Automatic, free, with CDN

---

### 9. CI/CD Pipeline

**GitHub Actions** (`.github/workflows/deploy.yml`):

```yaml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy-frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: cd web && npm ci
      - run: cd web && npm run build
      - uses: amondnet/vercel-action@v25
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.ORG_ID }}
          vercel-project-id: ${{ secrets.PROJECT_ID }}
          vercel-args: '--prod'

  deploy-api:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cd api && cargo build --release
      - run: docker build -t animaforge-api .
      - run: docker push ${{ secrets.REGISTRY }}/animaforge-api:latest
      - run: kubectl rollout restart deployment/api

  run-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: cargo test --all
      - run: cd engine && pytest
      - run: cd web && npm test
```

---

### 10. Security Checklist

- [ ] HTTPS everywhere (redirect HTTP to HTTPS)
- [ ] JWT secret is strong (32+ random characters)
- [ ] Database passwords are strong
- [ ] API rate limiting enabled
- [ ] CORS properly configured
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS protection (Content Security Policy)
- [ ] CSRF tokens for state changes
- [ ] File upload validation
- [ ] Secrets in environment variables (not code)
- [ ] Database backups automated
- [ ] Error messages don't leak sensitive info
- [ ] Security headers configured
- [ ] Dependency updates automated (Dependabot)

---

### 11. Performance Optimization

**Frontend:**
- [ ] Enable Next.js Image optimization
- [ ] Lazy load animations
- [ ] Code splitting
- [ ] Prefetch critical resources
- [ ] Service worker for offline support

**API:**
- [ ] Database query optimization
- [ ] Redis caching for hot data
- [ ] Connection pooling
- [ ] Gzip compression
- [ ] API response pagination

**Database:**
- [ ] Indexes on frequently queried columns
- [ ] Query plan analysis
- [ ] Vacuum and analyze regularly
- [ ] Read replicas for scaling

---

### 12. Scaling Strategy

**Horizontal Scaling:**
```
1-1000 users:     1 API instance, 1 DB
1000-10000 users: 3 API instances, 1 DB with replicas
10000+ users:     Auto-scaling, DB cluster, CDN
```

**Cost Estimates:**

| Users | Monthly Cost |
|-------|--------------|
| 0-100 | $25 (hobby) |
| 100-1K | $100 |
| 1K-10K | $500 |
| 10K+ | $2000+ |

---

### 13. Disaster Recovery

**Backup Strategy:**
- Database: Daily automated backups, 30-day retention
- S3 animations: Versioning enabled, cross-region replication
- Config: Version control in Git

**Recovery Plan:**
1. Database corruption: Restore from latest backup (RTO: 1 hour)
2. Server failure: Auto-scaling spins up new instance (RTO: 5 minutes)
3. Complete outage: Restore from backups to new infrastructure (RTO: 4 hours)

**Runbook:** Document in `docs/runbook.md`

---

### 14. Post-Deployment

**Health Checks:**
```bash
# API health
curl https://api.animaforge.dev/health

# Database connectivity
curl https://api.animaforge.dev/health/db

# Frontend
curl -I https://animaforge.dev
```

**Load Testing:**
```bash
# Install k6
brew install k6

# Run load test
k6 run tests/load-test.js
```

**Smoke Tests:**
```bash
# Test critical paths
./tests/smoke-test.sh production
```

---

## Rollback Procedure

If deployment fails:

```bash
# Frontend (Vercel)
vercel rollback

# API (DigitalOcean)
doctl apps deployment rollback <app-id>

# Database (restore backup)
pg_restore -d animaforge latest_backup.dump
```

---

## Support & Maintenance

**Weekly Tasks:**
- Review error logs
- Check performance metrics
- Update dependencies

**Monthly Tasks:**
- Review security vulnerabilities
- Optimize database
- Review and archive old data
- Cost optimization review

**Quarterly Tasks:**
- Load testing
- Disaster recovery drill
- Infrastructure review
- Security audit

---

## Cost Optimization

**Tips:**
1. Use CDN for static assets (reduce server load)
2. Compress images before upload
3. Cache API responses
4. Use spot instances for workers
5. Clean up old animations (lifecycle policy)
6. Monitor and alert on cost spikes

---

## Additional Resources

- [Next.js Deployment Docs](https://nextjs.org/docs/deployment)
- [Rust Web Production Guide](https://rocket.rs/v0.5-rc/guide/deployment/)
- [PostgreSQL Production Checklist](https://www.postgresql.org/docs/current/runtime-config.html)
- [DigitalOcean App Platform](https://docs.digitalocean.com/products/app-platform/)

---

**Your production AnimaForge deployment is ready!** 🎉

For questions: devops@animaforge.dev
