import { createContext } from "react";

import type { User } from "@/models/user";

export const Context = createContext<{
  user?: User;
}>({});
