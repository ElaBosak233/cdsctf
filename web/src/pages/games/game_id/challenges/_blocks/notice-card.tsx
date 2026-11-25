import {
  ArrowLeftIcon,
  BellIcon,
  CheckCheckIcon,
  MegaphoneIcon,
  XIcon,
} from "lucide-react";
import { type CSSProperties, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
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
import type { GameNoticeView } from "@/models/game_notice";
import { isNoticeEdited } from "@/storages/notice";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { parseTimestamp } from "@/utils/time";

type NoticeCardProps = {
  notices: GameNoticeView[];
  unreadIds: Set<number>;
  onMarkNoticesRead: (notices: GameNoticeView[]) => void;
  onMarkAllRead: () => void;
};

function formatShortTime(timestamp: string) {
  const date = parseTimestamp(timestamp);
  if (!date) return "-";

  const now = new Date();
  const sameDay = date.toDateString() === now.toDateString();
  return date.toLocaleString(undefined, {
    ...(sameDay ? {} : { month: "2-digit", day: "2-digit" }),
    hour: "2-digit",
    minute: "2-digit",
  });
}

function NoticeTypeIcon({
  challengeId,
  categoryId,
  className,
}: {
  challengeId: number | null;
  categoryId: number | null;
  className?: string;
}) {
  if (challengeId != null) {
    const category = getCategory(categoryId ?? 1);
    const Icon = category.icon!;
    return (
      <span
        className={cn(
          "inline-flex size-8 shrink-0 items-center justify-center rounded-md border",
          "bg-[color-mix(in_oklab,var(--color-cat)_12%,var(--card))]",
          "border-[color-mix(in_oklab,var(--color-cat)_26%,transparent)]",
          "text-[color-mix(in_oklab,var(--color-cat)_88%,var(--foreground))]",
          "dark:bg-[color-mix(in_oklab,var(--color-cat)_22%,var(--card))]",
          "dark:border-[color-mix(in_oklab,var(--color-cat)_40%,transparent)]",
          "dark:text-[color-mix(in_oklab,var(--color-cat)_62%,white)]",
          className
        )}
        style={{ "--color-cat": category.color } as CSSProperties}
        aria-hidden="true"
      >
        <Icon className="size-4" />
      </span>
    );
  }

  return (
    <span
      className={cn(
        "inline-flex size-8 shrink-0 items-center justify-center rounded-md border border-border/70 bg-muted/80 text-muted-foreground",
        className
      )}
      aria-hidden="true"
    >
      <MegaphoneIcon className="size-4" />
    </span>
  );
}

function NoticeChallengeChip({
  title,
  categoryId,
}: {
  title: string;
  categoryId: number | null;
}) {
  const category = getCategory(categoryId ?? 1);
  const Icon = category.icon!;

  return (
    <span
      className={cn(
        "inline-flex max-w-full items-center gap-1 rounded-md border px-1.5 py-0.5 text-[11px] font-medium",
        "bg-[color-mix(in_oklab,var(--color-cat)_10%,var(--card))]",
        "border-[color-mix(in_oklab,var(--color-cat)_24%,transparent)]",
        "text-[color-mix(in_oklab,var(--color-cat)_88%,var(--foreground))]",
        "dark:bg-[color-mix(in_oklab,var(--color-cat)_20%,var(--card))]",
        "dark:border-[color-mix(in_oklab,var(--color-cat)_38%,transparent)]",
        "dark:text-[color-mix(in_oklab,var(--color-cat)_62%,white)]"
      )}
      style={{ "--color-cat": category.color } as CSSProperties}
    >
      <Icon className="size-3 shrink-0" aria-hidden="true" />
      <span className="truncate">{title}</span>
    </span>
  );
}

function NoticeCard({
  notices,
  unreadIds,
  onMarkNoticesRead,
  onMarkAllRead,
}: NoticeCardProps) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const [selectedNoticeId, setSelectedNoticeId] = useState<number>();
  const [mobileShowDetail, setMobileShowDetail] = useState(false);

  const unreadCount = unreadIds.size;

  const selectedNotice = useMemo(
    () =>
      notices.find((notice) => notice.id === selectedNoticeId) ?? notices[0],
    [notices, selectedNoticeId]
  );

  const handleOpenChange = (nextOpen: boolean) => {
    setOpen(nextOpen);
    if (!nextOpen) {
      setMobileShowDetail(false);
      return;
    }

    const initial =
      notices.find((notice) => unreadIds.has(notice.id)) ?? notices[0];
    setSelectedNoticeId(initial?.id);
    setMobileShowDetail(false);

    if (
      initial &&
      typeof window !== "undefined" &&
      window.matchMedia("(min-width: 768px)").matches
    ) {
      onMarkNoticesRead([initial]);
    }
  };

  const handleSelectNotice = (notice: GameNoticeView) => {
    setSelectedNoticeId(notice.id);
    setMobileShowDetail(true);
    onMarkNoticesRead([notice]);
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <Tooltip>
        <TooltipTrigger
          render={
            <DialogTrigger
              render={
                unreadCount > 0 ? (
                  <Button
                    className="relative h-9 gap-1.5 px-3 text-xs"
                    size="sm"
                    variant="tonal"
                    level="warning"
                    aria-label={`${t("game:notice.board")} - ${t(
                      "game:notice.unread_count",
                      { count: unreadCount }
                    )}`}
                  >
                    <span className="relative inline-flex items-center justify-center">
                      <BellIcon className="size-4" />
                      <span
                        className="absolute -top-0.5 -right-0.5 flex size-2"
                        aria-hidden="true"
                      >
                        <span className="absolute inline-flex size-full rounded-full bg-warning/70 motion-safe:animate-ping" />
                        <span className="relative inline-flex size-2 rounded-full bg-warning" />
                      </span>
                    </span>
                    <span className="font-mono font-semibold tabular-nums">
                      {unreadCount > 99 ? "99+" : unreadCount}
                    </span>
                  </Button>
                ) : (
                  <Button
                    className="relative size-9 text-muted-foreground"
                    size="sm"
                    square
                    variant="ghost"
                    aria-label={t("game:notice.board")}
                  >
                    <BellIcon />
                  </Button>
                )
              }
            />
          }
        />
        <TooltipContent>
          {unreadCount > 0
            ? `${t("game:notice.board")} (${t("game:notice.unread_count", {
                count: unreadCount,
              })})`
            : t("game:notice.board")}
        </TooltipContent>
      </Tooltip>

      <DialogContent
        size="preview"
        className="max-w-5xl"
        slotProps={{ title: { children: t("game:notice.board") } }}
      >
        <Card className="flex h-[min(72vh,640px)] min-h-104 w-full flex-col overflow-hidden rounded-elevated shadow-lg">
          {/* Header */}
          <div className="flex shrink-0 items-center justify-between gap-3 border-b px-5 py-3.5">
            <div className="flex min-w-0 items-center gap-2.5">
              <BellIcon
                className="size-4 shrink-0 text-primary"
                aria-hidden="true"
              />
              <h2 className="truncate text-sm font-semibold">
                {t("game:notice.board")}
              </h2>
              {notices.length > 0 && (
                <span className="rounded-full bg-muted px-2 py-0.5 font-mono text-xs text-muted-foreground tabular-nums">
                  {notices.length}
                </span>
              )}
              {unreadCount > 0 && (
                <span className="rounded-full bg-warning/15 px-2 py-0.5 font-mono text-xs font-medium text-warning tabular-nums">
                  {t("game:notice.unread_count", { count: unreadCount })}
                </span>
              )}
            </div>

            <div className="flex shrink-0 items-center gap-1.5">
              {unreadCount > 0 && (
                <Button
                  size="sm"
                  variant="ghost"
                  icon={<CheckCheckIcon />}
                  className="h-8 gap-1.5 px-2.5 text-xs"
                  onClick={onMarkAllRead}
                >
                  {t("game:notice.mark_all_read")}
                </Button>
              )}
              <Button
                size="sm"
                square
                variant="ghost"
                className="size-8"
                icon={<XIcon />}
                aria-label={t("common:actions.close")}
                onClick={() => handleOpenChange(false)}
              />
            </div>
          </div>

          {/* Two-column master-detail body */}
          {notices.length === 0 ? (
            <div className="flex flex-1 flex-col items-center justify-center gap-2.5 text-sm text-muted-foreground">
              <BellIcon className="size-8 opacity-40" aria-hidden="true" />
              <span>{t("game:notice.empty")}</span>
            </div>
          ) : (
            <div className="flex min-h-0 flex-1 overflow-hidden">
              {/* Left list column */}
              <div
                className={cn(
                  "flex min-h-0 w-full shrink-0 flex-col border-border/70 md:w-80 md:border-r lg:w-88",
                  mobileShowDetail ? "hidden md:flex" : "flex"
                )}
              >
                <ScrollArea className="min-h-0 flex-1">
                  <div className="flex flex-col gap-1 p-2">
                    {notices.map((notice) => {
                      const isUnread = unreadIds.has(notice.id);
                      const isSelected = selectedNotice?.id === notice.id;
                      const edited = isNoticeEdited(notice);

                      return (
                        <button
                          key={notice.id}
                          type="button"
                          onClick={() => handleSelectNotice(notice)}
                          className={cn(
                            "flex w-full cursor-pointer items-center gap-2.5 rounded-lg px-2.5 py-2.5 text-left transition-colors",
                            isSelected
                              ? "bg-muted text-foreground"
                              : "text-foreground/90 hover:bg-muted/50"
                          )}
                        >
                          <NoticeTypeIcon
                            challengeId={notice.challenge_id}
                            categoryId={notice.challenge_category}
                          />
                          <div className="flex min-w-0 flex-1 flex-col gap-1.5">
                            <div className="flex items-center gap-2">
                              {isUnread && (
                                <span
                                  className="size-2 shrink-0 rounded-full bg-warning"
                                  aria-label={t("game:notice.unread")}
                                />
                              )}
                              <span
                                className={cn(
                                  "min-w-0 flex-1 truncate text-sm",
                                  isUnread || isSelected
                                    ? "font-semibold"
                                    : "font-medium"
                                )}
                              >
                                {notice.title}
                              </span>
                            </div>

                            <div className="flex items-center justify-between gap-2 text-xs text-muted-foreground">
                              <div className="min-w-0 flex-1">
                                {notice.challenge_id != null &&
                                notice.challenge_title ? (
                                  <NoticeChallengeChip
                                    title={notice.challenge_title}
                                    categoryId={notice.challenge_category}
                                  />
                                ) : (
                                  <span className="inline-flex items-center gap-1 text-[11px] text-muted-foreground/85">
                                    {t("game:notice.general")}
                                  </span>
                                )}
                              </div>
                              <span className="shrink-0 font-mono text-[11px] tabular-nums">
                                {formatShortTime(notice.created_at)}
                                {edited ? ` · ${t("game:notice.edited")}` : ""}
                              </span>
                            </div>
                          </div>
                        </button>
                      );
                    })}
                  </div>
                </ScrollArea>
              </div>

              {/* Right detail column */}
              <div
                className={cn(
                  "min-h-0 min-w-0 flex-1 flex-col",
                  mobileShowDetail ? "flex" : "hidden md:flex"
                )}
              >
                {selectedNotice ? (
                  <>
                    <div className="flex shrink-0 items-center gap-3 border-b border-border/70 px-5 py-4 sm:px-6">
                      <Button
                        className="size-8 shrink-0 md:hidden"
                        size="sm"
                        square
                        variant="ghost"
                        icon={<ArrowLeftIcon />}
                        aria-label={t("common:actions.cancel")}
                        onClick={() => setMobileShowDetail(false)}
                      />
                      <NoticeTypeIcon
                        challengeId={selectedNotice.challenge_id}
                        categoryId={selectedNotice.challenge_category}
                        className="size-9"
                      />
                      <div className="flex min-w-0 flex-1 flex-col gap-2">
                        <h3 className="min-w-0 text-base font-semibold leading-snug text-foreground">
                          {selectedNotice.title}
                        </h3>

                        <div className="flex flex-wrap items-center gap-x-3 gap-y-1.5 text-xs text-muted-foreground">
                          {selectedNotice.challenge_id != null &&
                          selectedNotice.challenge_title ? (
                            <NoticeChallengeChip
                              title={selectedNotice.challenge_title}
                              categoryId={selectedNotice.challenge_category}
                            />
                          ) : (
                            <span className="inline-flex items-center gap-1 rounded-md bg-muted px-1.5 py-0.5 text-[11px] text-muted-foreground">
                              <MegaphoneIcon
                                className="size-3 shrink-0"
                                aria-hidden="true"
                              />
                              {t("game:notice.general")}
                            </span>
                          )}
                          <span className="font-mono tabular-nums">
                            {t("game:notice.created_at")}{" "}
                            {parseTimestamp(
                              selectedNotice.created_at
                            )?.toLocaleString() ?? "-"}
                          </span>
                          {isNoticeEdited(selectedNotice) && (
                            <span className="font-mono tabular-nums">
                              {t("game:notice.updated_at")}{" "}
                              {parseTimestamp(
                                selectedNotice.updated_at
                              )?.toLocaleString() ?? "-"}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>

                    <ScrollArea className="min-h-0 flex-1">
                      <div className="px-5 py-5 sm:px-6">
                        <Typography>
                          <MarkdownRender src={selectedNotice.content} />
                        </Typography>
                      </div>
                    </ScrollArea>
                  </>
                ) : (
                  <div className="flex flex-1 items-center justify-center text-sm text-muted-foreground">
                    {t("game:notice.select_prompt")}
                  </div>
                )}
              </div>
            </div>
          )}
        </Card>
      </DialogContent>
    </Dialog>
  );
}

export { NoticeCard };
