import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Image } from "@/components/ui/image";
import { MarkdownRender } from "@/components/utils/markdown-render";
import { State } from "@/models/team";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { ArrowRightIcon, PlayIcon, SwordsIcon } from "lucide-react";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router";
import { TeamGatheringDialog } from "./team-gathering-dialog";

export default function Index() {
    const { currentGame, selfTeam } = useGameStore();
    const navigate = useNavigate();

    const status = useMemo(() => {
        if (!currentGame) return "loading";

        const startedAt = new Date(Number(currentGame.started_at) * 1000);
        const endedAt = new Date(Number(currentGame.ended_at) * 1000);

        if (startedAt > new Date()) return "upcoming";
        if (endedAt < new Date()) return "ended";
        return "ongoing";
    }, [currentGame]);

    const invalidMessage = useMemo(() => {
        if (selfTeam?.state === State.Banned) {
            return "禁赛中";
        } else if (selfTeam?.state === State.Preparing) {
            return "赛前准备中";
        } else if (selfTeam?.state === State.Pending) {
            return "审核中";
        }
        return undefined;
    }, [selfTeam]);

    const [teamGatheringDialogOpen, setTeamGatheringDialogOpen] =
        useState<boolean>(false);

    return (
        <div
            className={cn([
                "w-full",
                "flex",
                "flex-col",
                "lg:px-30",
                "xl:px-60",
                "lg:flex-row",
                "justify-center",
                "gap-12",
            ])}
        >
            <div
                className={cn([
                    "w-full",
                    "lg:sticky",
                    "lg:top-16",
                    "lg:w-2/5",
                    "lg:h-[calc(100vh-64px)]",
                    "flex",
                    "flex-col",
                    "lg:justify-between",
                    "py-10",
                    "px-20",
                    "lg:px-0",
                    "gap-10",
                ])}
            >
                <div
                    className={cn([
                        "flex",
                        "flex-col",
                        "gap-5",
                        "items-center",
                    ])}
                >
                    <Image
                        src={`/api/games/${currentGame?.id}/poster`}
                        className={cn([
                            "object-cover",
                            "rounded-xl",
                            "overflow-hidden",
                            "border",
                            "aspect-16/9",
                            "w-full",
                            "shadow-md",
                            "select-none",
                        ])}
                    />
                    <h2 className={cn(["text-2xl"])}>{currentGame?.title}</h2>
                    <p
                        className={cn([
                            "max-w-3/4",
                            "text-sm",
                            "text-secondary-foreground",
                            "text-ellipsis",
                            "text-nowrap",
                            "whitespace-nowrap",
                            "overflow-hidden",
                        ])}
                    >
                        {currentGame?.sketch}
                    </p>
                    <Badge
                        className={cn(
                            ["bg-info", "text-info-foreground"],
                            status === "ongoing" && [
                                "bg-success",
                                "text-success-foreground",
                            ],
                            status === "ended" && [
                                "bg-error",
                                "text-error-foreground",
                            ],
                            status === "upcoming" && [
                                "bg-info",
                                "text-info-foreground",
                            ]
                        )}
                        size={"sm"}
                    >
                        {new Date(
                            Number(currentGame?.started_at) * 1000
                        ).toLocaleString()}
                        <ArrowRightIcon />
                        {new Date(
                            Number(currentGame?.ended_at) * 1000
                        ).toLocaleString()}
                    </Badge>
                </div>
                <div>
                    {selfTeam ? (
                        <Button
                            className={cn(["w-full"])}
                            variant={"solid"}
                            level={"success"}
                            size={"lg"}
                            icon={PlayIcon}
                            disabled={
                                status !== "ongoing" ||
                                selfTeam.state !== State.Passed
                            }
                            onClick={() =>
                                navigate(`/games/${currentGame?.id}/challenges`)
                            }
                        >
                            <span>作为 {selfTeam?.name} 参赛</span>
                            {invalidMessage && (
                                <span>（{invalidMessage}）</span>
                            )}
                        </Button>
                    ) : (
                        <>
                            <Button
                                variant={"solid"}
                                level={"info"}
                                size={"lg"}
                                className={cn(["w-full"])}
                                icon={SwordsIcon}
                                onClick={() => setTeamGatheringDialogOpen(true)}
                            >
                                集结你的队伍
                            </Button>
                            <Dialog
                                open={teamGatheringDialogOpen}
                                onOpenChange={setTeamGatheringDialogOpen}
                            >
                                <DialogContent>
                                    <TeamGatheringDialog
                                        onClose={() =>
                                            setTeamGatheringDialogOpen(false)
                                        }
                                    />
                                </DialogContent>
                            </Dialog>
                        </>
                    )}
                </div>
            </div>
            <Card
                className={cn([
                    "lg:w-3/5",
                    "min-h-[calc(100vh-64px)]",
                    "p-15",
                    "rounded-none",
                    "border-y-0",
                    "shadow-md",
                ])}
            >
                <MarkdownRender src={currentGame?.description} />
            </Card>
        </div>
    );
}
