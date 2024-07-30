import { User } from "@/types/user";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface AuthState {
    user?: User;
    pgsToken?: string;
    setUser: (user?: User) => void;
    setPgsToken: (pgsToken: string) => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set) => ({
            setUser: (user) => set({ user }),
            setPgsToken: (pgsToken) => set({ pgsToken }),
        }),
        {
            name: "auth_storage",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
