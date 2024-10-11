import { create } from "zustand";
import { persist } from "zustand/middleware";

type SettingsState = {
  test: string;
};

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      test: "test",
    }),
    { name: "settings" },
  ),
);
