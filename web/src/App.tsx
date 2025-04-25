import { RouterProvider } from "react-router";
import routers from "@/routers";
import { Toaster } from "@/components/ui/sonner";
import { useEffect } from "react";
import { useThemeStore } from "@/storages/theme";
import { useConfigStore } from "@/storages/config";
import { cn } from "./utils";
import { ScrollArea } from "./components/ui/scroll-area";
import { getConfigs, getVersion } from "./api/configs";

function App() {
    const { theme } = useThemeStore();
    const { setConfig, setVersion } = useConfigStore();

    useEffect(() => {
        const root = window.document.documentElement;
        root.classList.remove("light", "dark");
        if (theme === "system") {
            const systemTheme = window.matchMedia(
                "(prefers-color-scheme: dark)"
            ).matches
                ? "dark"
                : "light";

            root.classList.add(systemTheme);
            return;
        }

        root.classList.add(theme);
    }, [theme]);

    useEffect(() => {
        getConfigs().then((res) => {
            setConfig(res.data!);
        });

        getVersion().then((res) => {
            setVersion(res.data!);
        });
    }, []);

    return (
        <>
            <Toaster />
            <ScrollArea
                className={cn([
                    "relative",
                    "w-screen",
                    "h-screen",
                    "m-0",
                    "overflow-auto",
                ])}
            >
                <RouterProvider router={routers} />
            </ScrollArea>
        </>
    );
}

export default App;
