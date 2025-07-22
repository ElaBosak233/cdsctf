import { create } from "zustand";

import type { Game } from "@/models/game";
import type { Team } from "@/models/team";
import type { UserMini } from "@/models/user";

interface GameState {
  currentGame?: Game;
  setCurrentGame: (game?: Game) => void;

  selfTeam?: Team;
  setSelfTeam: (team?: Team) => void;

  members?: Array<UserMini>;
  setMembers: (users?: Array<UserMini>) => void;
}

export const useGameStore = create<GameState>()((set, _get) => ({
  setCurrentGame: (game) => set({ currentGame: game }),
  setSelfTeam: (team) => set({ selfTeam: team }),
  setMembers: (users) => set({ members: users }),
}));
