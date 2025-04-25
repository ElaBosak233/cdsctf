import { create } from "zustand";

interface SharedState {
    refresh: number;
    setRefresh: () => void;
}

export const useSharedStore = create<SharedState>()((set, get) => ({
    refresh: 0,
    setRefresh: () => set({ refresh: get().refresh + 1 }),
}));
