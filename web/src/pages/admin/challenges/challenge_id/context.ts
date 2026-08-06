import { createContext } from "react";

import type { ChallengeDetail } from "@/models/challenge";

export const Context = createContext<{
  challenge?: ChallengeDetail;
}>({});
