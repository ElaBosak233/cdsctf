import { Config } from "@/types/config";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface ConfigState {
    pltCfg: Config;
    setPltCfg: (pltCfg: Config) => void;
    refresh: number;
    setRefresh: (refresh: number) => void;
}

export const useConfigStore = create<ConfigState>()(
    persist(
        (set, _get) => ({
            pltCfg: {
                site: {
                    title: "Cloudsdale",
                    description: "Hack for fun not for profit",
                },
            },
            setPltCfg: (pltCfg) => set({ pltCfg }),
            refresh: 0,
            setRefresh: (refresh) => set({ refresh }),
        }),
        {
            name: "config_storage",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
