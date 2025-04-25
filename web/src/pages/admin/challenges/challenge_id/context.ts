import { Challenge } from "@/models/challenge";
import { createContext } from "react";

export const Context = createContext<{
    challenge?: Challenge;
}>({});
