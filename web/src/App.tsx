import { RouterProvider } from "react-router";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Toaster } from "@/components/ui/sonner";
import { CheckerWatcher } from "@/components/utils/checker-watcher";
import routers from "@/routers";
import { cn } from "@/utils";
import "@/utils/i18n";
import { QueryClientProvider } from "@tanstack/react-query";
import { NuqsAdapter } from "nuqs/adapters/react-router/v7";
import { queryClient } from "@/utils/query-client";
import { ThemeWatcher } from "./components/utils/theme-watcher";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Toaster />
      <ScrollArea
        className={cn([
          "relative",
          "w-screen",
          "h-screen",
          "m-0",
          "overflow-hidden",
        ])}
        horizontal={false}
      >
        <CheckerWatcher />
        <ThemeWatcher />
        <NuqsAdapter>
          <RouterProvider router={routers} />
        </NuqsAdapter>
      </ScrollArea>
    </QueryClientProvider>
  );
}

export default App;
