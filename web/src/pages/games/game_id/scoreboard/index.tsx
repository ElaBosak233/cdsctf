import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ListOrderedIcon, MessageCircleDashedIcon, StarIcon } from "lucide-react";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { getGameScoreboard } from "@/api/games/game_id";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { Pagination } from "@/components/ui/pagination";
import { Select } from "@/components/ui/select";
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
    enabled: !!currentGame?.id,
  });

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
                  options={[
                    { value: "10" },
                    { value: "20" },
                    { value: "40" },
                    { value: "60" },
                  ]}
                  value={String(size)}
                  onValueChange={(value) => setSize(Number(value))}
                />
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
                    <DialogTrigger>
                      <Card
                        className={cn([
                          "flex",
                          "items-center",
                          "gap-5",
                          "p-5",
                          "cursor-pointer",
                          "hover:bg-muted/50",
                          "transition-colors",
                          "w-full",
                        ])}
                      >
                        <Badge
                          variant="outline"
                        >
                          {record.team?.rank}
                        </Badge>

                        <Avatar
                          className={cn(["size-10", "shrink-0"])}
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
                            "flex-1",
                            "min-w-0",
                          ])}
                        >
                          <span
                            className={cn(["font-semibold", "text-base"])}
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
                            "px-4",
                          ])}
                        >
                          <StarIcon className={cn(["size-4"])} />
                          {record.team?.pts}
                        </Badge>
                      </Card>
                    </DialogTrigger>
                    <DialogContent>
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
