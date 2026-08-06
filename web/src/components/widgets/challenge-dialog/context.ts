import { createContext } from "react";

import type { ChallengeView } from "@/models/challenge";
import type { TeamView } from "@/models/team";

export const Context = createContext<{
  challenge?: Partial<ChallengeView> &
    Pick<ChallengeView, "id" | "title" | "category">;
  team?: TeamView;
  debug?: boolean;
  cheated?: boolean;
}>({});
