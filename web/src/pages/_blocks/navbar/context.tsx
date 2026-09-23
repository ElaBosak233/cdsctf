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
import { isGameActive } from "@/utils/time";

export const Context = createContext<{
  mode: "default" | "game";
}>({
  mode: "default",
});

export function useOptions() {
  const { mode } = useContext(Context);
  const { currentGame, selfTeam } = useGameStore();

  const { t } = useTranslation();
  const gameActive = isGameActive(currentGame ?? {});

  const options = useMemo(() => {
    switch (mode) {
      case "game":
        return [
          {
            link: `/games/${currentGame?.id}`,
            name: t("common:home"),
            icon: <HouseIcon />,
          },
          {
            link: `/games/${currentGame?.id}/team`,
            name: t("team:_"),
            icon: <UsersRoundIcon />,
            disabled: !selfTeam?.id,
          },
          {
            link: `/games/${currentGame?.id}/challenges`,
            name: t("challenge:_"),
            icon: <StarIcon />,
            disabled:
              selfTeam?.state !== State.Passed ||
              currentGame?.paused ||
              !gameActive,
          },
          {
            link: `/games/${currentGame?.id}/scoreboard`,
            name: t("game:scoreboard._"),
            icon: <ChartNoAxesCombinedIcon />,
          },
          {
            link: `/games`,
            name: t("common:exit"),
            icon: <LogOutIcon />,
            warning: true,
          },
        ];
      default:
        return [
          {
            link: "/",
            name: t("common:home"),
            icon: <HouseIcon />,
          },
          {
            link: "/playground",
            name: t("challenge:playground"),
            icon: <LibraryIcon />,
          },
          {
            link: "/games",
            name: t("game:_"),
            icon: <FlagIcon />,
          },
        ];
    }
  }, [
    mode,
    currentGame?.id,
    currentGame?.paused,
    gameActive,
    selfTeam?.id,
    selfTeam?.state,
    t,
  ]);

  return options;
}
