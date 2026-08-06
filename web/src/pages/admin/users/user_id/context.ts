import { createContext } from "react";

import type { UserAccountView } from "@/models/user";

export const Context = createContext<{
  user?: UserAccountView;
}>({});
