import { ChartNoAxesCombined, FilePenIcon, Star } from "lucide-react";
import { Link } from "react-router";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

function TeamCard() {
  const { currentGame, selfTeam } = useGameStore();

  return (
    <Card className={cn(["p-5", "flex", "flex-col", "items-center"])}>
      <div
        className={cn([
          "flex",
          "flex-col",
          "items-center",
          "gap-3",
          "justify-center",
          "select-none",
          "w-full",
        ])}
      >
        <div
          className={cn([
            "flex",
            "justify-center",
            "w-full",
            "items-center",
            "gap-5",
            "max-w-full",
            "overflow-hidden",
          ])}
        >
          <Avatar
            className={cn(["w-16", "h-16", "flex-shrink-0"])}
            src={
              selfTeam?.has_avatar &&
              `/api/games/${currentGame?.id}/teams/${selfTeam?.id}/avatar`
            }
            fallback={selfTeam?.name?.charAt(0)}
          />
          <div className={cn(["flex", "flex-col", "min-w-0"])}>
            <p className={cn(["text-lg", "truncate"])}>{selfTeam?.name}</p>
            <p
              className={cn([
                "truncate",
                "text-xs",
                "text-secondary-foreground",
                "max-w-full",
              ])}
            >
              {`# ${selfTeam?.id?.toString(16).padStart(6, "0")}`}
            </p>
          </div>
        </div>

        <div className={cn(["flex", "gap-3", "w-full", "justify-center"])}>
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

        {currentGame?.is_need_write_up && (
          <>
            <Separator className={"w-full"} />
            <div
              className={cn([
                "flex",
                "gap-3",
                "items-center",
                "select-none",
                "justify-between",
              ])}
            >
              <Button
                className={cn([
                  "flex",
                  "gap-3",
                  "items-center",
                  "text-secondary-foreground",
                ])}
                size={"sm"}
                icon={<FilePenIcon />}
                asChild
              >
                <Link to={`/games/${selfTeam?.game_id}/team/writeup`}>
                  {selfTeam?.has_write_up
                    ? "你已提交 Write-up"
                    : "你还未提交 Write-up"}
                </Link>
              </Button>
            </div>
          </>
        )}
      </div>
    </Card>
  );
}

export { TeamCard };
