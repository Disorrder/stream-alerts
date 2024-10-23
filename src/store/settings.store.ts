import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { TwitchUser } from "~/types/accounts.type";

type SettingsState = {
  twitch: TwitchUser | null;
  setTwitch: (twitch: TwitchUser) => void;
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      twitch: null,
      setTwitch: (twitch: TwitchUser) => set({ twitch }),
    }),
    { name: "settings" },
  ),
);
