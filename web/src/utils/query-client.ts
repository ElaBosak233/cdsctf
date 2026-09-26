import { QueryCache, QueryClient } from "@tanstack/react-query";
import { notifyApiError } from "@/utils/query";

export const queryClient = new QueryClient({
  queryCache: new QueryCache({
    onError: (error, query) => {
      if (query.meta?.suppressErrorToast) return;
      void notifyApiError(error);
    },
  }),
});
