import { useEffect } from "react";
import { RouterProvider } from "react-router";

import { ScrollArea } from "@/components/ui/scroll-area";
import { Toaster } from "@/components/ui/sonner";
import { CheckerWatcher } from "@/components/utils/checker-watcher";
import { Background } from "@/components/widgets/background";
import routers from "@/routers";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";
import "@/utils/i18n";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

function App() {
  const { theme } = useApperanceStore();

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");
    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(theme);
  }, [theme]);

  return (
    <QueryClientProvider client={queryClient}>
      <Toaster />
      <ScrollArea
        className={cn([
          "relative",
          "w-screen",
          "h-screen",
          "m-0",
          "overflow-auto",
        ])}
        horizontal={false}
      >
        <Background />
        <CheckerWatcher />
        <RouterProvider router={routers} />
      </ScrollArea>
    </QueryClientProvider>
  );
}

export default App;
