import type { ColumnFiltersState } from "@tanstack/react-table";
import { createContext } from "react";

export type AdminListContextValue = {
  createDialogOpen: boolean;
  setCreateDialogOpen: (open: boolean) => void;
  columnFilters: ColumnFiltersState;
  setColumnFilters: (
    updater:
      | ColumnFiltersState
      | ((prev: ColumnFiltersState) => ColumnFiltersState)
  ) => void;
  page: number;
  setPage: (page: number) => void;
  size: number;
  setSize: (size: number) => void;
  total: number;
  setTotal: (total: number) => void;
};

export const AdminListContext = createContext<AdminListContextValue | null>(
  null
);

/** @deprecated 使用 AdminListContext 替代 */
export const CreateDialogContext = AdminListContext;
