import { useEffect } from "react";
import { useApperanceStore } from "@/storages/appearance";

function ThemeWatcher() {
  const { theme, computedTheme, setComputedTheme } = useApperanceStore();

  useEffect(() => {
    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";

      setComputedTheme(systemTheme);
    } else {
      setComputedTheme(theme);
    }
  }, [theme, setComputedTheme]);

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove("light", "dark");
    root.classList.add(computedTheme);
  }, [computedTheme]);

  return null;
}

export { ThemeWatcher };
