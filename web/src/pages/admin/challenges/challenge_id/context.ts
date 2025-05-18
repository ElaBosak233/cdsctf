import { createContext } from "react";

import { Challenge } from "@/models/challenge";

export const Context = createContext<{
  challenge?: Challenge;
}>({});
