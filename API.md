# AnimaForge API Documentation 🔌

> **RESTful API for animation generation and marketplace operations**

## Base URL
```
Production:  https://api.animaforge.dev/v1
Staging:     https://staging-api.animaforge.dev/v1
Development: http://localhost:8080/v1
```

## Authentication

### API Keys
```bash
# Include in Authorization header
Authorization: Bearer YOUR_API_KEY

# Example
curl -H "Authorization: Bearer af_live_abc123..." \
  https://api.animaforge.dev/v1/animations
```

### Getting an API Key
```bash
# Via CLI
animaforge auth login
animaforge config get api_key

# Via Web Dashboard
https://animaforge.dev/dashboard/settings/api-keys
```

---

## Rate Limits

| Tier | Requests/Hour | Burst |
|------|--------------|-------|
| Free | 100 | 10 |
| Pro | 1,000 | 50 |
| Enterprise | 10,000 | 200 |

**Headers:**
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1677886800
```

---

## Response Format

### Success Response
```json
{
  "success": true,
  "data": {
    "id": "anim_123",
    "title": "Rotating Cube"
  },
  "meta": {
    "timestamp": "2025-01-15T10:30:00Z",
    "request_id": "req_abc123"
  }
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid animation duration",
    "details": {
      "field": "duration",
      "constraint": "must be between 1 and 60"
    }
  },
  "meta": {
    "timestamp": "2025-01-15T10:30:00Z",
    "request_id": "req_abc123"
  }
}
```

### Error Codes
| Code | HTTP Status | Description |
|------|-------------|-------------|
| `AUTHENTICATION_ERROR` | 401 | Invalid or missing API key |
| `AUTHORIZATION_ERROR` | 403 | Insufficient permissions |
| `VALIDATION_ERROR` | 400 | Invalid request data |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Endpoints

## Authentication

### POST /auth/register
Register a new user

**Request:**
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePass123!"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "user_123",
      "username": "johndoe",
      "email": "john@example.com",
      "created_at": "2025-01-15T10:30:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "refresh_token": "refresh_abc123..."
  }
}
```

---

### POST /auth/login
Login existing user

**Request:**
```json
{
  "email": "john@example.com",
  "password": "SecurePass123!"
}
```

**Response:** Same as register

---

### POST /auth/refresh
Refresh access token

**Request:**
```json
{
  "refresh_token": "refresh_abc123..."
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_at": "2025-01-15T12:30:00Z"
  }
}
```

---

