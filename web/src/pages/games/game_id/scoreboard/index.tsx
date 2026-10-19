import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  ListOrderedIcon,
  MessageCircleDashedIcon,
  MoonIcon,
  RefreshCwIcon,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { getGameScoreboard } from "@/api/games/game_id";
import { Button } from "@/components/ui/button";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Pagination } from "@/components/ui/pagination";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useGameStore } from "@/storages/game";
import { ChampionChart } from "./_blocks/champion-chart";
import { ScoreboardTable } from "./_blocks/scoreboard-table";

export default function Index() {
  const { t } = useTranslation();
  const { currentGame, selfTeam } = useGameStore();
  const [size, setSize] = useState(10);
  const [page, setPage] = useState(1);
  const enabled = !!currentGame?.id && !currentGame.blacked_out;
  const chartQuery = useQuery({
    queryKey: ["scoreboard", currentGame?.id, "chart"],
    queryFn: () => getGameScoreboard({ id: currentGame?.id, size: 6, page: 1 }),
    enabled,
  });
  const scoreboardQuery = useQuery({
    queryKey: ["scoreboard", currentGame?.id, size, page],
    queryFn: () => getGameScoreboard({ id: currentGame?.id, size, page }),
    placeholderData: keepPreviousData,
    enabled,
  });
  const data = scoreboardQuery.data;
  const challenges = data?.challenges ?? [];
  const refresh = () => {
    void scoreboardQuery.refetch();
    void chartQuery.refetch();
  };

  if (currentGame?.blacked_out)
    return (
      <>
        <title>{`${t("game:scoreboard._")} - ${currentGame.title}`}</title>
        <div className="flex flex-1 flex-col items-center justify-center gap-5 select-none">
          <MoonIcon className="size-16 text-muted-foreground" />
          <span className="text-base font-medium">
            {t("game:scoreboard.blackout")}
          </span>
        </div>
      </>
    );
  return (
    <>
      <title>{`${t("game:scoreboard._")} - ${currentGame?.title}`}</title>
      <div className="mx-auto flex w-full max-w-360 min-w-0 flex-1 flex-col gap-6 px-4 py-6 sm:px-6 lg:px-10 lg:py-8">
        {data?.total ? (
          <>
            <ChampionChart scoreboard={chartQuery.data?.records} />
            <section
              className="flex min-w-0 flex-col gap-3"
              aria-labelledby="standings-heading"
            >
              <div className="flex flex-wrap items-center justify-between gap-3">
                <div className="flex items-center gap-3">
                  <h2
                    id="standings-heading"
                    className="text-base font-semibold"
                  >
                    {t("game:scoreboard._")}
                  </h2>
                  <span className="text-xs text-muted-foreground">
                    {t("game:scoreboard.chart.teams", { count: data.total })}
                  </span>
                </div>
                <Button
                  size="sm"
                  variant="ghost"
                  icon={<RefreshCwIcon />}
                  aria-label={t("game:scoreboard.refresh")}
                  title={t("game:scoreboard.refresh")}
                  disabled={scoreboardQuery.isFetching || chartQuery.isFetching}
                  onClick={refresh}
                />
              </div>
              <p className="text-xs text-muted-foreground">
                {t(
                  challenges.length
                    ? "game:scoreboard.matrix_hint"
                    : "game:scoreboard.no_challenges"
                )}
              </p>
              <div
                className="relative min-w-0"
                aria-busy={scoreboardQuery.isFetching}
              >
                <ScoreboardTable
                  records={data.records}
                  challenges={challenges}
                  selfTeamId={selfTeam?.id}
                />
                <LoadingOverlay loading={scoreboardQuery.isFetching} />
              </div>
              <div className="flex flex-wrap items-center justify-between gap-3 text-xs text-muted-foreground">
                <span>
                  {t("game:scoreboard.showing", {
                    from: data.records.length ? (page - 1) * size + 1 : 0,
                    to: (page - 1) * size + data.records.length,
                    total: data.total,
                  })}
                </span>
                <div className="flex flex-wrap items-center gap-3">
                  <Field size="sm" className="w-32 sm:w-36">
                    <FieldIcon>
                      <ListOrderedIcon />
                    </FieldIcon>
                    <Select
                      value={String(size)}
                      onValueChange={(value) => {
                        if (value) {
                          setSize(Number(value));
                          setPage(1);
                        }
                      }}
                    >
                      <SelectTrigger
                        aria-label={t("game:scoreboard.page_size")}
                      >
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {[10, 20, 40, 60].map((value) => (
                          <SelectItem key={value} value={String(value)}>
                            {value}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </Field>
                  <Pagination
                    size="sm"
                    value={page}
                    onChange={setPage}
                    total={Math.ceil(data.total / size)}
                  />
                </div>
              </div>
            </section>
          </>
        ) : (
          <div className="relative flex min-h-80 flex-1 flex-col items-center justify-center gap-5">
            {scoreboardQuery.isPending ? (
              <LoadingOverlay loading />
            ) : scoreboardQuery.isError ? (
              <Button
                variant="outline"
                icon={<RefreshCwIcon />}
                onClick={refresh}
              >
                {t("game:scoreboard.retry")}
              </Button>
            ) : (
              <>
                <MessageCircleDashedIcon className="size-12 text-muted-foreground" />
                <span>{t("game:scoreboard.empty")}</span>
              </>
            )}
          </div>
        )}
      </div>
    </>
  );
}
