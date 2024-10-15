import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { TwitchAccount } from "~/types/accounts.type";

type SettingsState = {
  twitch: TwitchAccount | null;
  setTwitch: (twitch: TwitchAccount) => void;
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      twitch: null,
      setTwitch: (twitch: TwitchAccount) => set({ twitch }),
    }),
    { name: "settings" },
  ),
);
