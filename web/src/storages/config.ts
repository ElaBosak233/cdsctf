import { Config, Version } from "@/models/config";
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

export interface ConfigState {
    config?: Config;
    setConfig: (config: ConfigState["config"]) => void;

    version?: Version;
    setVersion: (version: Version) => void;
}

export const useConfigStore = create<ConfigState>()(
    persist(
        (set, _get) => ({
            setConfig: (config) => set({ config }),
            setVersion: (version) => set({ version }),
        }),
        {
            name: "config",
            storage: createJSONStorage(() => localStorage),
        }
    )
);
