import { createContext } from "react";

import { Challenge } from "@/models/challenge";
import { Team } from "@/models/team";

export const Context = createContext<{
  challenge?: Challenge;
  team?: Team;
}>({});
