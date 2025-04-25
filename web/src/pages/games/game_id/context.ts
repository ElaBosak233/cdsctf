import { createContext } from "react";

export const Context = createContext<{
    gtLoaded?: boolean;
}>({});
