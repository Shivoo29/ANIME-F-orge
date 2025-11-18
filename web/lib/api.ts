import axios from "axios";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8000/api";

export const api = axios.create({
  baseURL: API_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem("auth-token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// API functions
export const authAPI = {
  login: (email: string, password: string) =>
    api.post("/auth/login", { email, password }),
  register: (username: string, email: string, password: string) =>
    api.post("/auth/register", { username, email, password }),
  me: () => api.get("/auth/me"),
};

export const animationsAPI = {
  getAll: (params?: { search?: string; category?: string; page?: number }) =>
    api.get("/animations", { params }),
  getById: (id: string) => api.get(`/animations/${id}`),
  create: (data: FormData) =>
    api.post("/animations", data, {
      headers: { "Content-Type": "multipart/form-data" },
    }),
  update: (id: string, data: any) => api.put(`/animations/${id}`, data),
  delete: (id: string) => api.delete(`/animations/${id}`),
  like: (id: string) => api.post(`/animations/${id}/like`),
  download: (id: string) => api.get(`/animations/${id}/download`),
};

export const userAPI = {
  getProfile: (username: string) => api.get(`/users/${username}`),
  getAnimations: (username: string) => api.get(`/users/${username}/animations`),
  updateProfile: (data: any) => api.put("/users/me", data),
};
