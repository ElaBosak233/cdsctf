import { createContext } from "react";

import type { Challenge } from "@/models/challenge";
import type { Team } from "@/models/team";

export const Context = createContext<{
  challenge?: Challenge;
  team?: Team;
  debug?: boolean;
}>({});