### GET /auth/me
Get current user info

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Response:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "user_123",
      "username": "johndoe",
      "email": "john@example.com",
      "display_name": "John Doe",
      "avatar_url": "https://cdn.animaforge.dev/avatars/user_123.png",
      "bio": "Animation enthusiast",
      "tier": "pro",
      "stats": {
        "animations_created": 42,
        "total_downloads": 1250,
        "followers": 89
      },
      "created_at": "2025-01-01T00:00:00Z"
    }
  }
}
```

---

## Animations

### POST /animations/generate
Generate animation from prompt

**Request:**
```json
{
  "prompt": "A blue sphere rotating and morphing into a cube",
  "config": {
    "quality": "high",
    "duration": 5,
    "fps": 60,
    "background": "#1a1a1a",
    "resolution": "1080p"
  },
  "backend": "gemini",
  "template": null
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "job_id": "job_abc123",
    "status": "queued",
    "estimated_time": 30,
    "webhook_url": "https://api.animaforge.dev/v1/webhooks/jobs/job_abc123"
  }
}
```

**Webhook Payload (when complete):**
```json
{
  "job_id": "job_abc123",
  "status": "completed",
  "animation": {
    "id": "anim_123",
    "url": "https://cdn.animaforge.dev/animations/anim_123.mp4",
    "thumbnail_url": "https://cdn.animaforge.dev/thumbnails/anim_123.jpg",
    "source_code": "from manim import *\n\nclass MyScene(Scene):\n    ...",
    "metadata": {
      "duration": 5.0,
      "fps": 60,
      "resolution": "1920x1080",
      "file_size": 2048576
    }
  }
}
```

---

### GET /animations/jobs/{job_id}
Check generation job status

**Response:**
```json
{
  "success": true,
  "data": {
    "job_id": "job_abc123",
    "status": "processing",
    "progress": 65,
    "estimated_remaining": 10,
    "logs": [
      "Starting generation...",
      "LLM processing complete",
      "Rendering frame 390/600"
    ]
  }
}
```

**Status values:**
- `queued` - Waiting in queue
- `processing` - Currently generating
- `completed` - Successfully completed
- `failed` - Generation failed
- `cancelled` - User cancelled

---

### GET /animations
List animations

**Query Parameters:**
```
?page=1
&per_page=20
&sort=created_at
&order=desc
&category=math
&license=MIT
&free=true
&search=physics
```

**Response:**
```json
{
  "success": true,
  "data": {
    "animations": [
      {
        "id": "anim_123",
        "title": "Pythagorean Theorem Proof",
        "description": "Visual proof using colored squares",
        "thumbnail_url": "https://cdn.animaforge.dev/thumbnails/anim_123.jpg",
        "creator": {
          "id": "user_456",
          "username": "mathteacher",
          "avatar_url": "..."
        },
        "stats": {
          "views": 1250,
          "downloads": 89,
          "likes": 42
        },
        "duration": 8.5,
        "resolution": "1080p",
        "license": "MIT",
        "price": 0.00,
        "tags": ["math", "geometry", "education"],
        "created_at": "2025-01-10T15:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 156,
      "pages": 8
    }
  }
}
```

---

### GET /animations/{id}
Get animation details

**Response:**
```json
{
  "success": true,
  "data": {
    "animation": {
      "id": "anim_123",
      "title": "Pythagorean Theorem Proof",
      "description": "Visual proof using colored squares...",
      "video_url": "https://cdn.animaforge.dev/animations/anim_123.mp4",
      "thumbnail_url": "https://cdn.animaforge.dev/thumbnails/anim_123.jpg",
      "creator": {
        "id": "user_456",
        "username": "mathteacher",
        "display_name": "Math Teacher",
        "avatar_url": "...",
        "bio": "Passionate about math education"
      },
      "stats": {
        "views": 1250,
        "downloads": 89,
        "likes": 42,
        "comments": 7
      },
      "metadata": {
        "duration": 8.5,
        "fps": 60,
        "resolution": "1920x1080",
        "file_size": 4096000,
        "format": "mp4"
      },
      "license": "MIT",
      "price": 0.00,
      "tags": ["math", "geometry", "education"],
      "versions": [
        {
          "version": 2,
          "created_at": "2025-01-12T10:00:00Z",
          "changelog": "Fixed color contrast"
        },
        {
          "version": 1,
          "created_at": "2025-01-10T15:00:00Z",
          "changelog": "Initial release"
        }
      ],
      "source_code_url": "https://api.animaforge.dev/v1/animations/anim_123/source",
      "created_at": "2025-01-10T15:00:00Z",
      "updated_at": "2025-01-12T10:00:00Z"
    }
  }
}
```

---

### POST /animations
Upload new animation

**Request (multipart/form-data):**
```
POST /v1/animations
Content-Type: multipart/form-data

--boundary
Content-Disposition: form-data; name="file"; filename="animation.mp4"
Content-Type: video/mp4

[binary data]
--boundary
Content-Disposition: form-data; name="metadata"

{
  "title": "My Animation",
  "description": "Cool animation",
  "tags": ["cool", "animation"],
  "license": "MIT",
  "price": 0.00
}
--boundary--
```

**Response:**
```json
{
  "success": true,
  "data": {
    "animation": {
      "id": "anim_789",
      "status": "processing",
      "message": "Animation is being processed. Thumbnail generation in progress."
    }
  }
}
```

---

### PUT /animations/{id}
Update animation

**Request:**
```json
{
  "title": "Updated Title",
  "description": "Updated description",
  "tags": ["updated", "tags"],
  "price": 9.99
}
```

**Response:** Returns updated animation object

---

### DELETE /animations/{id}
Delete animation

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Animation deleted successfully"
  }
}
```

---

### GET /animations/{id}/download
Download animation

**Headers:**
```
Authorization: Bearer YOUR_TOKEN
```

**Response:**
- Redirects to signed CDN URL
- File download begins immediately

---

### POST /animations/{id}/like
Like an animation

**Response:**
```json
{
  "success": true,
  "data": {
    "liked": true,
    "like_count": 43
  }
}
```

---

### GET /animations/{id}/source
Get source code

**Response:**
```json
{
  "success": true,
  "data": {
    "source_code": "from manim import *\n\nclass MyScene(Scene):\n    def construct(self):\n        circle = Circle()\n        self.play(Create(circle))\n        self.wait()",
    "language": "python",
    "framework": "manim-community",
    "version": "0.17.3"
  }
}
```

---

## Search

### GET /search
Search animations

**Query Parameters:**
```
?q=physics simulation
&category=science
&min_duration=5
&max_duration=15
&license=MIT
&free=true
&sort=relevance
&page=1
&per_page=20
```

**Response:**
```json
{
  "success": true,
  "data": {
    "results": [
      {
        "id": "anim_123",
        "title": "Physics Simulation",
        "score": 0.95,
        "highlights": {
          "title": "Physics <em>Simulation</em>",
          "description": "Realistic <em>physics</em> engine..."
        },
        ...
      }
    ],
    "facets": {
      "categories": {
        "science": 42,
        "education": 38,
        "entertainment": 12
      },
      "licenses": {
        "MIT": 67,
        "CC-BY": 25
      }
    },
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 89,
      "pages": 5
    }
  }
}
```

---

### GET /search/autocomplete
Autocomplete suggestions

**Query Parameters:**
```
?q=phys
&limit=5
```

**Response:**
```json
{
  "success": true,
  "data": {
    "suggestions": [
      "physics simulation",
      "physics engine",
      "physics visualization",
      "physical chemistry",
      "physiological processes"
    ]
  }
}
```

---

## Users

### GET /users/{id}
Get user profile

**Response:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "user_123",
      "username": "johndoe",
      "display_name": "John Doe",
      "bio": "Animation creator and educator",
      "avatar_url": "...",
      "stats": {
        "animations": 42,
        "downloads": 1250,
        "likes_received": 567,
        "followers": 89,
        "following": 34
      },
      "badges": [
        "early_adopter",
        "top_creator"
      ],
      "joined_at": "2025-01-01T00:00:00Z"
    }
  }
}
```

---

### GET /users/{id}/animations
Get user's animations

**Response:** Same format as GET /animations

---

### POST /users/{id}/follow
Follow a user

**Response:**
```json
{
  "success": true,
  "data": {
    "following": true,
    "follower_count": 90
  }
}
```

---

### DELETE /users/{id}/follow
Unfollow a user

**Response:**
```json
{
  "success": true,
  "data": {
    "following": false,
    "follower_count": 89
  }
}
```

---

## Marketplace

### GET /marketplace/featured
Get featured animations

**Response:**
```json
{
  "success": true,
  "data": {
    "featured": [
      // Array of animation objects
    ]
  }
}
```

---

### GET /marketplace/trending
Get trending animations

**Query Parameters:**
```
?timeframe=week  # day|week|month|all
&limit=10
```

**Response:** Similar to featured

---

### POST /marketplace/purchase
Purchase an animation

**Request:**
```json
{
  "animation_id": "anim_123",
  "payment_method": "card_abc123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "purchase": {
      "id": "purchase_789",
      "animation_id": "anim_123",
      "amount": 9.99,
      "currency": "USD",
      "status": "completed",
      "receipt_url": "https://animaforge.dev/receipts/purchase_789",
      "created_at": "2025-01-15T10:30:00Z"
    }
  }
}
```

---

### GET /marketplace/purchases
Get user's purchases

**Response:**
```json
{
  "success": true,
  "data": {
    "purchases": [
      {
        "id": "purchase_789",
        "animation": {
          "id": "anim_123",
          "title": "Cool Animation",
          ...
        },
        "amount": 9.99,
        "created_at": "2025-01-15T10:30:00Z"
      }
    ]
  }
}
```

---

## Analytics

### GET /analytics/dashboard
Get creator dashboard stats

**Query Parameters:**
```
?start_date=2025-01-01
&end_date=2025-01-31
```

**Response:**
```json
{
  "success": true,
  "data": {
    "overview": {
      "total_views": 5420,
      "total_downloads": 234,
      "total_revenue": 156.75,
      "total_likes": 89
    },
    "top_animations": [
      {
        "animation_id": "anim_123",
        "title": "Popular Animation",
        "views": 1250,
        "downloads": 89
      }
    ],
    "timeline": [
      {
        "date": "2025-01-15",
        "views": 142,
        "downloads": 8,
        "revenue": 5.99
      }
    ]
  }
}
```

---

### GET /analytics/animations/{id}
Get animation-specific analytics

**Response:**
```json
{
  "success": true,
  "data": {
    "animation_id": "anim_123",
    "metrics": {
      "total_views": 1250,
      "unique_viewers": 890,
      "total_downloads": 89,
      "likes": 42,
      "shares": 15
    },
    "demographics": {
      "countries": {
        "US": 450,
        "UK": 230,
        "IN": 180
      }
    },
    "referrers": {
      "direct": 560,
      "google": 320,
      "twitter": 180
    },
    "timeline": [
      // Daily stats
    ]
  }
}
```

---

## Webhooks

### POST /webhooks
Register a webhook

**Request:**
```json
{
  "url": "https://your-server.com/webhook",
  "events": [
    "animation.generated",
    "animation.purchased",
    "user.followed"
  ],
  "secret": "your_webhook_secret"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "webhook": {
      "id": "webhook_123",
      "url": "https://your-server.com/webhook",
      "events": ["animation.generated", ...],
      "status": "active",
      "created_at": "2025-01-15T10:30:00Z"
    }
  }
}
```

**Event Payload:**
```json
{
  "event": "animation.generated",
  "data": {
    "animation_id": "anim_123",
    "job_id": "job_abc123",
    "status": "completed"
  },
  "timestamp": "2025-01-15T10:30:00Z",
  "signature": "sha256=..."
}
```

---

## SDKs

### Python
```python
from animaforge import AnimaForge

