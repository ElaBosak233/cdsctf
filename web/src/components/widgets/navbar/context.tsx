import {
  ChartNoAxesCombinedIcon,
  FlagIcon,
  HouseIcon,
  LibraryIcon,
  LogOutIcon,
  StarIcon,
  UsersRoundIcon,
} from "lucide-react";
import { createContext, useContext, useMemo } from "react";
import { useTranslation } from "react-i18next";

import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";

export const Context = createContext<{
  mode: "default" | "game";
}>({
  mode: "default",
});

export function useOptions() {
  const { mode } = useContext(Context);
  const { currentGame, selfTeam } = useGameStore();

  const { t } = useTranslation();

  const options = useMemo(() => {
    switch (mode) {
      case "game":
        return [
          {
            link: `/games/${currentGame?.id}`,
            name: t("common.home"),
            icon: <HouseIcon />,
          },
          {
            link: `/games/${currentGame?.id}/team`,
            name: t("team._"),
            icon: <UsersRoundIcon />,
            disabled: !selfTeam?.id,
          },
          {
            link: `/games/${currentGame?.id}/challenges`,
            name: t("challenge._"),
            icon: <StarIcon />,
            disabled:
              selfTeam?.state !== State.Passed ||
              new Date(Number(currentGame?.ended_at) * 1000) < new Date() ||
              new Date(Number(currentGame?.started_at) * 1000) > new Date(),
          },
          {
            link: `/games/${currentGame?.id}/scoreboard`,
            name: t("game.scoreboard._"),
            icon: <ChartNoAxesCombinedIcon />,
          },
          {
            link: `/games`,
            name: t("common.exit"),
            icon: <LogOutIcon />,
            warning: true,
          },
        ];
      default:
        return [
          {
            link: "/",
            name: t("common.home"),
            icon: <HouseIcon />,
          },
          {
            link: "/playground",
            name: t("challenge.playground"),
            icon: <LibraryIcon />,
          },
          {
            link: "/games",
            name: t("game._"),
            icon: <FlagIcon />,
          },
        ];
    }
  }, [
    mode,
    currentGame?.id,
    currentGame?.started_at,
    currentGame?.ended_at,
    selfTeam?.id,
    selfTeam?.state,
    t,
  ]);

  return options;
}
