import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  ArrowRightIcon,
  FlagIcon,
  PackageOpenIcon,
  SearchIcon,
  XIcon,
} from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useOutletContext } from "react-router";
import { type GetGameRequest, getGames } from "@/api/games";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Field, FieldButton, FieldIcon } from "@/components/ui/field";
import { Image } from "@/components/ui/image";
import { Pagination } from "@/components/ui/pagination";
import { TextField } from "@/components/ui/text-field";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameSummary } from "@/models/game";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

type GameStatus = "upcoming" | "ongoing" | "ended";

const GAME_STATUS_STYLES: Record<
  GameStatus,
  { badge: string; icon: string; dot: string; ring: string }
> = {
  upcoming: {
    badge: "bg-info text-info-foreground",
    icon: "text-info",
    dot: "bg-info",
    ring: "ring-info/20",
  },
  ongoing: {
    badge: "bg-success text-success-foreground",
    icon: "text-success",
    dot: "bg-success",
    ring: "ring-success/20",
  },
  ended: {
    badge: "bg-error text-error-foreground",
    icon: "text-error",
    dot: "bg-error",
    ring: "ring-error/20",
  },
};

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
      games: response.games || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

function getGameStatus(game: GameSummary, nowSeconds: number): GameStatus {
  if (game.started_at != null && nowSeconds < game.started_at) {
    return "upcoming";
  }
  if (game.ended_at != null && nowSeconds > game.ended_at) {
    return "ended";
  }
  return "ongoing";
}

function formatTimestamp(
  timestamp: number | undefined,
  formatter: Intl.DateTimeFormat
) {
  if (timestamp == null) return "-";
  return formatter.format(new Date(timestamp * 1000));
}

