import { RouterProvider } from "react-router";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Toaster } from "@/components/ui/sonner";
import { CheckerWatcher } from "@/components/utils/checker-watcher";
import { Background } from "@/components/widgets/background";
import routers from "@/routers";
import { cn } from "@/utils";
import "@/utils/i18n";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeWatcher } from "./components/utils/theme-watcher";

const queryClient = new QueryClient();

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
          "overflow-auto",
        ])}
        horizontal={false}
      >
        <Background />
        <CheckerWatcher />
        <ThemeWatcher />
        <RouterProvider router={routers} />
      </ScrollArea>
    </QueryClientProvider>
  );
}

export default App;
