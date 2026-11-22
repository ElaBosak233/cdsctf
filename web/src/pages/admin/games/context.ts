import { createContext } from "react";

export type GameFilters = {
  q: string;
  enabled: boolean | null;
};

type GameListContextValue = {
  createDialogOpen: boolean;
  setCreateDialogOpen: (open: boolean) => void;
  filters: GameFilters;
};

export const GameListContext = createContext<GameListContextValue | null>(null);
