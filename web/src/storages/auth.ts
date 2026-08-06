import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

import type { UserAccountView } from "@/models/user";

type AuthState = {
  user?: UserAccountView;
  setUser: (user?: UserAccountView) => void;
  clear: () => void;
};

export const useAuthStore = create<AuthState>()(
  persist(
    (set, _get) => ({
      setUser: (user?: UserAccountView) => set({ user }),
      clear: () => set({ user: undefined }),
    }),
    {
      name: "auth",
      storage: createJSONStorage(() => localStorage),
    }
  )
);
