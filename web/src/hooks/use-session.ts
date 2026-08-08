import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { getUserProfile } from "@/api/users/me";
import {
  clearAuthenticatedUser,
  markAuthUnavailable,
  sessionQueryKey,
  setAuthenticatedUser,
} from "@/storages/auth";

export function useSession() {
  return useQuery({
    queryKey: sessionQueryKey,
    queryFn: async () => {
      try {
        const user = (await getUserProfile()).user;
        setAuthenticatedUser(user);
        return user;
      } catch (error) {
        if (
          error instanceof HTTPError &&
          error.response.status === StatusCodes.UNAUTHORIZED
        ) {
          clearAuthenticatedUser();
          return null;
        }

        markAuthUnavailable();
        throw error;
      }
    },
    retry: false,
    staleTime: 30_000,
  });
}
