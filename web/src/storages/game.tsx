import { Game } from "@/models/game";
import { Team } from "@/models/team";
import { create } from "zustand";

interface GameState {
    currentGame?: Game;
    setCurrentGame: (game?: Game) => void;

    selfTeam?: Team;
    setSelfTeam: (team?: Team) => void;
}

export const useGameStore = create<GameState>()((set, get) => ({
    setCurrentGame: (game) => set({ currentGame: game }),
    setSelfTeam: (team) => set({ selfTeam: team }),
}));
