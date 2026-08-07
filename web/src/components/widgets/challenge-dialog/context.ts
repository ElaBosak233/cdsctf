import { createContext } from "react";

import type { ChallengeView } from "@/models/challenge";
import type { PlayerTeamView } from "@/models/team";

export const Context = createContext<{
  challenge?: Partial<ChallengeView> &
    Pick<ChallengeView, "id" | "title" | "category">;
  team?: PlayerTeamView;
  debug?: boolean;
  cheated?: boolean;
}>({});
