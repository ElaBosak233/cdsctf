import { Link, Outlet, useLocation, useParams } from "react-router";
import { Context } from "./context";
import { useEffect, useMemo, useState } from "react";
import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import {
    LibraryIcon,
    InfoIcon,
    FlagIcon,
    UsersRoundIcon,
    MessageCircleIcon,
} from "lucide-react";
import { useSharedStore } from "@/storages/shared";
import { getGames } from "@/api/admin/games";
import { Game } from "@/models/game";

export default function Layout() {
    const location = useLocation();
    const pathname = location.pathname;
    const sharedStore = useSharedStore();
    const { game_id } = useParams<{ game_id: string }>();
    const [game, setGame] = useState<Game>();

    useEffect(() => {
        if (game_id) {
            getGames({
                id: Number(game_id),
            }).then((res) => {
                if (res.code === 200) {
                    setGame(res?.data?.[0]);
                }
            });
        }
    }, [game_id, sharedStore?.refresh]);

    const options = useMemo(() => {
        return [
            {
                link: `/admin/games/${game_id}`,
                name: "基本信息",
                icon: InfoIcon,
            },
            {
                link: `/admin/games/${game_id}/challenges`,
                name: "题目",
                icon: LibraryIcon,
            },
            {
                link: `/admin/games/${game_id}/teams`,
                name: "团队",
                icon: UsersRoundIcon,
            },
            {
                link: `/admin/games/${game_id}/notices`,
                name: "通知",
                icon: MessageCircleIcon,
            },
        ];
    }, [game]);

    return (
        <Context.Provider value={{ game }}>
            <div
                className={cn([
                    "flex",
                    "flex-col",
                    "xl:flex-row",
                    "flex-1",
                    "gap-10",
                    "xl:mx-30",
                ])}
            >
                <div
                    className={cn([
                        "xl:w-64",
                        "xl:sticky",
                        "xl:top-25",
                        "space-y-6",
                        "h-fit",
                        "my-10",
                        "mx-10",
                        "xl:mx-0",
                    ])}
                >
                    <div
                        className={cn([
                            "flex",
                            "flex-wrap",
                            "justify-center",
                            "gap-3",
                            "select-none",
                        ])}
                    >
                        <FlagIcon />
                        比赛编辑
                    </div>
                    <Card className={cn(["flex", "flex-col", "p-5", "gap-3"])}>
                        {options?.map((option, index) => {
                            return (
                                <Button
                                    key={index}
                                    icon={option?.icon}
                                    variant={
                                        pathname === option?.link
                                            ? "tonal"
                                            : "ghost"
                                    }
                                    className={cn(["justify-start"])}
                                    asChild
                                >
                                    <Link to={option?.link}>
                                        {option?.name}
                                    </Link>
                                </Button>
                            );
                        })}
                    </Card>
                </div>
                <Card
                    className={cn([
                        "flex-1",
                        "p-10",
                        "border-y-0",
                        "rounded-none",
                        "flex",
                        "flex-col",
                    ])}
                >
                    <Outlet />
                </Card>
            </div>
        </Context.Provider>
    );
}
