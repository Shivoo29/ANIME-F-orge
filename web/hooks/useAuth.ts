import { useMutation, useQuery } from "@tanstack/react-query";
import { useAuthStore } from "@/store/authStore";
import { authAPI } from "@/lib/api";

/**
 * Hook to handle user login
 */
export function useLogin() {
  const { login } = useAuthStore();

  return useMutation({
    mutationFn: ({ email, password }: { email: string; password: string }) =>
      authAPI.login(email, password),
    onSuccess: (response) => {
      const { user, token } = response.data;
      login(user, token);
    },
  });
}

/**
 * Hook to handle user registration
 */
export function useRegister() {
  const { login } = useAuthStore();

  return useMutation({
    mutationFn: ({
      username,
      email,
      password,
    }: {
      username: string;
      email: string;
      password: string;
    }) => authAPI.register(username, email, password),
    onSuccess: (response) => {
      const { user, token } = response.data;
      login(user, token);
    },
  });
}

/**
 * Hook to fetch current user data
 */
export function useCurrentUser() {
  const { token } = useAuthStore();

  return useQuery({
    queryKey: ["user", "me"],
    queryFn: () => authAPI.me(),
    enabled: !!token,
  });
}

/**
 * Hook to handle logout
 */
export function useLogout() {
  const { logout } = useAuthStore();

  return () => {
    logout();
    // Optionally clear React Query cache
    // queryClient.clear();
  };
}

/**
 * Hook to check if user is authenticated
 */
export function useIsAuthenticated() {
  const { user, token } = useAuthStore();
  return !!user && !!token;
}
