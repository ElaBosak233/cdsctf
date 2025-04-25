import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { useGameStore } from "@/storages/game";
import { Rss } from "lucide-react";
import { Separator } from "@/components/ui/separator";
import { getGameNotice } from "@/api/games/game_id/notices";
import { useState } from "react";
import { GameNotice } from "@/models/game_notice";
import { useInterval } from "@/hooks/use-interval";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { MarkdownRender } from "@/components/utils/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";

function NoticeCard() {
    const { currentGame } = useGameStore();
    const [gameNotices, setGameNotices] = useState<Array<GameNotice>>();

    function fetchNotices() {
        getGameNotice({
            game_id: currentGame?.id,
        }).then((res) => {
            setGameNotices(res.data);
        });
    }

    useInterval(fetchNotices, 1000 * 30, [currentGame?.id], {
        immediate: true,
    });

    return (
        <>
            <Card
                className={cn([
                    "p-5",
                    "flex",
                    "flex-col",
                    "flex-1",
                    "gap-5",
                    "overflow-auto",
                ])}
            >
                <div
                    className={cn([
                        "flex",
                        "gap-3",
                        "items-center",
                        "select-none",
                    ])}
                >
                    <Rss className={cn(["size-4"])} />
                    <h3 className={cn(["text-sm"])}>公告栏</h3>
                </div>
                <ScrollArea>
                    <div
                        className={cn([
                            "overflow-auto",
                            "flex-1",
                            "flex",
                            "flex-col",
                            "gap-3",
                        ])}
                    >
                        {gameNotices?.map((gameNotice) => (
                            <Dialog>
                                <DialogTrigger>
                                    <Card
                                        className={cn([
                                            "flex",
                                            "flex-col",
                                            "gap-1",
                                            "p-3",
                                            "cursor-pointer",
                                            "hover:bg-foreground/10",
                                            "select-none",
                                        ])}
                                    >
                                        <h4 className={cn(["text-sm"])}>
                                            {gameNotice?.title}
                                        </h4>
                                        <span
                                            className={cn([
                                                "text-secondary-foreground",
                                                "text-xs",
                                            ])}
                                        >
                                            {new Date(
                                                Number(gameNotice?.created_at) *
                                                    1000
                                            ).toLocaleString()}
                                        </span>
                                    </Card>
                                </DialogTrigger>
                                <DialogContent>
                                    <Card
                                        className={cn([
                                            "p-6",
                                            "min-h-81",
                                            "w-screen",
                                            "md:w-xl",
                                            "flex",
                                            "flex-col",
                                            "gap-5",
                                        ])}
                                    >
                                        <div
                                            className={cn(
                                                "flex",
                                                "flex-col",
                                                "gap-3"
                                            )}
                                        >
                                            <div
                                                className={cn([
                                                    "flex",
                                                    "justify-between",
                                                    "items-baseline",
                                                ])}
                                            >
                                                <div
                                                    className={cn([
                                                        "flex",
                                                        "gap-3",
                                                        "items-center",
                                                    ])}
                                                >
                                                    <Rss
                                                        className={cn([
                                                            "size-5",
                                                        ])}
                                                    />
                                                    <h3>{gameNotice?.title}</h3>
                                                </div>
                                                <span
                                                    className={cn([
                                                        "text-secondary-foreground",
                                                        "text-xs",
                                                    ])}
                                                >
                                                    {new Date(
                                                        Number(
                                                            gameNotice?.created_at
                                                        ) * 1000
                                                    ).toLocaleString()}
                                                </span>
                                            </div>
                                            <Separator />
                                        </div>
                                        <div
                                            className={cn([
                                                "flex",
                                                "flex-1",
                                                "flex-col",
                                                "max-h-144",
                                                "overflow-auto",
                                            ])}
                                        >
                                            <MarkdownRender
                                                src={gameNotice?.content}
                                            />
                                        </div>
                                    </Card>
                                </DialogContent>
                            </Dialog>
                        ))}
                    </div>
                </ScrollArea>
            </Card>
        </>
    );
}

export { NoticeCard };
