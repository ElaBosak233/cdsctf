import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  ChevronRightIcon,
  ListOrderedIcon,
  MedalIcon,
  MessageCircleDashedIcon,
  MoonIcon,
  StarIcon,
  TrophyIcon,
} from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { getGameScoreboard } from "@/api/games/game_id";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { Pagination } from "@/components/ui/pagination";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

import { ChampionChart } from "./_blocks/champion-chart";
import { TeamDetailsDialog } from "./_blocks/team-details-dialog";

export default function Index() {
  const { t } = useTranslation();

  const { currentGame } = useGameStore();
  const [size, setSize] = useState<number>(10);
  const [page, setPage] = useState<number>(1);

  const { data: scoreboardData } = useQuery({
    queryKey: ["scoreboard", currentGame?.id, size, page],
    queryFn: () =>
      getGameScoreboard({
        id: currentGame?.id,
        size,
        page,
      }),
    select: (response) => ({
      scoreboard: response.records || [],
      total: response.total || 0,
    }),
    placeholderData: keepPreviousData,
    enabled: !!currentGame?.id && !currentGame.blacked_out,
  });

  if (currentGame?.blacked_out) {
    return (
      <>
        <title>{`${t("game:scoreboard._")} - ${currentGame.title}`}</title>
        <div
          className={cn([
            "flex",
            "flex-1",
            "flex-col",
            "items-center",
            "justify-center",
            "gap-5",
            "select-none",
          ])}
        >
          <MoonIcon className="size-16 text-muted-foreground" />
          <span className="text-base font-medium">
            {t("game:scoreboard.blackout")}
          </span>
        </div>
      </>
    );
  }

  return (
    <>
      <title>{`${t("game:scoreboard._")} - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "xl:mx-60",
          "mx-10",
          "my-10",
          "flex",
          "flex-col",
          "gap-10",
          "items-center",
          "flex-1",
          "min-h-0",
        ])}
      >
        {scoreboardData?.total ? (
          <>
            <ChampionChart scoreboard={scoreboardData?.scoreboard} />
            <div className={cn(["flex", "items-center", "gap-10", "w-full"])}>
              <div className="flex-1 text-sm text-muted-foreground">
                {scoreboardData?.scoreboard.length} / {scoreboardData?.total}
              </div>
              <Field size={"sm"} className={cn(["w-48"])}>
                <FieldIcon>
                  <ListOrderedIcon />
                </FieldIcon>
                <Select
                  value={String(size)}
                  onValueChange={(value) => setSize(Number(value))}
                >
                  <SelectTrigger>
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
                value={page}
                onChange={(value) => setPage(value)}
                total={Math.ceil((scoreboardData?.total || 0) / size)}
              />
            </div>
            <div
              className={cn([
                "flex-1",
                "min-h-0",
                "flex",
                "flex-col",
                "w-full",
              ])}
            >
              <div className={cn(["flex", "flex-col", "gap-4", "w-full"])}>
                {scoreboardData.scoreboard.map((record) => (
                  <Dialog key={record.team?.id}>
                    <DialogTrigger
                      render={
                        <Card
                          render={<button type="button" />}
                          className={cn([
                            "group",
                            "flex",
                            "items-center",
                            "gap-3",
                            "p-4",
                            "sm:gap-4",
                            "sm:p-5",
                            "cursor-pointer",
                            "border-border/70",
                            "hover:border-primary/30",
                            "hover:bg-muted/40",
                            "hover:shadow-sm",
                            "focus-visible:outline-none",
                            "focus-visible:ring-2",
                            "focus-visible:ring-ring",
                            "focus-visible:ring-offset-2",
                            "transition-[background-color,border-color,box-shadow]",
                            "w-full",
                          ])}
                        >
                          {record.team?.rank != null &&
                          record.team.rank <= 3 ? (
                            <span
                              className={cn(
                                [
                                  "flex",
                                  "size-9",
                                  "shrink-0",
                                  "items-center",
                                  "justify-center",
                                  "rounded-full",
                                  "bg-muted",
                                  "text-muted-foreground",
                                ],
                                record.team.rank === 1 &&
                                  "bg-warning/15 text-warning",
                                record.team.rank === 3 &&
                                  "bg-warning/10 text-warning/80"
                              )}
                            >
                              <span className="sr-only">
                                {record.team.rank}
                              </span>
                              {record.team.rank === 1 ? (
                                <TrophyIcon
                                  className="size-5"
                                  aria-hidden="true"
                                />
                              ) : (
                                <MedalIcon
                                  className="size-5"
                                  aria-hidden="true"
                                />
                              )}
                            </span>
                          ) : (
                            <span
                              className={cn([
                                "flex",
                                "size-9",
                                "shrink-0",
                                "items-center",
                                "justify-center",
                                "rounded-full",
                                "bg-muted",
                                "font-mono",
                                "text-sm",
                                "font-semibold",
                                "tabular-nums",
                                "text-muted-foreground",
                              ])}
                            >
                              {record.team?.rank}
                            </span>
                          )}

                          <Avatar
                            className={cn([
                              "size-11",
                              "shrink-0",
                              "rounded-full",
                              "ring-1",
                              "ring-border/70",
                            ])}
                            src={
                              record.team?.avatar_hash &&
                              `/api/media?hash=${record.team?.avatar_hash}`
                            }
                            fallback={record.team?.name?.charAt(0)}
                          />

                          <div
                            className={cn([
                              "flex",
                              "flex-col",
                              "items-start",
                              "flex-1",
                              "min-w-0",
                              "text-left",
                            ])}
                          >
                            <span
                              className={cn([
                                "truncate",
                                "w-full",
                                "font-semibold",
                                "text-base",
                              ])}
                            >
                              {record.team?.name}
                            </span>
                            <span
                              className={cn([
                                "text-sm",
                                "text-muted-foreground",
                                "truncate",
                              ])}
                            >
                              {record.team?.slogan}
                            </span>
                          </div>

                          <Badge
                            variant="tonal"
                            size="md"
                            className={cn([
                              "font-mono",
                              "flex",
                              "gap-1.5",
                              "items-center",
                              "shrink-0",
                              "px-3",
                              "bg-warning/10",
                              "text-warning",
                              "group-hover:bg-warning/15",
                            ])}
                          >
                            <StarIcon className={cn(["size-4"])} />
                            {record.team?.pts}
                          </Badge>
                          <ChevronRightIcon
                            className={cn([
                              "size-4",
                              "shrink-0",
                              "text-muted-foreground/60",
                              "transition-transform",
                              "group-hover:translate-x-0.5",
                              "group-hover:text-foreground",
                            ])}
                            aria-hidden="true"
                          />
                        </Card>
                      }
                    />
                    <DialogContent size="preview">
                      <TeamDetailsDialog team={record.team!} />
                    </DialogContent>
                  </Dialog>
                ))}
              </div>
            </div>
          </>
        ) : (
          <div
            className={cn([
              "flex",
              "flex-col",
              "items-center",
              "justify-center",
              "flex-1",
              "gap-5",
              "select-none",
            ])}
          >
            <MessageCircleDashedIcon className={cn(["size-12"])} />
            <span>{t("game:scoreboard.empty")}</span>
          </div>
        )}
      </div>
    </>
  );
}
