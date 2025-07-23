import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { FlagIcon, PackageOpenIcon, SearchIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { type GetGameRequest, getGames } from "@/api/games";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Field, FieldIcon } from "@/components/ui/field";
import { Image } from "@/components/ui/image";
import { Pagination } from "@/components/ui/pagination";
import { ScrollArea } from "@/components/ui/scroll-area";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameMini } from "@/models/game";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

function useGameQuery(params: GetGameRequest, trigger: number = 0) {
  return useQuery({
    queryKey: [
      "games",
      trigger,
      params.size,
      params.page,
      params.title,
      params.sorts,
    ],
    queryFn: () => getGames(params),
    select: (response) => ({
      games: response.data || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const { config } = useConfigStore();
  const navigate = useNavigate();

  const [title, setTitle] = useState<string>("");
  const debouncedTitle = useDebounce(title, 500);
  const [page, setPage] = useState<number>(1);
  const [size, _setSize] = useState<number>(10);

  const { data: { games, total } = { games: [], total: 0 } } = useGameQuery({
    title: debouncedTitle,
    page,
    size,
    sorts: "-started_at",
  });

  const [selectedGame, setSelectedGame] = useState<GameMini>();

  useEffect(() => {
    if (games) {
      setSelectedGame(games?.[0]);
    }
  }, [games]);

  return (
    <>
      <title>{`比赛 - ${config?.meta?.title}`}</title>
      <div
        className={cn([
          "w-full",
          "p-10",
          "xl:p-0",
          "flex",
          "flex-col",
          "xl:flex-row",
          "xl:gap-25",
          "gap-10",
          "xl:h-[calc(100vh-64px)]",
          "items-center",
          "justify-center",
        ])}
      >
        <div
          className={cn([
            "flex",
            "h-full",
            "xl:py-16",
            "flex-col",
            "gap-5",
            "w-full",
            "xl:w-90",
            "items-center",
          ])}
        >
          <Field className={cn(["w-full"])}>
            <FieldIcon>
              <SearchIcon />
            </FieldIcon>
            <TextField
              placeholder={"比赛名"}
              value={title}
              onChange={(e) => setTitle(e.target.value)}
            />
          </Field>
          <Pagination
            size={"sm"}
            total={Math.ceil(total / size)}
            max={5}
            value={page}
            onChange={setPage}
          />
          <Card
            className={cn([
              "w-full",
              "flex-1",
              "max-h-128",
              "p-5",
              "select-none",
              "overflow-hidden",
            ])}
            asChild
          >
            <ScrollArea className={cn(["h-full"])}>
              <div className={cn(["space-y-3", "h-full"])}>
                {games?.map((game) => (
                  <Button
                    key={game?.id}
                    className={cn(["justify-start", "w-full"])}
                    variant={selectedGame?.id === game?.id ? "tonal" : "ghost"}
                    onClick={() => setSelectedGame(game)}
                  >
                    <span
                      className={cn([
                        "size-1.5",
                        "rounded-full",
                        "bg-success",
                        Date.now() / 1000 > game.ended_at! && "bg-error",
                        Date.now() / 1000 < game.started_at! && "bg-info",
                      ])}
                      aria-hidden="true"
                    />
                    {game?.title}
                  </Button>
                ))}

                {!games?.length && (
                  <div
                    className={cn([
                      "text-secondary-foreground",
                      "flex",
                      "justify-center",
                      "gap-3",
                      "w-full",
                    ])}
                  >
                    <PackageOpenIcon />
                    好像还没有比赛哦。
                  </div>
                )}
              </div>
            </ScrollArea>
          </Card>
        </div>
        <div className={cn(["relative", "select-none", "w-full", "xl:w-1/2"])}>
          <Image
            src={`/api/games/${selectedGame?.id}/poster`}
            className={cn([
              "object-cover",
              "rounded-xl",
              "overflow-hidden",
              "border",
              "aspect-16/9",
              "w-full",
              "bg-card/50",
              "shadow-sm",
            ])}
            fallback={
              <FlagIcon
                className={cn([
                  "size-25",
                  "rotate-15",
                  "text-secondary-foreground",
                ])}
                strokeWidth={1}
              />
            }
          />
          {selectedGame?.id && (
            <Card
              className={cn([
                "absolute",
                "top-0",
                "left-0",
                "m-6",
                "2xl:top-auto",
                "2xl:left-auto",
                "2xl:-right-20",
                "2xl:-bottom-16",
                "p-4",
                "bg-card/90",
                "backdrop-blur-sm",
                "min-h-24",
                "w-128",
                "max-w-3/4",
                "hover:bg-card/80",
                "cursor-pointer",
                "flex",
                "items-center",
                "gap-3",
                "transition-all",
              ])}
              onClick={() => navigate(`/games/${selectedGame?.id}`)}
            >
              <Image
                src={`/api/games/${selectedGame?.id}/icon`}
                fallback={
                  <FlagIcon
                    className={cn([
                      "rotate-15",
                      "text-secondary-foreground",
                      "size-6",
                    ])}
                  />
                }
                className={cn([
                  "aspect-square",
                  "h-16",
                  "rounded-full",
                  "overflow-hidden",
                ])}
              />
              <div className={cn(["space-y-1", "flex-1", "max-w-100"])}>
                <h2
                  className={cn([
                    "text-xl",
                    "max-w-3/4",
                    "text-ellipsis",
                    "overflow-hidden",
                    "text-nowrap",
                    "font-semibold",
                  ])}
                >
                  {selectedGame?.title}
                </h2>
                <p
                  className={cn([
                    "text-sm",
                    "text-secondary-foreground",
                    "max-w-full",
                    "text-ellipsis",
                    "overflow-hidden",
                    "max-h-24",
                  ])}
                >
                  {selectedGame?.sketch}
                </p>
              </div>
            </Card>
          )}
        </div>
      </div>
    </>
  );
}
