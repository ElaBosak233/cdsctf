import { createContext } from "react";
import type { ColumnFiltersState } from "@/utils/data-table";

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
