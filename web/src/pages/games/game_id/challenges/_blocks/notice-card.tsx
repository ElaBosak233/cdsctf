import { useQuery } from "@tanstack/react-query";
import { ArrowLeftIcon, BellIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { getGameNotice } from "@/api/games/game_id/notices";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Typography } from "@/components/ui/typography";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import {
  getNoticeFingerprints,
  getNoticeScopeKey,
  useNoticeReadStore,
} from "@/storages/notice";
import { cn } from "@/utils";

function NoticeCard() {
  const { t } = useTranslation();
  const { user } = useAuthStore();
  const { currentGame } = useGameStore();
  const [open, setOpen] = useState(false);
  const [selectedNoticeId, setSelectedNoticeId] = useState<number>();
  const gameId = currentGame?.id;
  const userId = user?.id;
  const scopeKey =
    userId != null && gameId != null
      ? getNoticeScopeKey(userId, gameId)
      : undefined;
  const noticeScope = useNoticeReadStore((state) =>
    scopeKey ? state.scopes[scopeKey] : undefined
  );
  const syncNotices = useNoticeReadStore((state) => state.syncNotices);
  const markAsRead = useNoticeReadStore((state) => state.markAsRead);
  const { data: gameNotices } = useQuery({
    queryKey: ["game_notices", gameId],
    queryFn: () =>
      getGameNotice({
        game_id: gameId!,
      }),
    select: (response) => response.notices,
    refetchInterval: 15000,
    enabled: gameId != null,
  });
  const noticeFingerprints = useMemo(
    () => getNoticeFingerprints(gameNotices ?? []),
    [gameNotices]
  );
  const hasUnreadNotices =
    scopeKey != null &&
    noticeFingerprints.some(
      (fingerprint) => noticeScope?.seen[fingerprint] == null
    );

  useEffect(() => {
    if (!scopeKey || !gameNotices) return;

    syncNotices(scopeKey, gameNotices);
  }, [gameNotices, scopeKey, syncNotices]);

  useEffect(() => {
    if (!open || !scopeKey || !gameNotices) return;

    markAsRead(scopeKey, noticeFingerprints);
  }, [gameNotices, markAsRead, noticeFingerprints, open, scopeKey]);

  const selectedNotice = gameNotices?.find(
    (gameNotice) => gameNotice?.id === selectedNoticeId
  );

  const handleOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) setSelectedNoticeId(undefined);
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <Tooltip>
        <TooltipTrigger asChild>
          <DialogTrigger>
            <Button
              className={cn([
                "relative",
                "size-9",
                hasUnreadNotices ? "text-warning" : "text-muted-foreground",
              ])}
              size={"sm"}
              square
              variant={"ghost"}
              aria-label={
                hasUnreadNotices
                  ? `${t("game:notice.board")} - ${t("game:notice.new")}`
                  : t("game:notice.board")
              }
            >
              <BellIcon />
              {hasUnreadNotices && (
                <span
                  className={cn([
                    "absolute",
                    "right-1",
                    "top-1",
                    "flex",
                    "size-3",
                  ])}
                  aria-hidden="true"
                >
                  <span
                    className={cn([
                      "absolute",
                      "inline-flex",
                      "size-full",
                      "rounded-full",
                      "bg-warning/60",
                      "motion-safe:animate-ping",
                    ])}
                  />
                  <span
                    className={cn([
                      "relative",
                      "inline-flex",
                      "size-2.5",
                      "rounded-full",
                      "bg-warning",
                      "ring-2",
                      "ring-card",
                    ])}
                  />
                </span>
              )}
            </Button>
          </DialogTrigger>
        </TooltipTrigger>
        <TooltipContent>{t("game:notice.board")}</TooltipContent>
      </Tooltip>
      <DialogContent
        size={"wide"}
        slotProps={{ title: { children: t("game:notice.board") } }}
      >
        <Card
          className={cn([
            "flex",
            "min-h-64",
            "max-h-[70vh]",
            "w-full",
            "flex-col",
            "gap-4",
            "rounded-elevated",
            "p-5",
            "shadow-lg",
          ])}
        >
          {selectedNotice ? (
            <>
              <div className={cn(["flex", "items-start", "gap-3"])}>
                <Button
                  className={cn(["size-9", "shrink-0"])}
                  size={"sm"}
                  square
                  variant={"ghost"}
                  onClick={() => setSelectedNoticeId(undefined)}
                  aria-label={t("common:actions.cancel")}
                >
                  <ArrowLeftIcon />
                </Button>
                <div className={cn(["min-w-0", "pt-1"])}>
                  <h2 className={cn(["truncate", "text-sm", "font-semibold"])}>
                    {selectedNotice.title}
                  </h2>
                  <p className={cn(["text-xs", "text-muted-foreground"])}>
                    {new Date(
                      Number(selectedNotice.created_at) * 1000
                    ).toLocaleString()}
                  </p>
                </div>
              </div>
              <ScrollArea className={cn(["min-h-0", "flex-1"])}>
                <Typography>
                  <MarkdownRender src={selectedNotice.content} />
                </Typography>
              </ScrollArea>
            </>
          ) : (
            <>
              <div className={cn(["flex", "items-center", "gap-2"])}>
                <BellIcon className={cn(["size-4", "text-primary"])} />
                <h2 className={cn(["text-sm", "font-semibold"])}>
                  {t("game:notice.board")}
                </h2>
                {!!gameNotices?.length && (
                  <span
                    className={cn([
                      "rounded-full",
                      "bg-primary/10",
                      "px-2",
                      "py-0.5",
                      "font-mono",
                      "text-xs",
                      "text-primary",
                    ])}
                  >
                    {gameNotices.length}
                  </span>
                )}
              </div>
              <ScrollArea className={cn(["min-h-0", "flex-1"])}>
                {gameNotices?.length ? (
                  <div className={cn(["flex", "flex-col", "gap-1"])}>
                    {gameNotices.map((gameNotice) => (
                      <button
                        key={gameNotice?.id}
                        type="button"
                        className={cn([
                          "flex",
                          "items-center",
                          "justify-between",
                          "gap-3",
                          "rounded-md",
                          "px-3",
                          "py-3",
                          "text-left",
                          "transition-colors",
                          "hover:bg-foreground/5",
                        ])}
                        onClick={() => setSelectedNoticeId(gameNotice?.id)}
                      >
                        <span
                          className={cn(["min-w-0", "truncate", "text-sm"])}
                        >
                          {gameNotice?.title}
                        </span>
                        <span
                          className={cn([
                            "shrink-0",
                            "text-xs",
                            "text-muted-foreground",
                          ])}
                        >
                          {new Date(
                            Number(gameNotice?.created_at) * 1000
                          ).toLocaleDateString()}
                        </span>
                      </button>
                    ))}
                  </div>
                ) : (
                  <div
                    className={cn([
                      "flex",
                      "h-full",
                      "min-h-40",
                      "flex-col",
                      "items-center",
                      "justify-center",
                      "gap-2",
                      "text-sm",
                      "text-muted-foreground",
                    ])}
                  >
                    <BellIcon className={cn(["size-7", "opacity-40"])} />
                    <span>{t("game:notice.empty")}</span>
                  </div>
                )}
              </ScrollArea>
            </>
          )}
        </Card>
      </DialogContent>
    </Dialog>
  );
}

export { NoticeCard };
