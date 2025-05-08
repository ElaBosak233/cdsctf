import { User } from "@/models/user";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface AuthState {
    user?: User;
    setUser: (user?: User) => void;
    clear: () => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set, _get) => ({
            setUser: (user?: User) => set({ user }),
            clear: () => set({ user: undefined }),
        }),
        {
            name: "auth",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
