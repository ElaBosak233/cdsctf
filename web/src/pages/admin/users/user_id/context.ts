import { User } from "@/models/user";
import { createContext } from "react";

export const Context = createContext<{
    user?: User;
}>({});
