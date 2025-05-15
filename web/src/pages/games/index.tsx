import { getGames } from "@/api/games";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Image } from "@/components/ui/image";
import { Field, FieldIcon } from "@/components/ui/field";
import { TextField } from "@/components/ui/text-field";
import { Pagination } from "@/components/ui/pagination";
import { useDebounce } from "@/hooks/use-debounce";
import { GameMini } from "@/models/game";
import { cn } from "@/utils";
import { FlagIcon, PackageOpenIcon, SearchIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { useConfigStore } from "@/storages/config";
import { ScrollArea } from "@/components/ui/scroll-area";

export default function Index() {
  const configStore = useConfigStore();
  const navigate = useNavigate();

  const [games, setGames] = useState<Array<GameMini>>();
  const [total, setTotal] = useState<number>(0);
  const [title, setTitle] = useState<string>("");
  const debouncedTitle = useDebounce(title, 500);
  const [page, setPage] = useState<number>(1);
  const [size, _setSize] = useState<number>(10);

  const [selectedGame, setSelectedGame] = useState<GameMini>();

  function fetchGames() {
    getGames({
      title: debouncedTitle,
      page,
      size,
      sorts: "-started_at",
    }).then((res) => {
      setTotal(res.total || 0);
      setGames(res.data);
    });
  }

  useEffect(() => {
    fetchGames();
  }, [page, size, debouncedTitle]);

  useEffect(() => {
    if (games) {
      setSelectedGame(games?.[0]);
    }
  }, [games]);

  return (
    <>
      <title>{`比赛 - ${configStore?.config?.meta?.title}`}</title>
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
                        Date.now() / 1000 > game?.ended_at! && "bg-error",
                        Date.now() / 1000 < game?.started_at! && "bg-info",
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
              "shadow-xl",
            ])}
            fallback={
              <FlagIcon
                className={cn(["size-25", "rotate-15"])}
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
                className={cn(["aspect-square", "h-16"])}
                imgClassName={cn(["rounded-full"])}
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
