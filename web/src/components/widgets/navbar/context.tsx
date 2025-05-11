import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
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

export const Context = createContext<{
    mode: "default" | "game";
}>({
    mode: "default",
});

export function useOptions() {
    const { mode } = useContext(Context);
    const { currentGame, selfTeam } = useGameStore();

    const { t, i18n } = useTranslation();

    const options = useMemo(() => {
        switch (mode) {
            case "game":
                return [
                    {
                        link: `/games/${currentGame?.id}`,
                        name: t("home"),
                        icon: <HouseIcon />,
                    },
                    {
                        link: `/games/${currentGame?.id}/team`,
                        name: "团队",
                        icon: <UsersRoundIcon />,
                        disabled: !selfTeam?.id,
                    },
                    {
                        link: `/games/${currentGame?.id}/challenges`,
                        name: "题目",
                        icon: <StarIcon />,
                        disabled:
                            selfTeam?.state !== State.Passed ||
                            new Date(Number(currentGame?.ended_at) * 1000) <
                                new Date() ||
                            new Date(Number(currentGame?.started_at) * 1000) >
                                new Date(),
                    },
                    {
                        link: `/games/${currentGame?.id}/scoreboard`,
                        name: "积分榜",
                        icon: <ChartNoAxesCombinedIcon />,
                    },
                    {
                        link: `/games`,
                        name: "退出",
                        icon: <LogOutIcon />,
                        warning: true,
                    },
                ];
            case "default":
            default:
                return [
                    {
                        link: "/",
                        name: t("home"),
                        icon: <HouseIcon />,
                    },
                    {
                        link: "/playground",
                        name: t("playground"),
                        icon: <LibraryIcon />,
                    },
                    {
                        link: "/games",
                        name: t("game"),
                        icon: <FlagIcon />,
                    },
                ];
        }
    }, [mode, currentGame?.id, selfTeam, i18n.language]);

    return options;
}
