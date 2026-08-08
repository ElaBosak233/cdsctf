import { ChartNoAxesCombined, FilePenIcon, Star } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link } from "react-router";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

function TeamCard() {
  const { t } = useTranslation();
  const { currentGame, selfTeam } = useGameStore();
  const writeupLabel = selfTeam?.has_writeup
    ? t("team:write_up.actions.submit.done")
    : t("team:write_up.actions.submit._");

  if (!selfTeam) return null;

  return (
    <section
      className={cn([
        "grid",
        "grid-cols-[minmax(0,1fr)_auto]",
        "min-w-0",
        "items-center",
        "gap-x-3",
        "gap-y-2",
        "lg:flex",
        "lg:flex-1",
        "lg:flex-wrap",
        "lg:gap-x-4",
      ])}
      aria-labelledby="game-team-summary"
    >
      <div className={cn(["flex", "min-w-0", "items-center", "gap-2.5"])}>
        <Avatar
          className={cn(["size-9", "shrink-0"])}
          src={
            selfTeam?.avatar_hash && `/api/media?hash=${selfTeam?.avatar_hash}`
          }
          fallback={selfTeam?.name?.charAt(0)}
        />
        <div className={cn(["min-w-0"])}>
          <h2
            id="game-team-summary"
            className={cn(["truncate", "text-sm", "font-semibold"])}
          >
            {selfTeam?.name}
          </h2>
          <p
            className={cn(["truncate", "text-[11px]", "text-muted-foreground"])}
          >
            {`# ${selfTeam?.id?.toString(16).padStart(6, "0")}`}
          </p>
        </div>
      </div>

      <div
        className={cn([
          "flex",
          "items-center",
          "justify-self-end",
          "gap-3",
          "lg:justify-self-auto",
          "lg:gap-4",
        ])}
      >
        {!currentGame?.blacked_out && (
          <>
            <div className={cn(["flex", "items-center", "gap-1.5"])}>
              <Star className={cn(["size-3.5", "text-muted-foreground"])} />
              <span className={cn(["font-mono", "text-sm", "tabular-nums"])}>
                {selfTeam?.pts}
              </span>
              <span className={cn(["text-xs", "text-muted-foreground"])}>
                {t("team:pts")}
              </span>
            </div>
            <div className={cn(["flex", "items-center", "gap-1.5"])}>
              <ChartNoAxesCombined
                className={cn(["size-3.5", "text-muted-foreground"])}
              />
              <span className={cn(["font-mono", "text-sm", "tabular-nums"])}>
                {selfTeam?.rank === 0 ? "-" : selfTeam?.rank}
              </span>
              <span className={cn(["text-xs", "text-muted-foreground"])}>
                {t("team:rank")}
              </span>
            </div>
          </>
        )}
        {currentGame?.writeup_required && (
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                className={cn(["size-9"])}
                size={"sm"}
                square
                variant={"ghost"}
                level={selfTeam?.has_writeup ? "secondary" : "warning"}
                asChild
                aria-label={writeupLabel}
              >
                <Link to={`/games/${selfTeam?.game_id}/team/writeup`}>
                  <FilePenIcon />
                </Link>
              </Button>
            </TooltipTrigger>
            <TooltipContent>{writeupLabel}</TooltipContent>
          </Tooltip>
        )}
      </div>
    </section>
  );
}

export { TeamCard };
