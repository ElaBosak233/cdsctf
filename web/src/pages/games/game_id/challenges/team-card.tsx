import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { Avatar } from "@/components/ui/avatar";
import { useGameStore } from "@/storages/game";
import { ChartNoAxesCombined, Star } from "lucide-react";
import { Separator } from "@/components/ui/separator";

function TeamCard() {
    const { currentGame, selfTeam } = useGameStore();

    return (
        <Card className={cn(["p-5", "flex", "flex-col", "items-center"])}>
            <div
                className={cn([
                    "flex",
                    "flex-col",
                    "items-center",
                    "gap-2",
                    "justify-center",
                    "select-none",
                    "w-full",
                ])}
            >
                <Avatar
                    className={cn(["w-12", "h-12"])}
                    src={`/api/games/${currentGame?.id}/teams/${selfTeam?.id}/avatar`}
                />
                <div
                    className={cn([
                        "flex",
                        "flex-col",
                        "w-full",
                        "items-center",
                    ])}
                >
                    <p
                        className={cn([
                            "max-w-4/5",
                            "text-ellipsis",
                            "text-nowrap",
                            "overflow-hidden",
                            "text-lg",
                        ])}
                    >
                        {selfTeam?.name}
                    </p>
                    <p
                        className={cn([
                            "max-w-1/2",
                            "text-ellipsis",
                            "text-nowrap",
                            "overflow-hidden",
                            "text-xs",
                            "text-secondary-foreground",
                        ])}
                    >
                        {`# ${selfTeam?.id?.toString(16).padStart(6, "0")}`}
                    </p>
                </div>
                <div
                    className={cn([
                        "flex",
                        "gap-3",
                        "w-full",
                        "justify-center",
                    ])}
                >
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                        <Star className={cn(["size-4"])} />
                        <span>{selfTeam?.pts}</span>
                    </div>
                    <Separator orientation={"vertical"} />
                    <div className={cn(["flex", "gap-2", "items-center"])}>
                        <ChartNoAxesCombined className={cn(["size-4"])} />
                        <span>{selfTeam?.rank}</span>
                    </div>
                </div>
            </div>
        </Card>
    );
}

export { TeamCard };
