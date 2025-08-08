import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

export type Theme = "light" | "dark" | "system";

interface AppearanceState {
  theme: Theme;
  setTheme: (theme: Theme) => void;

  computedTheme: Theme;
  setComputedTheme: (theme: Theme) => void;
}

export const useApperanceStore = create<AppearanceState>()(
  persist(
    (set) => ({
      theme: "system",
      setTheme: (theme: Theme) => set({ theme }),

      computedTheme: "light",
      setComputedTheme: (theme: Theme) => set({ computedTheme: theme }),
    }),
    {
      name: "apperance",
      storage: createJSONStorage(() => localStorage),
    }
  )
);
