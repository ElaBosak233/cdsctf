import { createContext } from "react";

import { User } from "@/models/user";

export const Context = createContext<{
  user?: User;
}>({});