client = AnimaForge(api_key="your_api_key")

# Generate animation
job = client.animations.generate(
    prompt="rotating cube",
    quality="high"
)

# Wait for completion
animation = job.wait()
print(animation.url)

# Search marketplace
results = client.search("physics simulation")
```

### JavaScript/TypeScript
```typescript
import { AnimaForge } from '@animaforge/sdk';

const client = new AnimaForge({ apiKey: 'your_api_key' });

// Generate animation
const job = await client.animations.generate({
  prompt: 'rotating cube',
  quality: 'high'
});

// Subscribe to progress
job.on('progress', (progress) => {
  console.log(`${progress}% complete`);
});

const animation = await job.waitForCompletion();
console.log(animation.url);
```

### Rust
```rust
use animaforge::AnimaForge;

let client = AnimaForge::new("your_api_key");

// Generate animation
let job = client.animations().generate(
    "rotating cube",
    GenerateConfig::default().quality(Quality::High)
).await?;

// Wait for completion
let animation = job.wait().await?;
println!("URL: {}", animation.url);
```

---

## Examples

### Complete Workflow
```bash
# 1. Register & Login
curl -X POST https://api.animaforge.dev/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "email": "user@example.com",
    "password": "SecurePass123!"
  }'

# 2. Generate Animation
curl -X POST https://api.animaforge.dev/v1/animations/generate \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "A blue sphere rotating",
    "config": {"quality": "high"}
  }'

# 3. Check Status
curl https://api.animaforge.dev/v1/animations/jobs/job_abc123 \
  -H "Authorization: Bearer YOUR_TOKEN"

# 4. Download Result
curl https://api.animaforge.dev/v1/animations/anim_123/download \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -o animation.mp4

# 5. Publish to Marketplace
curl -X POST https://api.animaforge.dev/v1/animations \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@animation.mp4" \
  -F 'metadata={"title":"My Animation","tags":["cool"]}'
```

---

## Support

- 📖 [Full API Docs](https://docs.animaforge.dev/api)
- 💬 [Discord Community](https://discord.gg/animaforge)
- 📧 [Email Support](mailto:api@animaforge.dev)
- 🐛 [Report Issues](https://github.com/yourusername/animaforge/issues)

---

**API Version:** v1  
**Last Updated:** 2025  
**Status:** [https://status.animaforge.dev](https://status.animaforge.dev)
