import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

type Theme = "light" | "dark" | "system";

interface ThemeState {
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

export const useThemeStore = create<ThemeState>()(
    persist(
        (set) => ({
            theme: "system",
            setTheme: (theme: Theme) => set({ theme }),
        }),
        {
            name: "theme",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