export default function Index() {
  const { config } = useConfigStore();
  const { t, i18n } = useTranslation();

  const { setEntranceGame } = useOutletContext<{
    setEntranceGame: (game: GameSummary) => void;
  }>();

  const [title, setTitle] = useQueryState("title");
  const debouncedTitle = useDebounce(title, 500);
  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const size = 6;

  const { data: { games, total } = { games: [], total: 0 } } = useGameQuery({
    title: debouncedTitle || undefined,
    page,
    size,
    sorts: "-started_at",
  });

  const [selectedGame, setSelectedGame] = useState<GameSummary>();
  const totalPages = Math.ceil(total / size);
  const language = i18n.resolvedLanguage ?? i18n.language;
  const nowSeconds = Date.now() / 1000;
  const listDateFormatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        year: "numeric",
        month: "short",
        day: "numeric",
      }),
    [language]
  );
  const detailDateFormatter = useMemo(
    () =>
      new Intl.DateTimeFormat(language, {
        year: "numeric",
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      }),
    [language]
  );

  useEffect(() => {
    setSelectedGame(games[0]);
  }, [games]);

  const selectedStatus = selectedGame
    ? getGameStatus(selectedGame, nowSeconds)
    : undefined;
  const selectedStatusStyle = selectedStatus
    ? GAME_STATUS_STYLES[selectedStatus]
    : undefined;

  const handleSearchChange = (value: string) => {
    setTitle(value || null);
    if (page !== 1) setPage(1);
  };

  const handleClearSearch = () => {
    setTitle(null);
    if (page !== 1) setPage(1);
  };

  return (
    <>
      <title>{`${t("game:_")} - ${config?.meta?.title}`}</title>
      <div
        className={cn([
          "mx-auto",
          "grid",
          "w-full",
          "max-w-[1180px]",
          "min-h-0",
          "flex-1",
          "grid-cols-1",
          "gap-6",
          "px-4",
          "py-6",
          "sm:px-6",
          "lg:h-(--app-content-height)",
          "lg:grid-cols-[minmax(17rem,21rem)_minmax(0,1fr)]",
          "lg:grid-rows-[auto_minmax(0,1fr)]",
          "lg:gap-x-16",
          "lg:gap-y-4",
          "lg:px-8",
          "lg:py-12",
          "xl:px-0",
        ])}
      >
        <div
          className={cn([
            "flex",
            "min-w-0",
            "flex-col",
            "gap-3",
            "lg:col-start-1",
            "lg:row-start-1",
          ])}
        >
          <Field className={cn(["w-full"])}>
            <FieldIcon>
              <SearchIcon />
            </FieldIcon>
            <TextField
              aria-label={t("game:search.title")}
              placeholder={t("game:search.title")}
              value={title ?? ""}
              onChange={(event) => handleSearchChange(event.target.value)}
            />
            {!!title && (
              <FieldButton
                variant={"ghost"}
                onClick={handleClearSearch}
                aria-label={t("game:search.clear")}
              >
                <XIcon />
              </FieldButton>
            )}
          </Field>
          <div
            className={cn([
              "flex",
              "items-center",
              "justify-between",
              "px-1",
              "text-xs",
              "text-muted-foreground",
            ])}
          >
            <span>{t("game:result_count", { count: total })}</span>
            {totalPages > 1 && (
              <span className={cn(["font-mono", "tabular-nums"])}>
                {`${page} / ${totalPages}`}
              </span>
            )}
          </div>
        </div>

        <section
          className={cn([
            "flex",
            "flex-col",
            "items-center",
            "min-w-0",
            "lg:col-start-2",
            "lg:row-start-1",
            "lg:row-span-2",
            "lg:min-h-0",
            "lg:flex",
            "lg:items-center",
            "lg:justify-center",
            "lg:overflow-y-auto",
          ])}
          aria-labelledby={selectedGame?.id ? "selected-game-title" : undefined}
        >
          {selectedGame ? (
            <div
              className={cn([
                "flex",
                "mx-auto",
                "w-full",
                "max-w-2xl",
                "min-w-0",
                "flex-col",
                "items-center",
                "text-center",
              ])}
            >
              <div className={cn(["mx-auto", "w-full"])}>
                {selectedGame.poster_hash ? (
                  <Image
                    key={selectedGame.id}
                    src={`/api/media?hash=${selectedGame.poster_hash}`}
                    alt={selectedGame.title}
                    fallback={
                      <FlagIcon
                        className={cn(["size-12", "text-muted-foreground"])}
                        strokeWidth={1.25}
                      />
                    }
                    className={cn([
                      "aspect-video",
                      "mx-auto",
                      "w-full",
                      "rounded-elevated",
                      "border",
                      "bg-card/50",
                      "shadow-sm",
                    ])}
                  />
                ) : (
                  <div
                    className={cn([
                      "flex",
                      "w-full",
                      "aspect-video",
                      "items-center",
                      "justify-center",
                      "rounded-elevated",
                      "border",
                      "bg-card/50",
                      "shadow-sm",
                    ])}
                  >
                    <FlagIcon
                      className={cn(["size-16", "text-secondary-foreground"])}
                      strokeWidth={1}
                    />
                  </div>
                )}
              </div>

              <div
                className={cn([
                  "mt-6",
                  "flex",
                  "w-full",
                  "flex-col",
                  "items-center",
                  "gap-3",
                ])}
              >
                {selectedStatus && (
                  <Badge
                    className={cn([selectedStatusStyle?.badge])}
                    size={"sm"}
                  >
                    {t(`game:status.${selectedStatus}._`)}
                  </Badge>
                )}
                <h2
                  id="selected-game-title"
                  className={cn([
                    "max-w-full",
                    "break-words",
                    "text-2xl",
                    "font-semibold",
                  ])}
                >
                  {selectedGame.title}
                </h2>
                <div
                  className={cn([
                    "flex",
                    "min-h-12",
                    "w-full",
                    "items-center",
                    "justify-center",
                  ])}
                >
                  {selectedGame.sketch && (
                    <p
                      className={cn([
                        "max-w-xl",
                        "line-clamp-2",
                        "text-sm",
                        "leading-relaxed",
                        "text-secondary-foreground",
                      ])}
                    >
                      {selectedGame.sketch}
                    </p>
                  )}
                </div>
                <div
                  className={cn([
                    "flex",
                    "max-w-full",
                    "flex-wrap",
                    "items-center",
                    "justify-center",
                    "gap-2",
                    "text-xs",
                    "text-secondary-foreground",
                  ])}
                >
                  <span>
                    {formatTimestamp(
                      selectedGame.started_at,
                      detailDateFormatter
                    )}
                  </span>
                  <ArrowRightIcon className={cn(["size-3.5"])} />
                  <span>
                    {formatTimestamp(
                      selectedGame.ended_at,
                      detailDateFormatter
                    )}
                  </span>
                </div>
                <Button
                  className={cn(["mx-auto", "mt-2", "w-full", "max-w-sm"])}
                  size={"lg"}
                  variant={"solid"}
                  onClick={() => setEntranceGame(selectedGame)}
                >
                  {t("game:actions.enter")}
                  <ArrowRightIcon />
                </Button>
              </div>
            </div>
          ) : (
            <div
              className={cn([
                "flex",
                "min-h-48",
                "flex-col",
                "items-center",
                "justify-center",
                "gap-3",
                "text-sm",
                "text-muted-foreground",
                "lg:h-full",
              ])}
            >
              <PackageOpenIcon className={cn(["size-8", "opacity-50"])} />
              <span>{t("game:empty")}</span>
            </div>
          )}
        </section>

        <div
          className={cn([
            "flex",
            "min-h-0",
            "flex-col",
            "gap-4",
            "lg:col-start-1",
            "lg:row-start-2",
          ])}
        >
          <div
            className={cn([
              "flex",
              "min-h-0",
              "flex-col",
              "gap-2",
              "lg:flex-1",
              "lg:overflow-y-auto",
              "lg:pr-1",
            ])}
          >
            {games.map((game) => {
              const status = getGameStatus(game, nowSeconds);
              const statusStyle = GAME_STATUS_STYLES[status];
              const selected = selectedGame?.id === game.id;
              const gameMarkClass = cn([
                "flex",
                "size-full",
                "items-center",
                "justify-center",
                statusStyle.icon,
              ]);
              const gamePlaceholder = (
                <span className={gameMarkClass}>
                  <FlagIcon className={cn(["size-4"])} fill="currentColor" />
                </span>
              );

              return (
                <Button
                  key={game.id}
                  className={cn([
                    "h-auto",
                    "min-h-16",
                    "w-full",
                    "justify-start",
                    "gap-3",
                    "whitespace-normal",
                    "px-3",
                    "py-2.5",
                    "text-left",
                  ])}
                  variant={selected ? "tonal" : "ghost"}
                  onClick={() => setSelectedGame(game)}
                  aria-pressed={selected}
                >
                  <div
                    className={cn([
                      "flex",
                      "size-9",
                      "shrink-0",
                      "items-center",
                      "justify-center",
                      "overflow-hidden",
                      "rounded-md",
                    ])}
                  >
                    {game.icon_hash ? (
                      <Image
                        src={`/api/media?hash=${game.icon_hash}`}
                        alt=""
                        delay={0}
                        glass={false}
                        fallback={gamePlaceholder}
                        className={cn(["size-full", "rounded-md"])}
                      />
                    ) : (
                      gamePlaceholder
                    )}
                  </div>
                  <div className={cn(["min-w-0", "flex-1"])}>
                    <h3
                      className={cn(["truncate", "text-sm", "font-semibold"])}
                    >
                      {game.title}
                    </h3>
                    <p
                      className={cn([
                        "mt-1",
                        "truncate",
                        "text-xs",
                        "text-muted-foreground",
                      ])}
                    >
                      {`${formatTimestamp(game.started_at, listDateFormatter)} - ${formatTimestamp(game.ended_at, listDateFormatter)}`}
                    </p>
                  </div>
                  <span
                    className={cn([
                      "size-2",
                      "shrink-0",
                      "self-center",
                      "rounded-full",
                      statusStyle.dot,
                      status === "ongoing" && [
                        "ring-4",
                        statusStyle.ring,
                        "motion-safe:animate-pulse",
                      ],
                    ])}
                    role="img"
                    aria-label={t(`game:status.${status}._`)}
                    title={t(`game:status.${status}._`)}
                  ></span>
                </Button>
              );
            })}
          </div>

          {totalPages > 1 && (
            <Pagination
              className={cn(["shrink-0", "self-center"])}
              size={"sm"}
              total={totalPages}
              max={5}
              value={page}
              onChange={setPage}
            />
          )}
        </div>
      </div>
    </>
  );
}
