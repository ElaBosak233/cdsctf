import { Game } from "@/models/game";
import { createContext } from "react";

export const Context = createContext<{
    game?: Game;
}>({});
