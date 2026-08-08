import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import type { UserAccountView } from "@/models/user";
import { queryClient } from "@/utils/query-client";

export const sessionQueryKey = ["session"] as const;

export type AuthStatus =
  | "checking"
  | "authenticated"
  | "anonymous"
  | "unavailable";

type AuthState = {
  status: AuthStatus;
  user?: UserAccountView;
};

export const useAuthStore = create<AuthState>()(
  persist(
    (): AuthState => ({
      status: "checking",
      user: undefined,
    }),
    {
      name: "auth",
      storage: createJSONStorage(() => localStorage),
      partialize: (state): AuthState => ({
        status: "checking",
        user: state.user,
      }),
    }
  )
);

export function hydrateAuth(user: UserAccountView | null) {
  useAuthStore.setState({
    status: user ? "authenticated" : "anonymous",
    user: user ?? undefined,
  });
}

export function markAuthUnavailable() {
  useAuthStore.setState({ status: "unavailable" });
}

export function setAuthenticatedUser(user: UserAccountView) {
  queryClient.setQueryData(sessionQueryKey, user);
  hydrateAuth(user);
}

export function patchAuthenticatedUser(patch: Partial<UserAccountView>) {
  const current = useAuthStore.getState().user;
  if (!current) return;

  setAuthenticatedUser({ ...current, ...patch });
}

export function clearAuthenticatedUser() {
  queryClient.setQueryData(sessionQueryKey, null);
  hydrateAuth(null);
}
