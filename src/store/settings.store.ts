import { create } from "zustand";
import { persist } from "zustand/middleware";

type SettingsState = {
  twitchAccessToken: string | null;
  setTwitchTokens: (twitchAccessToken: string) => void;
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      twitchAccessToken: null,
      setTwitchTokens: (twitchAccessToken) => set({ twitchAccessToken }),
    }),
    { name: "settings" },
  ),
);
