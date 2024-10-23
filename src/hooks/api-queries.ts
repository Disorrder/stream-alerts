import { useQuery } from "@tanstack/react-query";
import { api } from "~/lib/api";
import type { TwitchUser } from "~/types/accounts.type";

/* Twitch */

export function useTwitchUserQuery() {
  return useQuery({
    queryKey: ["twitch", "user"],
    queryFn: () => api.get<TwitchUser>("/twitch/user").then((res) => res.data),
    meta: { persist: true },
  });
}
