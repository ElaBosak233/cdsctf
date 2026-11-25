import { useTranslation } from "react-i18next";
import { Avatar } from "@/components/ui/avatar";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

function TeamCard({
  solvedCount = 0,
  totalCount = 0,
}: {
  solvedCount?: number;
  totalCount?: number;
}) {
  const { t } = useTranslation();
  const { currentGame, selfTeam } = useGameStore();

  if (!selfTeam) return null;

  const progressPercent =
    totalCount > 0 ? Math.round((solvedCount / totalCount) * 100) : 0;
  const isAllCleared = totalCount > 0 && solvedCount === totalCount;

  return (
    <div
      className="flex min-w-0 flex-wrap items-center gap-x-6 gap-y-3"
      aria-labelledby="game-team-summary"
    >
      {/* Team Identity */}
      <div className="flex min-w-0 items-center gap-3">
        <Avatar
          className="size-10 shrink-0"
          src={
            selfTeam?.avatar_hash && `/api/media?hash=${selfTeam?.avatar_hash}`
          }
          fallback={selfTeam?.name?.charAt(0)}
        />
        <div className="min-w-0">
          <h2
            id="game-team-summary"
            className="max-w-44 truncate text-sm font-semibold sm:max-w-60"
            title={selfTeam?.name}
          >
            {selfTeam?.name}
          </h2>
          <p className="mt-0.5 truncate font-mono text-[11px] text-muted-foreground">
            {`# ${selfTeam?.id?.toString(16).padStart(6, "0")}`}
          </p>
        </div>
      </div>

      {/* Metrics Cluster */}
      {(!currentGame?.blacked_out || totalCount > 0) && (
        <>
          <Separator orientation="vertical" className="hidden h-8 sm:block" />
          <div className="flex flex-wrap items-center gap-x-6 gap-y-2">
            {!currentGame?.blacked_out && (
              <>
                <div className="flex flex-col gap-0.5">
                  <span className="text-[11px] leading-none text-muted-foreground">
                    {t("team:pts")}
                  </span>
                  <span className="font-mono text-base font-semibold leading-tight tabular-nums text-foreground">
                    {(selfTeam?.pts ?? 0).toLocaleString()}
                  </span>
                </div>

                <div className="flex flex-col gap-0.5">
                  <span className="text-[11px] leading-none text-muted-foreground">
                    {t("team:rank")}
                  </span>
                  <span className="font-mono text-base font-semibold leading-tight tabular-nums text-foreground">
                    {selfTeam?.rank ? `#${selfTeam.rank}` : "—"}
                  </span>
                </div>
              </>
            )}

            {totalCount > 0 && (
              <div className="flex flex-col gap-1.5">
                <div className="flex items-baseline justify-between gap-3">
                  <span className="text-[11px] leading-none text-muted-foreground">
                    {t("game:filter.progress")}
                  </span>
                  <span
                    className={cn(
                      "font-mono text-xs font-medium leading-none tabular-nums",
                      isAllCleared ? "text-success" : "text-foreground"
                    )}
                  >
                    {solvedCount}
                    <span className="text-muted-foreground">/{totalCount}</span>
                  </span>
                </div>
                <Progress
                  value={progressPercent}
                  aria-label={t("game:filter.progress")}
                  className={cn(
                    "w-24 [&_[data-slot=progress-track]]:h-1.5",
                    isAllCleared &&
                      "[&_[data-slot=progress-indicator]]:bg-success"
                  )}
                />
              </div>
            )}
          </div>
        </>
      )}
    </div>
  );
}

export { TeamCard };
