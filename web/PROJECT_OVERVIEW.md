# AnimaForge Web - Project Overview

## Complete Next.js 14 Frontend Implementation

This is a production-ready Next.js 14 frontend for AnimaForge, featuring a neo-brutalism design system, complete authentication flow, animation browsing, and creation capabilities.

---

## Project Statistics

- **Total Files**: 30+
- **Pages**: 6 (Landing, Browse, Login, Register, Dashboard, Animation Detail)
- **Components**: 5 reusable components
- **Custom Hooks**: 2 hook modules
- **TypeScript**: Strict mode enabled
- **Design System**: Neo-brutalism with 3 primary colors

---

## Technology Stack

### Core
- **Next.js 14.2.15** - React framework with App Router
- **React 18.3.1** - UI library
- **TypeScript 5.6.3** - Type safety

### Styling
- **Tailwind CSS 3.4.14** - Utility-first CSS
- **Custom Neo-Brutalism Design** - Unique visual identity

### State & Data
- **Zustand 4.5.5** - State management
- **TanStack Query 5.59.0** - Data fetching & caching
- **Axios 1.7.7** - HTTP client

### Developer Tools
- **ESLint** - Code quality
- **PostCSS** - CSS processing
- **Autoprefixer** - CSS compatibility

---

## File Structure

```
web/
├── app/                          # Next.js App Router
│   ├── animation/[id]/
│   │   └── page.tsx             # Dynamic animation detail page
│   ├── browse/
│   │   └── page.tsx             # Marketplace with filters
│   ├── dashboard/
│   │   └── page.tsx             # User dashboard
│   ├── login/
│   │   └── page.tsx             # Login page
│   ├── register/
│   │   └── page.tsx             # Registration page
│   ├── globals.css               # Global styles + Tailwind
│   ├── layout.tsx                # Root layout with Header/Footer
│   ├── page.tsx                  # Landing page
│   └── providers.tsx             # React Query provider
│
├── components/                   # Reusable components
│   ├── AnimationCard.tsx         # Card with thumbnail & info
│   ├── AnimationPlayer.tsx       # Video player with controls
│   ├── Button.tsx                # Neo-brutalism button
│   ├── Footer.tsx                # Footer with links
│   └── Header.tsx                # Navigation header
│
├── hooks/                        # Custom React hooks
│   ├── useAnimations.ts          # Animation CRUD hooks
│   └── useAuth.ts                # Authentication hooks
│
├── lib/                          # Libraries & utilities
│   └── api.ts                    # Axios API client
│
├── store/                        # State management
│   └── authStore.ts              # Zustand auth store
│
├── types/                        # TypeScript types
│   └── index.ts                  # Shared type definitions
│
├── utils/                        # Utility functions
│   └── helpers.ts                # Common helper functions
│
├── public/                       # Static assets
│
├── .env.example                  # Environment variables template
├── .eslintrc.json                # ESLint configuration
├── .gitignore                    # Git ignore rules
├── COMPONENTS.md                 # Component documentation
├── next-env.d.ts                 # Next.js TypeScript definitions
├── next.config.mjs               # Next.js configuration
├── package.json                  # Dependencies & scripts
├── postcss.config.mjs            # PostCSS configuration
├── PROJECT_OVERVIEW.md           # This file
├── README.md                     # Main documentation
├── SETUP.md                      # Quick setup guide
├── tailwind.config.ts            # Tailwind configuration
└── tsconfig.json                 # TypeScript configuration
```

---

## Design System

### Neo-Brutalism Principles

1. **Bold Borders**: 4px thick black borders on all interactive elements
2. **Heavy Shadows**: 8px offset shadows for depth (no blur)
3. **Vibrant Colors**: High-contrast color palette
4. **Uppercase Typography**: Bold, uppercase headings
5. **Generous Spacing**: Ample padding and margins
6. **Playful Interactions**: Translate + shadow effects on hover

### Color Palette

```css
Primary:   #FF6B35  /* Orange - Main CTAs */
Secondary: #004E89  /* Blue - Secondary elements */
Accent:    #F7B801  /* Yellow - Highlights */
Dark:      #000000  /* Borders & text */
Light:     #FFFFFF  /* Backgrounds */
```

### Custom Tailwind Classes

```css
.brutal-card          /* Card with border & shadow */
.brutal-card-hover    /* Card with hover animation */
.brutal-input         /* Styled input field */
.brutal-container     /* Responsive container */
```

---

## Pages Overview

### 1. Landing Page (`/`)
**Purpose**: Convert visitors to users

**Sections**:
- Hero with clear value proposition
- How It Works (3 steps)
- Features showcase (4 features)
- Sample animations grid
- Multiple CTAs to sign up

**Key Message**: "Transform words into stunning animations. No code required."

---

### 2. Browse Marketplace (`/browse`)
**Purpose**: Explore community animations

**Features**:
- Search bar with live filtering
- Category filters (7 categories)
- Animation grid (responsive)
- View counters and likes
- Load more functionality

**Categories**: All, Anime, Fantasy, Sci-Fi, Action, Nature, Abstract

---

### 3. Login Page (`/login`)
**Purpose**: User authentication

**Features**:
- Email/password fields
- Form validation
- Error messaging
- Link to registration
- Forgot password link

**Redirects**: Dashboard on success

---

### 4. Register Page (`/register`)
**Purpose**: New user onboarding

**Features**:
- Username, email, password fields
- Password confirmation
- Strength requirements (8+ chars)
- Terms acceptance
- Form validation
- Error messaging

**Redirects**: Dashboard on success

---

