import { createContext } from "react";

export const Context = createContext<{
    mode: "default" | "game";
}>({
    mode: "default",
});
