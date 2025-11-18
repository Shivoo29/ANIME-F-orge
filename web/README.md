# AnimaForge Web Frontend

A modern Next.js 14 frontend for AnimaForge with neo-brutalism design.

## Features

- **Next.js 14** with App Router
- **TypeScript** strict mode
- **Tailwind CSS** with custom neo-brutalism design system
- **React Query** for data fetching and caching
- **Zustand** for state management
- **Axios** for API calls
- **Responsive Design** optimized for all devices

## Design System

### Neo-Brutalism Style
- **Colors:**
  - Primary: #FF6B35 (Orange)
  - Secondary: #004E89 (Blue)
  - Accent: #F7B801 (Yellow)
- **Borders:** 4px thick black borders
- **Shadows:** 8px offset shadows
- **Typography:** Bold, uppercase headings

## Getting Started

### Prerequisites
- Node.js 18+ installed
- npm or yarn package manager

### Installation

1. Install dependencies:
```bash
npm install
# or
yarn install
```

2. Create environment file:
```bash
cp .env.example .env
```

3. Update the `.env` file with your API URL:
```
NEXT_PUBLIC_API_URL=http://localhost:8000/api
```

### Development

Run the development server:
```bash
npm run dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build

Create a production build:
```bash
npm run build
# or
yarn build
```

### Start Production Server

```bash
npm start
# or
yarn start
```

## Project Structure

```
web/
├── app/                      # Next.js App Router pages
│   ├── animation/[id]/       # Animation detail page
│   ├── browse/               # Browse marketplace
│   ├── dashboard/            # User dashboard
│   ├── login/                # Login page
│   ├── register/             # Registration page
│   ├── layout.tsx            # Root layout
│   ├── page.tsx              # Landing page
│   ├── providers.tsx         # React Query provider
│   └── globals.css           # Global styles
├── components/               # Reusable components
│   ├── AnimationCard.tsx     # Animation card component
│   ├── AnimationPlayer.tsx   # Video player component
│   ├── Button.tsx            # Button component
│   ├── Footer.tsx            # Footer component
│   └── Header.tsx            # Header/navigation component
├── lib/                      # Utility libraries
│   └── api.ts                # API client and functions
├── store/                    # State management
│   └── authStore.ts          # Authentication store
├── public/                   # Static assets
├── next.config.mjs           # Next.js configuration
├── tailwind.config.ts        # Tailwind CSS configuration
├── tsconfig.json             # TypeScript configuration
└── package.json              # Dependencies and scripts
```

## Pages

- **Landing Page** (`/`) - Hero, features, how it works, sample animations
- **Browse** (`/browse`) - Grid of animations with search and filters
- **Login** (`/login`) - User authentication
- **Register** (`/register`) - New user registration
- **Dashboard** (`/dashboard`) - User's animations and analytics
- **Animation Detail** (`/animation/[id]`) - View and download animations

## Components

### Button
Neo-brutalism styled button with multiple variants:
- `primary` - Orange background
- `secondary` - Blue background
- `accent` - Yellow background
- `outline` - White background with border

### AnimationCard
Card component displaying animation thumbnail, title, creator info, and stats.

### AnimationPlayer
Video player component with custom controls and play button overlay.

### Header
Navigation header with logo, menu links, and authentication buttons.

### Footer
Footer with links and company information.

## State Management

### Auth Store (Zustand)
Manages user authentication state:
- `user` - Current user object
- `token` - Authentication token
- `login()` - Login function
- `logout()` - Logout function
- `updateUser()` - Update user info

## API Integration

API client configured in `lib/api.ts` with the following endpoints:

### Authentication
- `authAPI.login(email, password)`
- `authAPI.register(username, email, password)`
- `authAPI.me()`

### Animations
- `animationsAPI.getAll(params)`
- `animationsAPI.getById(id)`
- `animationsAPI.create(data)`
- `animationsAPI.update(id, data)`
- `animationsAPI.delete(id)`
- `animationsAPI.like(id)`
- `animationsAPI.download(id)`

### Users
- `userAPI.getProfile(username)`
- `userAPI.getAnimations(username)`
- `userAPI.updateProfile(data)`

## Styling

Uses Tailwind CSS with custom utility classes:
- `.brutal-card` - Card with border and shadow
- `.brutal-card-hover` - Card with hover animation
- `.brutal-input` - Input field with brutal styling
- `.brutal-container` - Responsive container

## License

Part of the AnimaForge project.
