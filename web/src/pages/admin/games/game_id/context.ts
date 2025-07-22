import { createContext } from "react";

import type { Game } from "@/models/game";

export const Context = createContext<{
  game?: Game;
}>({});
