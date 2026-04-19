import type { ColumnFiltersState } from "@tanstack/react-table";
import { createContext } from "react";

export type ChallengeListContextValue = {
  createDialogOpen: boolean;
  setCreateDialogOpen: (open: boolean) => void;
  columnFilters: ColumnFiltersState;
  setColumnFilters: (
    updater:
      | ColumnFiltersState
      | ((prev: ColumnFiltersState) => ColumnFiltersState)
  ) => void;
};

export const ChallengeListContext =
  createContext<ChallengeListContextValue | null>(null);
