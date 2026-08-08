import { createContext } from "react";
import type { UserProfile } from "@/models/user";

export const Context = createContext<{
  user?: UserProfile;
}>({});
