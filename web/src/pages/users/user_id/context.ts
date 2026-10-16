import { createContext } from "react";
import type { UserProfile, UserStatistics } from "@/models/user";

export const Context = createContext<{
  user?: UserProfile;
  statistics?: UserStatistics;
}>({});
