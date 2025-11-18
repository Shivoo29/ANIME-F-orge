import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { animationsAPI } from "@/lib/api";

/**
 * Hook to fetch all animations with optional filters
 */
export function useAnimations(params?: {
  search?: string;
  category?: string;
  page?: number;
}) {
  return useQuery({
    queryKey: ["animations", params],
    queryFn: () => animationsAPI.getAll(params),
  });
}

/**
 * Hook to fetch a single animation by ID
 */
export function useAnimation(id: string) {
  return useQuery({
    queryKey: ["animation", id],
    queryFn: () => animationsAPI.getById(id),
    enabled: !!id,
  });
}

/**
 * Hook to create a new animation
 */
export function useCreateAnimation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: FormData) => animationsAPI.create(data),
    onSuccess: () => {
      // Invalidate and refetch animations list
      queryClient.invalidateQueries({ queryKey: ["animations"] });
    },
  });
}

/**
 * Hook to like an animation
 */
export function useLikeAnimation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => animationsAPI.like(id),
    onSuccess: (_, id) => {
      // Invalidate the specific animation
      queryClient.invalidateQueries({ queryKey: ["animation", id] });
    },
  });
}

/**
 * Hook to delete an animation
 */
export function useDeleteAnimation() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => animationsAPI.delete(id),
    onSuccess: () => {
      // Invalidate animations list
      queryClient.invalidateQueries({ queryKey: ["animations"] });
    },
  });
}
