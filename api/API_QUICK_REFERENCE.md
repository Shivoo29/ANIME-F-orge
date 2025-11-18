# AnimaForge API - Quick Reference

## Base URL
```
http://localhost:8080/api/v1
```

## Authentication
All protected endpoints require a JWT token in the Authorization header:
```
Authorization: Bearer <your_jwt_token>
```

## Endpoints Summary

### 🔐 Authentication
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/auth/register` | ❌ | Register new user |
| POST | `/auth/login` | ❌ | Login user |
| GET | `/auth/me` | ✅ | Get current user |

### 🎬 Animations
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/animations` | ❌ | List all public animations |
| POST | `/animations` | ✅ | Create new animation |
| GET | `/animations/:id` | ❌ | Get animation by ID |
| PUT | `/animations/:id` | ✅ | Update animation (owner only) |
| DELETE | `/animations/:id` | ✅ | Delete animation (owner only) |
| GET | `/animations/:id/download` | ❌ | Download animation file |

### 👤 Users
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/users/:id/profile` | ❌ | Get user profile |
| GET | `/users/:id/animations` | ❌ | Get user's animations |
| PUT | `/users/:id/profile` | ✅ | Update profile |

### 🏪 Marketplace
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/marketplace/featured` | ❌ | Get featured animations |
| GET | `/marketplace/trending` | ❌ | Get trending animations |
| GET | `/marketplace/search?q=...` | ❌ | Search animations |

### 🔍 Search
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/search?q=...` | ❌ | Search animations |

### ❤️ Health
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/health` | ❌ | Health check |

## Common Query Parameters

### Pagination
- `page` (default: 1) - Page number
- `limit` (default: 20) - Items per page

### Search
- `q` (required) - Search query string

## Example Requests

### Register User
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepass123"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepass123"
  }'
```

### Get Current User
```bash
curl http://localhost:8080/api/v1/auth/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### List Animations
```bash
curl "http://localhost:8080/api/v1/animations?page=1&limit=10"
```

### Create Animation
```bash
curl -X POST http://localhost:8080/api/v1/animations \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My Animation",
    "description": "A cool animation",
    "file_url": "https://example.com/animation.mp4",
    "thumbnail_url": "https://example.com/thumb.jpg",
    "duration": 10.5,
    "is_public": true
  }'
```

### Get Animation
```bash
curl http://localhost:8080/api/v1/animations/{animation_id}
```

### Update Animation
```bash
curl -X PUT http://localhost:8080/api/v1/animations/{animation_id} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "description": "Updated description"
  }'
```

### Delete Animation
```bash
curl -X DELETE http://localhost:8080/api/v1/animations/{animation_id} \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Download Animation
```bash
curl http://localhost:8080/api/v1/animations/{animation_id}/download
```

### Get Featured Animations
```bash
curl "http://localhost:8080/api/v1/marketplace/featured?page=1&limit=20"
```

### Get Trending Animations
```bash
curl "http://localhost:8080/api/v1/marketplace/trending?page=1&limit=20"
```

### Search Animations
```bash
curl "http://localhost:8080/api/v1/marketplace/search?q=cool&page=1&limit=10"
```

### Get User Profile
```bash
curl http://localhost:8080/api/v1/users/{user_id}/profile
```

### Get User Animations
```bash
curl "http://localhost:8080/api/v1/users/{user_id}/animations?page=1&limit=10"
```

### Health Check
```bash
curl http://localhost:8080/api/v1/health
```

## Response Format

### Success Response
```json
{
  "data": { /* response data */ }
}
```

### Paginated Response
```json
{
  "data": [ /* array of items */ ],
  "page": 1,
  "limit": 20,
  "total": 100
}
```

### Error Response
```json
{
  "error": "Error message",
  "status": 400
}
```

## HTTP Status Codes
- `200` - Success
- `201` - Created
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict
- `422` - Validation Error
- `500` - Server Error
