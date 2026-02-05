import { createContext } from "react";

import type { Challenge } from "@/models/challenge";

export const Context = createContext<{
  challenge?: Challenge;
}>({});
