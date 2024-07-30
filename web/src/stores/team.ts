import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface TeamState {
    selectedTeamID?: number;
    setSelectedTeamID: (teamID: number) => void;
}

export const useTeamStore = create<TeamState>()(
    persist(
        (set) => ({
            selectedTeamID: 0,
            setSelectedTeamID: (teamID: number) =>
                set({ selectedTeamID: teamID }),
        }),
        {
            name: "team_storage",
            storage: createJSONStorage(() => sessionStorage),
        }
    )
);
