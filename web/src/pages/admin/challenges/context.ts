import { createContext } from "react";

export type ChallengeFilters = {
  q: string;
  category: number | null;
  public: boolean | null;
};

type ChallengeListContextValue = {
  createDialogOpen: boolean;
  setCreateDialogOpen: (open: boolean) => void;
  filters: ChallengeFilters;
};

export const ChallengeListContext =
  createContext<ChallengeListContextValue | null>(null);
