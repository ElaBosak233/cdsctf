import { Button } from "@/components/ui/button";
import {
    House,
    Library,
    Flag,
    LogOut,
    Star,
    ChartNoAxesCombined,
    UsersRoundIcon,
} from "lucide-react";
import { useContext, useMemo } from "react";
import { Link, useLocation } from "react-router";
import { useGameStore } from "@/storages/game";
import { Context } from "./context";
import { State } from "@/models/team";
import { useTranslation } from "react-i18next";

function TabSection() {
    const location = useLocation();
    const pathname = location.pathname;
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
                        icon: House,
                    },
                    {
                        link: `/games/${currentGame?.id}/team`,
                        name: "团队",
                        icon: UsersRoundIcon,
                        disabled: !selfTeam?.id,
                    },
                    {
                        link: `/games/${currentGame?.id}/challenges`,
                        name: "题目",
                        icon: Star,
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
                        icon: ChartNoAxesCombined,
                    },
                    {
                        link: `/games`,
                        name: "退出",
                        icon: LogOut,
                        warning: true,
                    },
                ];
            case "default":
            default:
                return [
                    {
                        link: "/",
                        name: t("home"),
                        icon: House,
                    },
                    {
                        link: "/playground",
                        name: t("playground"),
                        icon: Library,
                    },
                    {
                        link: "/games",
                        name: t("game"),
                        icon: Flag,
                    },
                ];
        }
    }, [mode, currentGame?.id, selfTeam, i18n.language]);

    return (
        <>
            {options?.map((option, index) => {
                const Comp = option?.disabled ? Button : Link;

                return (
                    <Button
                        key={index}
                        asChild
                        variant={pathname === option?.link ? "tonal" : "ghost"}
                        size={"sm"}
                        className={"font-semibold"}
                        disabled={option?.disabled}
                        icon={option.icon}
                        level={option?.warning ? "warning" : "primary"}
                    >
                        <Comp to={option.link}>{option?.name}</Comp>
                    </Button>
                );
            })}
        </>
    );
}

export { TabSection };
