import { createContext } from "react";
import type { UserPublic } from "@/models/user";

export const Context = createContext<{
  user?: UserPublic;
}>({});
