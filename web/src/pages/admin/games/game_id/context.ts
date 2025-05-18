import { createContext } from "react";

import { Game } from "@/models/game";

export const Context = createContext<{
  game?: Game;
}>({});
