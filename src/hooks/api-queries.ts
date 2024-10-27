import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "~/lib/api";
import type { TwitchUser } from "~/types/accounts.types";

/* Twitch */

export function useTwitchUserQuery() {
  return useQuery({
    queryKey: ["twitch", "user"],
    queryFn: () => api.get<TwitchUser>("/twitch/user").then((res) => res.data),
    meta: { persist: true },
  });
}

export function useTwitchDetachQuery() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => api.delete("/twitch/user"),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["twitch", "user"] });
    },
  });
}
