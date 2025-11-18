// User types
export interface User {
  id: string;
  username: string;
  email: string;
  avatar?: string;
  bio?: string;
  createdAt: string;
}

// Animation types
export interface Animation {
  id: string;
  title: string;
  description: string;
  videoUrl: string;
  thumbnail: string;
  creator: {
    username: string;
    avatar?: string;
    bio?: string;
  };
  views: number;
  likes: number;
  tags: string[];
  category: string;
  duration: string;
  resolution: string;
  fps: number;
  createdAt: string;
  updatedAt: string;
}

// API Response types
export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message?: string;
}

export interface PaginatedResponse<T> {
  success: boolean;
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

// Auth types
export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  user: User;
  token: string;
}

// Animation creation types
export interface CreateAnimationRequest {
  prompt: string;
  style: "anime" | "realistic" | "cartoon" | "3d";
  duration: 3 | 5 | 10 | 15;
  resolution: "720p" | "1080p" | "4k";
}
