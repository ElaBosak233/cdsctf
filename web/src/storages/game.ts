import { create } from "zustand";

import type { GameView } from "@/models/game";
import type { PlayerTeamView } from "@/models/team";
import type { UserSummary } from "@/models/user";

type GameState = {
  currentGame?: GameView;
  setCurrentGame: (game?: GameView) => void;

  selfTeam?: PlayerTeamView;
  setSelfTeam: (team?: PlayerTeamView) => void;

  members?: Array<UserSummary>;
  setMembers: (users?: Array<UserSummary>) => void;
};

export const useGameStore = create<GameState>()((set, _get) => ({
  setCurrentGame: (game) => set({ currentGame: game }),
  setSelfTeam: (team) => set({ selfTeam: team }),
  setMembers: (users) => set({ members: users }),
}));
