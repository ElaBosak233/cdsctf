import { Challenge } from "@/models/challenge";
import { Team } from "@/models/team";
import { createContext } from "react";

export const Context = createContext<{
    challenge?: Challenge;
    team?: Team;
}>({});
