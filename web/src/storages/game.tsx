import { Game } from "@/models/game";
import { Team } from "@/models/team";
import { UserMini } from "@/models/user";
import { create } from "zustand";

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
