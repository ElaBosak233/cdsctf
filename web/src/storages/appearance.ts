import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

type Theme = "light" | "dark" | "system";

interface AppearanceState {
    theme: Theme;
    setTheme: (theme: Theme) => void;
}

export const useApperanceStore = create<AppearanceState>()(
    persist(
        (set) => ({
            theme: "system",
            setTheme: (theme: Theme) => set({ theme }),
        }),
        {
            name: "apperance",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
