# AnimaForge API Documentation

Complete API documentation for the AnimaForge backend.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

Most endpoints require authentication using JWT tokens. Include the token in the `Authorization` header:

```
Authorization: Bearer <your-jwt-token>
```

Tokens are obtained by logging in and are valid for 24 hours by default.

---

## Endpoints

### Health Check

#### GET /health

Check if the API is running.

**Authentication:** Not required

**Response:**
```json
{
  "status": "healthy",
  "service": "AnimaForge API",
  "version": "0.1.0"
}
```

---

## Authentication Endpoints

### Register User

#### POST /auth/register

Create a new user account.

**Authentication:** Not required

**Request Body:**
```json
{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Validation:**
- `username`: 3-50 characters
- `email`: Valid email format
- `password`: Minimum 8 characters

**Success Response (201):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

**Error Response (409):**
```json
{
  "error": "CONFLICT",
  "message": "User with this email or username already exists"
}
```

---

### Login

#### POST /auth/login

Authenticate a user and receive a JWT token.

**Authentication:** Not required

**Request Body:**
```json
{
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Success Response (200):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

**Error Response (401):**
```json
{
  "error": "UNAUTHORIZED",
  "message": "Invalid email or password"
}
```

---

### Logout

#### POST /auth/logout

Logout the current user (client-side token removal).

**Authentication:** Not required (but recommended to send token)

**Success Response (200):**
```json
{
  "message": "Logged out successfully"
}
```

---

### Get Current User

#### GET /auth/me

Get the currently authenticated user's information.

**Authentication:** Required

**Success Response (200):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Response (401):**
```json
{
  "error": "UNAUTHORIZED",
  "message": "No authentication token provided"
}
```

---

## Animation Endpoints

### List Animations

#### GET /animations

Get a paginated list of all animations.

**Authentication:** Not required

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Example:** `/animations?page=1&limit=10`

**Success Response (200):**
```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "user_id": "660e8400-e29b-41d4-a716-446655440000",
      "title": "Cool Animation",
      "description": "A really cool animation",
      "file_url": "https://storage.example.com/animations/cool.mp4",
      "thumbnail_url": "https://storage.example.com/thumbnails/cool.jpg",
      "views": 150,
      "likes": 25,
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    }
  ],
  "page": 1,
  "limit": 10,
  "total": 42
}
```

---

### Create Animation

#### POST /animations

Create a new animation.

**Authentication:** Required

**Request Body:**
```json
{
  "title": "My New Animation",
  "description": "Description of my animation",
  "file_url": "https://storage.example.com/animations/my-animation.mp4",
  "thumbnail_url": "https://storage.example.com/thumbnails/my-animation.jpg"
}
```

**Validation:**
- `title`: 1-200 characters (required)
- `description`: Max 1000 characters (optional)
- `file_url`: Required
- `thumbnail_url`: Optional

**Success Response (201):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440000",
  "title": "My New Animation",
  "description": "Description of my animation",
  "file_url": "https://storage.example.com/animations/my-animation.mp4",
  "thumbnail_url": "https://storage.example.com/thumbnails/my-animation.jpg",
  "views": 0,
  "likes": 0,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

---

### Get Animation

#### GET /animations/:id

Get details of a specific animation. This also increments the view count.

**Authentication:** Not required

**Success Response (200):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440000",
  "title": "Cool Animation",
  "description": "A really cool animation",
  "file_url": "https://storage.example.com/animations/cool.mp4",
  "thumbnail_url": "https://storage.example.com/thumbnails/cool.jpg",
  "views": 151,
  "likes": 25,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

**Error Response (404):**
```json
{
  "error": "NOT_FOUND",
  "message": "Animation not found"
}
```

---

### Update Animation

#### PUT /animations/:id

Update an animation. Only the owner can update.

**Authentication:** Required

**Request Body:**
```json
{
  "title": "Updated Title",
  "description": "Updated description",
  "thumbnail_url": "https://storage.example.com/thumbnails/updated.jpg"
}
```

All fields are optional.

**Success Response (200):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "660e8400-e29b-41d4-a716-446655440000",
  "title": "Updated Title",
  "description": "Updated description",
  "file_url": "https://storage.example.com/animations/cool.mp4",
  "thumbnail_url": "https://storage.example.com/thumbnails/updated.jpg",
  "views": 151,
  "likes": 25,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T14:45:00Z"
}
```

**Error Response (401):**
```json
{
  "error": "UNAUTHORIZED",
  "message": "You don't have permission to update this animation"
}
```

---

### Delete Animation

#### DELETE /animations/:id

Delete an animation. Only the owner can delete.

**Authentication:** Required

**Success Response (200):**
```json
{
  "message": "Animation deleted successfully"
}
```

**Error Response (401):**
```json
{
  "error": "UNAUTHORIZED",
  "message": "You don't have permission to delete this animation"
}
```

---

### Get Featured Animations

#### GET /animations/featured

Get featured animations (animations with more than 10 likes).

**Authentication:** Not required

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Success Response (200):**
Same format as List Animations.

---

### Get Trending Animations

#### GET /animations/trending

Get trending animations (recent animations with high views from the last 7 days).

**Authentication:** Not required

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Success Response (200):**
Same format as List Animations.

---

## User Endpoints

### Get User Profile

#### GET /users/:id/profile

Get a user's public profile.

**Authentication:** Not required

**Success Response (200):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Response (404):**
```json
{
  "error": "NOT_FOUND",
  "message": "User not found"
}
```

---

### Get User's Animations

#### GET /users/:id/animations

Get all animations created by a specific user.

**Authentication:** Not required

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Success Response (200):**
Same format as List Animations.

---

### Update User Profile

#### PUT /users/:id/profile

Update the user's profile. Users can only update their own profile.

**Authentication:** Required

**Request Body:**
```json
{
  "username": "new_username",
  "email": "newemail@example.com"
}
```

All fields are optional.

**Success Response (200):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "new_username",
  "email": "newemail@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
```

**Error Response (401):**
```json
{
  "error": "UNAUTHORIZED",
  "message": "You can only update your own profile"
}
```

---

## Search Endpoints

### Search Animations

#### GET /search

Search for animations by title or description.

**Authentication:** Not required

**Query Parameters:**
- `q` (required): Search query
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)

**Example:** `/search?q=cool&page=1&limit=10`

**Success Response (200):**
Same format as List Animations.

**Error Response (400):**
```json
{
  "error": "BAD_REQUEST",
  "message": "Search query cannot be empty"
}
```

---

## Error Responses

All error responses follow this format:

```json
{
  "error": "ERROR_TYPE",
  "message": "Detailed error message"
}
```

### Error Types

- `BAD_REQUEST` (400): Invalid request data
- `UNAUTHORIZED` (401): Authentication required or failed
- `NOT_FOUND` (404): Resource not found
- `CONFLICT` (409): Resource conflict (e.g., duplicate email)
- `VALIDATION_ERROR` (422): Request validation failed
- `INTERNAL_SERVER_ERROR` (500): Server error

---

## Rate Limiting

Currently, there are no rate limits implemented. This may change in production.

## CORS

The API is configured to accept requests from `http://localhost:3000` by default. This can be configured via the `CORS_ALLOWED_ORIGIN` environment variable.

## Pagination

All list endpoints support pagination with the following parameters:
- `page`: Page number (starts at 1)
- `limit`: Number of items per page (default: 20)

Responses include:
- `data`: Array of items
- `page`: Current page
- `limit`: Items per page
- `total`: Total number of items
