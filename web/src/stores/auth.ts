import { User } from "@/types/user";
import { useNavigate } from "react-router-dom";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface AuthState {
    user?: User;
    pgsToken?: string;
    setUser: (user?: User) => void;
    setPgsToken: (pgsToken: string) => void;

    logout: () => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set) => ({
            setUser: (user) => set({ user }),
            setPgsToken: (pgsToken) => set({ pgsToken }),
            logout: () => set({ user: undefined, pgsToken: undefined }),
        }),
        {
            name: "auth_storage",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
