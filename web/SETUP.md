# Quick Setup Guide

## Prerequisites
- Node.js 18+ installed
- Backend API running on `http://localhost:8000`

## Installation Steps

### 1. Install Dependencies
```bash
cd /home/user/ANIME-F-orge/web
npm install
```

### 2. Configure Environment
```bash
cp .env.example .env
```

The default configuration points to `http://localhost:8000/api`. Update if needed.

### 3. Start Development Server
```bash
npm run dev
```

The application will be available at: **http://localhost:3000**

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Create production build
- `npm start` - Start production server
- `npm run lint` - Run ESLint

## First Steps

1. Visit **http://localhost:3000** to see the landing page
2. Click "Sign Up" to create an account
3. After registration, you'll be redirected to the dashboard
4. Click "Create New Animation" to generate your first animation
5. Browse the marketplace to see community animations

## Key Features to Test

### Landing Page (`/`)
- Hero section with clear value proposition
- "How It Works" section explaining the 3-step process
- Features showcase
- Sample animations grid
- Multiple CTAs to sign up

### Browse Page (`/browse`)
- Search functionality
- Category filters (All, Anime, Fantasy, Sci-Fi, Action, Nature, Abstract)
- Animation grid with cards
- Load more button

### Authentication
- Login page (`/login`) with email/password
- Register page (`/register`) with username, email, password
- Form validation with clear error messages
- Redirects to dashboard after successful auth

### Dashboard (`/dashboard`)
- Welcome message with username
- Stats overview (animations, views, likes)
- Quick actions to create or browse
- User's animations grid
- Create animation modal with prompt input

### Animation Detail (`/animation/[id]`)
- Video player with controls
- Animation information and description
- Technical details (duration, resolution, FPS)
- Creator profile info
- Download button
- Like/share buttons
- Related animations

## Design System

The application uses a **neo-brutalism** design with:
- **Primary Color**: #FF6B35 (Orange)
- **Secondary Color**: #004E89 (Blue)
- **Accent Color**: #F7B801 (Yellow)
- **Borders**: 4px thick black borders
- **Shadows**: 8px offset for depth
- **Typography**: Bold, uppercase headings

## Troubleshooting

### Port Already in Use
If port 3000 is already in use:
```bash
PORT=3001 npm run dev
```

### API Connection Issues
Make sure the backend API is running on `http://localhost:8000` and update `.env` if needed.

### Build Errors
Clear Next.js cache:
```bash
rm -rf .next
npm run dev
```

## Next Steps

1. Connect to actual backend API endpoints
2. Replace placeholder images with real content
3. Add user authentication persistence
4. Implement real-time animation generation
5. Add file upload functionality
6. Set up analytics tracking

## Support

For issues or questions, refer to the main project documentation or create an issue in the repository.