### 5. Dashboard (`/dashboard`)
**Purpose**: User's animation hub

**Features**:
- Stats overview (4 cards)
- Quick action buttons
- User's animations grid
- Create animation modal
- Prompt-based generation

**Protected**: Requires authentication

---

### 6. Animation Detail (`/animation/[id]`)
**Purpose**: View and download animations

**Features**:
- Full-size video player
- Animation metadata
- Like/download/share buttons
- Technical details (resolution, FPS, duration)
- Creator profile card
- Related animations
- Tags

---

## Components

### Button
Multi-variant button with neo-brutalism styling.

**Variants**: primary, secondary, accent, outline
**Sizes**: sm, md, lg
**Features**: Hover animations, disabled states

---

### AnimationCard
Displays animation preview with metadata.

**Data**: thumbnail, title, creator, views, likes
**Interaction**: Click to navigate to detail page
**Features**: Hover effect, responsive images

---

### AnimationPlayer
Custom video player with controls.

**Features**: Play/pause overlay, native controls, poster image
**Styling**: Neo-brutalism play button

---

### Header
Main navigation component.

**Features**: Logo, menu links, auth buttons
**States**: Logged in vs logged out
**Responsive**: Mobile-friendly

---

### Footer
Site footer with links.

**Sections**: Product, Resources, Legal
**Features**: Auto-updated copyright year

---

## State Management

### Auth Store (Zustand)
Persistent authentication state.

**State**:
- `user`: Current user object
- `token`: JWT token

**Actions**:
- `login(user, token)`
- `logout()`
- `updateUser(updates)`

**Persistence**: localStorage via Zustand middleware

---

## API Integration

### Base URL
`http://localhost:8000/api` (configurable via .env)

### Endpoints

**Authentication**:
- `POST /auth/login`
- `POST /auth/register`
- `GET /auth/me`

**Animations**:
- `GET /animations`
- `GET /animations/:id`
- `POST /animations`
- `PUT /animations/:id`
- `DELETE /animations/:id`
- `POST /animations/:id/like`
- `GET /animations/:id/download`

**Users**:
- `GET /users/:username`
- `GET /users/:username/animations`
- `PUT /users/me`

### Request Interceptor
Automatically adds JWT token to all requests.

---

## Custom Hooks

### useAnimations
Fetch animations with filters.

```tsx
const { data, isLoading } = useAnimations({
  search: "robot",
  category: "Sci-Fi"
});
```

### useAnimation
Fetch single animation by ID.

```tsx
const { data } = useAnimation("123");
```

### useCreateAnimation
Create new animation with mutation.

```tsx
const { mutate } = useCreateAnimation();
mutate(formData);
```

### useLogin
Handle user login.

```tsx
const { mutate } = useLogin();
mutate({ email, password });
```

### useRegister
Handle user registration.

```tsx
const { mutate } = useRegister();
mutate({ username, email, password });
```

---

## Utility Functions

**Formatting**:
- `formatNumber(num)` - Add commas
- `formatDate(date)` - Readable date
- `formatRelativeTime(date)` - "2 days ago"
- `formatFileSize(bytes)` - "1.5 MB"

**Validation**:
- `isValidEmail(email)` - Email validation
- `validatePassword(password)` - Password strength

**Helpers**:
- `truncateText(text, length)` - Text truncation
- `debounce(func, wait)` - Debounce function
- `copyToClipboard(text)` - Copy to clipboard
- `downloadFile(url, filename)` - File download

---

## User Journey

1. **Land** → See hero and value proposition
2. **Explore** → Browse sample animations
3. **Sign Up** → Create account
4. **Create** → Generate first animation
5. **Download** → Get animation file
6. **Share** → Show to community

---

## Key Features

✅ **Fast Loading**: Next.js Image optimization
✅ **SEO Ready**: Metadata configuration
✅ **Type Safe**: Strict TypeScript mode
✅ **Responsive**: Mobile-first design
✅ **Accessible**: Semantic HTML
✅ **Modern**: React 18 + Next.js 14
✅ **Performant**: React Query caching
✅ **Scalable**: Component-based architecture
✅ **User-Friendly**: Clear messaging throughout
✅ **Professional**: Production-ready code

---

## Quick Start

```bash
# Install dependencies
npm install

# Setup environment
cp .env.example .env

# Start development
npm run dev

# Visit http://localhost:3000
```

---

## Development Workflow

1. **Local Development**: `npm run dev`
2. **Type Checking**: TypeScript validates on save
3. **Linting**: ESLint catches issues
4. **Building**: `npm run build`
5. **Production**: `npm start`

---

## Deployment Checklist

- [ ] Update API URL in `.env`
- [ ] Run `npm run build` successfully
- [ ] Test all pages
- [ ] Verify authentication flow
- [ ] Check responsive design
- [ ] Test animation playback
- [ ] Validate forms
- [ ] Test error states
- [ ] Verify SEO metadata
- [ ] Performance audit

---

## Future Enhancements

1. **Real-time Updates**: WebSocket for live notifications
2. **Advanced Filters**: More search options
3. **User Profiles**: Public profile pages
4. **Animation Editor**: In-browser editing
5. **Social Features**: Comments, follows
6. **Analytics**: Detailed stats dashboard
7. **Payment**: Subscription tiers
8. **Mobile App**: React Native version

---

## Support & Documentation

- **Setup Guide**: See `SETUP.md`
- **Component Docs**: See `COMPONENTS.md`
- **Main README**: See `README.md`
- **API Docs**: See backend API documentation

---

## License

Part of the AnimaForge project.

---

**Built with ❤️ using Next.js 14 and neo-brutalism design**
