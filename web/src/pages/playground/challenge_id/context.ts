import { createContext } from "react";

import type { ChallengeView } from "@/models/challenge";

export const Context = createContext<{
  challenge?: ChallengeView;
}>({});
