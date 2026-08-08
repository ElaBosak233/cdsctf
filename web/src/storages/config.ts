import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

import type { PublicConfig, Version } from "@/models/config";

export type ConfigState = {
  config?: PublicConfig;
  setConfig: (config: ConfigState["config"]) => void;

  version?: Version;
  setVersion: (version: Version) => void;
};

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
