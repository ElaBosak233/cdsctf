import { createContext } from "react";

import type { GameDetail } from "@/models/game";

export const Context = createContext<{
  game?: GameDetail;
}>({});
