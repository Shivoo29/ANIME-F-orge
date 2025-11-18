# Components Documentation

## Core Components

### Button
A neo-brutalism styled button component with multiple variants and sizes.

**Props:**
- `variant`: "primary" | "secondary" | "accent" | "outline" (default: "primary")
- `size`: "sm" | "md" | "lg" (default: "md")
- All standard HTML button attributes

**Usage:**
```tsx
import Button from "@/components/Button";

<Button variant="primary" size="lg">
  Click Me
</Button>
```

**Variants:**
- `primary` - Orange background (#FF6B35), white text
- `secondary` - Blue background (#004E89), white text
- `accent` - Yellow background (#F7B801), dark text
- `outline` - White background, dark text with border

---

### Header
Navigation header with logo, menu links, and authentication buttons.

**Features:**
- Responsive design
- Logo with link to home
- Navigation menu (Browse, Features, How It Works)
- Auth buttons (Login/Sign Up or Dashboard/Logout)
- Auto-detects user state from auth store

**Usage:**
```tsx
import Header from "@/components/Header";

<Header />
```

---

### Footer
Footer component with links and company information.

**Features:**
- 4-column grid layout (responsive)
- Product, Resources, Legal sections
- Copyright notice with current year
- Neo-brutalism styling

**Usage:**
```tsx
import Footer from "@/components/Footer";

<Footer />
```

---

### AnimationCard
Card component displaying animation information with hover effects.

**Props:**
- `id`: string - Animation ID for routing
- `title`: string - Animation title
- `thumbnail`: string - Image URL for thumbnail
- `creator`: object - Creator info (username, avatar)
- `views`: number - View count (optional)
- `likes`: number - Like count (optional)

**Usage:**
```tsx
import AnimationCard from "@/components/AnimationCard";

<AnimationCard
  id="123"
  title="Amazing Animation"
  thumbnail="https://example.com/thumb.jpg"
  creator={{
    username: "artist",
    avatar: "https://example.com/avatar.jpg"
  }}
  views={1000}
  likes={50}
/>
```

**Features:**
- Click to navigate to animation detail page
- Displays creator avatar (or initial if no avatar)
- Shows view and like counts
- Hover animation (translate + shadow change)
- Responsive image loading with Next.js Image

---

### AnimationPlayer
Video player component with custom controls and styling.

**Props:**
- `videoUrl`: string - URL to video file
- `poster`: string (optional) - Poster image URL

**Usage:**
```tsx
import AnimationPlayer from "@/components/AnimationPlayer";

<AnimationPlayer
  videoUrl="https://example.com/video.mp4"
  poster="https://example.com/poster.jpg"
/>
```

**Features:**
- Native video controls
- Custom play button overlay
- Click to play/pause
- Neo-brutalism styled play button
- Responsive aspect ratio (16:9)
- Video fills container while maintaining aspect ratio

---

## Utility Classes

### Tailwind Custom Classes

**Cards:**
```css
.brutal-card
/* Border, shadow, no hover effect */

.brutal-card-hover
/* Border, shadow, with hover animation */
```

**Inputs:**
```css
.brutal-input
/* Styled input field with border and focus ring */
```

**Container:**
```css
.brutal-container
/* Responsive container with padding */
```

**Example Usage:**
```tsx
<div className="brutal-card p-6">
  <h3>Card Content</h3>
</div>

<input className="brutal-input w-full" placeholder="Type here..." />
```

---

## Layout Components

### Root Layout
The main layout wrapper for the entire application.

**Location:** `app/layout.tsx`

**Features:**
- Includes global styles
- Wraps app in Providers (React Query)
- Includes Header and Footer
- Sets metadata (title, description)
- Font configuration (Inter)

---

### Providers
Client component that wraps the app with necessary providers.

**Location:** `app/providers.tsx`

**Features:**
- React Query configuration
- 60-second stale time for queries
- Singleton QueryClient instance

---

## Page Components

### Landing Page (`/`)
Hero, features, how it works, sample animations.

### Browse Page (`/browse`)
Grid with search and category filters.

### Login Page (`/login`)
Email/password authentication.

### Register Page (`/register`)
User registration form.

### Dashboard Page (`/dashboard`)
User's animations and creation tools.

### Animation Detail Page (`/animation/[id]`)
Full animation view with download.

---

## State Management

### Auth Store (Zustand)
Global authentication state.

**Location:** `store/authStore.ts`

**Usage:**
```tsx
import { useAuthStore } from "@/store/authStore";

function MyComponent() {
  const { user, login, logout } = useAuthStore();

  return (
    <div>
      {user ? (
        <p>Welcome, {user.username}!</p>
      ) : (
        <p>Please log in</p>
      )}
    </div>
  );
}
```

**State:**
- `user`: User | null
- `token`: string | null

**Actions:**
- `login(user, token)` - Set user and token
- `logout()` - Clear user and token
- `updateUser(updates)` - Update user info

---

## API Client

### API Functions
Centralized API client with Axios.

**Location:** `lib/api.ts`

**Usage:**
```tsx
import { authAPI, animationsAPI } from "@/lib/api";

// Login
const { data } = await authAPI.login(email, password);

// Get animations
const { data } = await animationsAPI.getAll({ search: "robot" });

// Create animation
const formData = new FormData();
formData.append("prompt", "A dancing robot");
const { data } = await animationsAPI.create(formData);
```

**Available APIs:**
- `authAPI` - Authentication endpoints
- `animationsAPI` - Animation CRUD operations
- `userAPI` - User profile operations

---

## Styling Guidelines

### Neo-Brutalism Design Principles

1. **Borders**: Always use 4px thick black borders
2. **Shadows**: 8px offset for depth, no blur
3. **Colors**: Use primary, secondary, accent from theme
4. **Typography**: Bold, uppercase for headings
5. **Spacing**: Generous padding and margins
6. **Interactions**: Translate + shadow change on hover

### Color Palette
- Primary: `#FF6B35` - Use for main CTAs
- Secondary: `#004E89` - Use for secondary elements
- Accent: `#F7B801` - Use for highlights
- Dark: `#000000` - Borders and text
- Light: `#FFFFFF` - Backgrounds

### Typography Scale
- `h1`: 4xl-6xl, bold, uppercase
- `h2`: 3xl-5xl, bold, uppercase
- `h3`: 2xl-4xl, bold, uppercase
- `h4`: xl-3xl, bold, uppercase
- Body: base, normal weight

### Responsive Breakpoints
- `sm`: 640px
- `md`: 768px
- `lg`: 1024px
- `xl`: 1280px

---

## Best Practices

1. **Always use semantic HTML** for accessibility
2. **Prefer Next.js Image** over `<img>` tags
3. **Use React Query** for data fetching
4. **Keep components small** and focused
5. **Extract reusable logic** into hooks
6. **Type everything** with TypeScript
7. **Follow naming conventions**:
   - Components: PascalCase
   - Files: PascalCase for components, camelCase for utils
   - Variables: camelCase
   - Constants: UPPER_SNAKE_CASE

---

## Adding New Components

1. Create file in `components/` directory
2. Use TypeScript with proper types
3. Follow neo-brutalism design system
4. Add documentation here
5. Export from component file

**Template:**
```tsx
import { ReactNode } from "react";
import clsx from "clsx";

interface MyComponentProps {
  children: ReactNode;
  variant?: "default" | "special";
  className?: string;
}

export default function MyComponent({
  children,
  variant = "default",
  className,
}: MyComponentProps) {
  return (
    <div className={clsx("brutal-card", className)}>
      {children}
    </div>
  );
}
```
