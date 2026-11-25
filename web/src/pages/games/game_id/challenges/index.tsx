import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import {
  CheckIcon,
  CirclePauseIcon,
  Clock3Icon,
  EyeIcon,
  EyeOffIcon,
  FilePenIcon,
  LibraryIcon,
} from "lucide-react";
import { useQueryState } from "nuqs";
import {
  type CSSProperties,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router";
import { toast } from "sonner";
import { queryChallengeStatus } from "@/api/challenges";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { getGameNotice } from "@/api/games/game_id/notices";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { Separator } from "@/components/ui/separator";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { useTickerTime } from "@/hooks/use-ticker-time";
import type { GameChallengeSummary } from "@/models/game_challenge";
import type { GameNoticeView } from "@/models/game_notice";
import { useAuthStore } from "@/storages/auth";
import { useGameStore } from "@/storages/game";
import {
  getNoticeFingerprints,
  getNoticeScopeKey,
  isNoticeUnread,
  useNoticeReadStore,
} from "@/storages/notice";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { getGamePhase, isGameActive, secondsUntil } from "@/utils/time";
import { NoticeCard } from "./_blocks/notice-card";
import { TeamCard } from "./_blocks/team-card";

export default function Index() {
  const { t } = useTranslation();

  const user = useAuthStore((state) => state.user);
  const { currentGame, selfTeam: selfGameTeam } = useGameStore();
  const [category, setCategory] = useQueryState("category", {
    defaultValue: "all",
  });
  const [hideSolved, setHideSolved] = useState(false);
  const navigate = useNavigate();
  const now = useTickerTime();

  const gameId = currentGame?.id;
  const userId = user?.id;
  const noticeScopeKey =
    userId != null && gameId != null
      ? getNoticeScopeKey(userId, gameId)
      : undefined;
  const noticeScope = useNoticeReadStore((state) =>
    noticeScopeKey ? state.scopes[noticeScopeKey] : undefined
  );
  const syncNotices = useNoticeReadStore((state) => state.syncNotices);
  const markAsRead = useNoticeReadStore((state) => state.markAsRead);

  const {
    data: gameChallengesData,
    error,
    isLoading: challengeLoading,
  } = useQuery({
    queryKey: ["game_challenges", gameId],
    queryFn: () =>
      getGameChallenges({
        game_id: gameId!,
      }),
    select: (response) => {
      const challenges = [...(response.challenges ?? [])];
      return challenges.sort((a, b) => {
        if (a.challenge_category === b.challenge_category) {
          return (a.pts ?? 0) - (b.pts ?? 0);
        }
        return (a.challenge_category ?? 0) - (b.challenge_category ?? 0);
      });
    },
    enabled: gameId != null && !currentGame?.paused,
    meta: { suppressErrorToast: true },
  });

  useEffect(() => {
    if (!(error instanceof HTTPError)) return;

    if (error.response.status === StatusCodes.FORBIDDEN) {
      navigate(`/games/${currentGame?.id}`);
      toast.error(t("game:challenges.no_permission.title"), {
        description: t("game:challenges.no_permission.description"),
      });
    }
    if (error.response.status === StatusCodes.LOCKED) {
      navigate(`/games/${currentGame?.id}`);
      toast.error(t("game:paused.title"), {
        description: t("game:paused.description"),
      });
    }
  }, [error, navigate, currentGame?.id, t]);

  const teamId = selfGameTeam?.id;

  // Query statuses for the full challenge list once so category switching and
  // "hide solved" are instant and category progress counts stay accurate.
  const allChallengeIds = useMemo(
    () =>
      gameChallengesData
        ?.map((gc) => gc.challenge_id)
        .filter((id): id is number => id != null) ?? [],
    [gameChallengesData]
  );

  const { data: challengeStatus, isLoading: statusLoading } = useQuery({
    queryKey: ["game_challenge_status", allChallengeIds, gameId, teamId],
    queryFn: () =>
      queryChallengeStatus({
        challenge_ids: allChallengeIds,
        team_id: teamId!,
        game_id: gameId!,
      }),
    select: (response) => response.statuses,
    refetchInterval: 15000,
    enabled:
      gameId != null &&
      teamId != null &&
      allChallengeIds.length > 0 &&
      !currentGame?.paused,
  });

  const loading = statusLoading || challengeLoading;

  const { data: gameNotices } = useQuery({
    queryKey: ["game_notices", gameId],
    queryFn: () =>
      getGameNotice({
        game_id: gameId!,
      }),
    select: (response) => response.notices,
    refetchInterval: 15000,
    enabled: gameId != null && teamId != null,
    meta: { suppressErrorToast: true },
  });

  useEffect(() => {
    if (!noticeScopeKey || !gameNotices) return;
    syncNotices(noticeScopeKey, gameNotices);
  }, [gameNotices, noticeScopeKey, syncNotices]);

  const notices = gameNotices ?? [];

  const { unreadNoticeIds, noticesByChallenge, unreadChallengeIds } =
    useMemo(() => {
      const unreadIds = new Set<number>();
      const byChallenge = new Map<number, GameNoticeView[]>();
      const unreadChallenges = new Set<number>();

      for (const notice of notices) {
        const unread =
          noticeScopeKey != null && isNoticeUnread(noticeScope, notice);
        if (unread) unreadIds.add(notice.id);

        if (notice.challenge_id != null) {
          const list = byChallenge.get(notice.challenge_id) ?? [];
          list.push(notice);
          byChallenge.set(notice.challenge_id, list);
          if (unread) unreadChallenges.add(notice.challenge_id);
        }
      }

      return {
        unreadNoticeIds: unreadIds,
        noticesByChallenge: byChallenge,
        unreadChallengeIds: unreadChallenges,
      };
    }, [notices, noticeScope, noticeScopeKey]);

  const handleMarkNoticesRead = useCallback(
    (items: GameNoticeView[]) => {
      if (!noticeScopeKey || items.length === 0) return;
      markAsRead(noticeScopeKey, getNoticeFingerprints(items));
    },
    [markAsRead, noticeScopeKey]
  );

  const handleMarkAllNoticesRead = useCallback(() => {
    if (!noticeScopeKey || notices.length === 0) return;
    markAsRead(noticeScopeKey, getNoticeFingerprints(notices));
  }, [markAsRead, noticeScopeKey, notices]);

  const { categoryTabs, totalSolved, totalChallenges } = useMemo(() => {
    const all = gameChallengesData ?? [];
    const byCategory = new Map<
      number,
      {
        category: ReturnType<typeof getCategory>;
        total: number;
        solved: number;
      }
    >();
    let solvedTotal = 0;

    for (const gc of all) {
      const catId = gc.challenge_category ?? 1;
      const isSolved = Boolean(challengeStatus?.[gc.challenge_id]?.solved);
      if (isSolved) solvedTotal += 1;

      const entry = byCategory.get(catId) ?? {
        category: getCategory(catId),
        total: 0,
        solved: 0,
      };
      entry.total += 1;
      if (isSolved) entry.solved += 1;
      byCategory.set(catId, entry);
    }

    return {
      categoryTabs: Array.from(byCategory.values()),
      totalSolved: solvedTotal,
      totalChallenges: all.length,
    };
  }, [gameChallengesData, challengeStatus]);

  const filteredChallenges = useMemo(() => {
    return (gameChallengesData ?? []).filter((gc) => {
      if (
        category &&
        category !== "all" &&
        gc.challenge_category !== Number(category)
      ) {
        return false;
      }
      if (hideSolved && challengeStatus?.[gc.challenge_id]?.solved) {
        return false;
      }
      return true;
    });
  }, [gameChallengesData, category, hideSolved, challengeStatus]);

  const groupedChallenges = useMemo(() => {
    const groups: Array<{
      categoryId: number;
      category: ReturnType<typeof getCategory>;
      challenges: GameChallengeSummary[];
      solved: number;
      total: number;
    }> = [];

    const totalsByCat = new Map(
      categoryTabs.map((tab) => [tab.category.id ?? 1, tab])
    );

    for (const gc of filteredChallenges) {
      const catId = gc.challenge_category ?? 1;
      const last = groups[groups.length - 1];
      if (last && last.categoryId === catId) {
        last.challenges.push(gc);
      } else {
        const stats = totalsByCat.get(catId);
        groups.push({
          categoryId: catId,
          category: getCategory(catId),
          challenges: [gc],
          solved: stats?.solved ?? 0,
          total: stats?.total ?? 0,
        });
      }
    }
    return groups;
  }, [filteredChallenges, categoryTabs]);

  const remainingLabel = useMemo(() => {
    if (!currentGame) return "";

    const remaining = (target: string | null | undefined) => {
      const secondsTotal = Math.max(0, secondsUntil(target, now) ?? 0);
      return {
        hours: Math.floor(secondsTotal / 3600),
        minutes: Math.floor((secondsTotal % 3600) / 60),
        seconds: secondsTotal % 60,
      };
    };

    const phase = getGamePhase(currentGame, now);
    if (phase == null) return "";
    if (phase === "upcoming") {
      return t(
        "game:status.upcoming.remaining",
        remaining(currentGame.started_at)
      );
    }
    if (phase === "ongoing") {
      return t(
        "game:status.ongoing.remaining",
        remaining(currentGame.frozen_at)
      );
    }
    if (phase === "frozen") {
      return t("game:status.frozen.remaining", remaining(currentGame.ended_at));
    }
    return t("game:status.ended.remaining");
  }, [currentGame, now, t]);

  const statusKey = useMemo(() => {
    if (!currentGame) return undefined;
    return getGamePhase(currentGame, now);
  }, [currentGame, now]);

  const statusTone = useMemo(() => {
    if (statusKey === "ended") return "bg-error";
    if (statusKey === "frozen") return "bg-warning";
    if (statusKey === "upcoming") return "bg-info";
    if (statusKey == null) return "bg-muted-foreground";
    return "bg-success";
  }, [statusKey]);

  const statusChipClass = useMemo(() => {
    if (statusKey === "ended") return "border-error/25 bg-error/10";
    if (statusKey === "frozen") return "border-warning/25 bg-warning/10";
    if (statusKey === "upcoming") return "border-info/25 bg-info/10";
    if (statusKey == null) return "border-border bg-muted/40";
    return "border-success/25 bg-success/10";
  }, [statusKey]);

  const showWriteupAction =
    Boolean(selfGameTeam) &&
    currentGame != null &&
    Boolean(currentGame.writeup_required) &&
    isGameActive(currentGame);
  const writeupLabel = selfGameTeam?.has_writeup
    ? t("team:write_up.actions.submit.done")
    : t("team:write_up.actions.submit._");

  const hasActiveFilters = hideSolved || (category && category !== "all");
  const showGroupedSections = !category || category === "all";

  const renderChallengeCard = (gameChallenge: GameChallengeSummary) => {
    const status = challengeStatus?.[gameChallenge.challenge_id];
    const isCheated = status?.cheated ?? false;
    const isSolved = !isCheated && Boolean(status?.solved);
    const challengeNotices =
      noticesByChallenge.get(gameChallenge.challenge_id) ?? [];
    const hasUnreadNotice =
      !isSolved &&
      !isCheated &&
      unreadChallengeIds.has(gameChallenge.challenge_id);
    const challengeKey =
      gameChallenge.challenge_id ?? gameChallenge.challenge_title;

    if (isCheated) {
      return (
        <ChallengeCard
          key={challengeKey}
          digest={{
            id: gameChallenge.challenge_id,
            title: gameChallenge.challenge_title,
            category: gameChallenge.challenge_category,
          }}
          status={status}
        />
      );
    }

    return (
      <Dialog
        key={challengeKey}
        onOpenChange={(open) => {
          if (open && challengeNotices.length > 0) {
            handleMarkNoticesRead(challengeNotices);
          }
        }}
      >
        <DialogTrigger
          render={
            <ChallengeCard
              render={<button type="button" />}
              digest={{
                id: gameChallenge.challenge_id,
                title: gameChallenge.challenge_title,
                category: gameChallenge.challenge_category,
              }}
              status={status}
              hasUnreadNotice={hasUnreadNotice}
            />
          }
        />
        <DialogContent size="preview" className="max-w-4xl">
          <ChallengeDialog
            digest={{
              id: gameChallenge.challenge_id,
              title: gameChallenge.challenge_title,
              category: gameChallenge.challenge_category,
            }}
            gameTeam={selfGameTeam}
            frozenAt={gameChallenge?.frozen_at}
            notices={challengeNotices}
          />
        </DialogContent>
      </Dialog>
    );
  };

  if (currentGame?.paused) {
    return (
      <>
        <title>{`${t("challenge:_")} - ${currentGame.title}`}</title>
        <div className="flex flex-1 flex-col items-center justify-center gap-4 px-6 text-center select-none">
          <CirclePauseIcon className="size-16 text-muted-foreground" />
          <div className="flex flex-col gap-1.5">
            <h1 className="text-lg font-semibold">{t("game:paused.title")}</h1>
            <p className="text-sm text-muted-foreground">
              {t("game:paused.description")}
            </p>
          </div>
          {notices.length > 0 && (
            <div className="mt-2">
              <NoticeCard
                notices={notices}
                unreadIds={unreadNoticeIds}
                onMarkNoticesRead={handleMarkNoticesRead}
                onMarkAllRead={handleMarkAllNoticesRead}
              />
            </div>
          )}
        </div>
      </>
    );
  }

  return (
    <>
      <title>{`${t("challenge:_")} - ${currentGame?.title}`}</title>
      <div className="flex min-h-0 w-full flex-1 flex-col">
        {/* Full-width docked single-row workspace HUD (z-20: strictly below Navbar z-40) */}
        <section
          className="sticky top-(--app-navbar-height) z-20 w-full border-b bg-card/80 backdrop-blur-md"
          aria-label={t("challenge:_")}
        >
          <div className="mx-auto flex w-full max-w-[1720px] flex-col gap-3 px-4 py-3.5 sm:px-6 lg:flex-row lg:items-center lg:justify-between lg:px-8">
            <TeamCard solvedCount={totalSolved} totalCount={totalChallenges} />

            <div className="flex flex-wrap items-center gap-3 lg:shrink-0">
              <div
                className={cn(
                  "inline-flex items-center gap-2 rounded-lg border px-3 py-1.5 select-none",
                  statusChipClass
                )}
                aria-label={
                  statusKey ? t(`game:status.${statusKey}._`) : undefined
                }
              >
                <span
                  className={cn("size-2 shrink-0 rounded-full", statusTone)}
                  aria-hidden="true"
                />
                <Clock3Icon
                  className="size-3.5 shrink-0 text-muted-foreground"
                  aria-hidden="true"
                />
                <span className="text-xs font-medium text-foreground">
                  {statusKey ? t(`game:status.${statusKey}._`) : "-"}
                </span>
                {remainingLabel && statusKey !== "ended" && (
                  <>
                    <span
                      className="text-border select-none"
                      aria-hidden="true"
                    >
                      ·
                    </span>
                    <span className="font-mono text-xs text-muted-foreground tabular-nums">
                      {remainingLabel}
                    </span>
                  </>
                )}
              </div>

              <div className="flex items-center gap-1.5">
                {showWriteupAction && (
                  <Button
                    size="sm"
                    variant="tonal"
                    level={selfGameTeam?.has_writeup ? "success" : "warning"}
                    icon={<FilePenIcon />}
                    className="h-9 gap-1.5 px-3 text-xs"
                    render={
                      <Link
                        to={`/games/${selfGameTeam?.game_id}/team/writeup`}
                      />
                    }
                  >
                    {writeupLabel}
                  </Button>
                )}
                <NoticeCard
                  notices={notices}
                  unreadIds={unreadNoticeIds}
                  onMarkNoticesRead={handleMarkNoticesRead}
                  onMarkAllRead={handleMarkAllNoticesRead}
                />
              </div>
            </div>
          </div>
        </section>

        {/* Challenge workspace */}
        <main className="mx-auto flex min-w-0 w-full max-w-[1720px] flex-1 flex-col gap-5 px-4 pt-2 pb-8 sm:px-6 lg:px-8 lg:pt-3 lg:pb-10">
          {/* Category filter bar + Hide solved (scrolls with content to preserve vertical space) */}
          {totalChallenges > 0 && (
            <div className="flex items-center gap-2 border-b border-border/60">
              <ScrollableNav
                className="h-15 min-w-0 flex-1 border-b-0 bg-transparent"
                contentClassName="gap-1.5 px-0"
                aria-label={t("challenge:_")}
              >
                {categoryTabs.length > 0 && (
                  <Button
                    icon={<LibraryIcon />}
                    size="sm"
                    className="shrink-0 min-w-fit gap-2"
                    onClick={() => setCategory("all")}
                    variant={category === "all" ? "solid" : "ghost"}
                    aria-pressed={category === "all"}
                  >
                    <span>{t("game:filter.all")}</span>
                    <span
                      className={cn(
                        "font-mono text-xs tabular-nums",
                        category === "all"
                          ? "text-primary-foreground/75"
                          : "text-muted-foreground"
                      )}
                    >
                      {totalSolved}/{totalChallenges}
                    </span>
                  </Button>
                )}
                {categoryTabs.map(({ category: c, solved, total }) => {
                  const Icon = c.icon!;
                  const isActive = c.id?.toString() === category;
                  const isCleared = total > 0 && solved === total;

                  return (
                    <Button
                      key={c.id}
                      icon={<Icon />}
                      size="sm"
                      className="shrink-0 min-w-fit gap-2"
                      variant={isActive ? "solid" : "ghost"}
                      onClick={() => setCategory(String(c.id))}
                      aria-pressed={isActive}
                    >
                      <span>{c.name?.toUpperCase()}</span>
                      <span
                        className={cn(
                          "inline-flex items-center gap-0.5 font-mono text-xs tabular-nums",
                          isActive
                            ? "text-primary-foreground/75"
                            : isCleared
                              ? "text-success"
                              : "text-muted-foreground"
                        )}
                      >
                        {isCleared && (
                          <CheckIcon className="size-3" aria-hidden="true" />
                        )}
                        {solved}/{total}
                      </span>
                    </Button>
                  );
                })}
              </ScrollableNav>

              <Separator orientation="vertical" className="h-6 shrink-0" />

              <Button
                size="sm"
                variant={hideSolved ? "solid" : "ghost"}
                icon={hideSolved ? <EyeOffIcon /> : <EyeIcon />}
                className="shrink-0 gap-1.5 px-3"
                onClick={() => setHideSolved((prev) => !prev)}
                aria-pressed={hideSolved}
              >
                {t("game:filter.hide_solved")}
              </Button>
            </div>
          )}

          <div className="relative flex-1">
            <LoadingOverlay loading={loading} />

            {filteredChallenges.length > 0 &&
              (showGroupedSections ? (
                <div className="flex flex-col gap-7">
                  {groupedChallenges.map((group) => {
                    const CategoryIcon = group.category.icon!;
                    const isCleared =
                      group.total > 0 && group.solved === group.total;

                    return (
                      <section
                        key={group.categoryId}
                        aria-labelledby={`category-section-${group.categoryId}`}
                      >
                        <div className="mb-3.5 flex items-center gap-2.5">
                          <span
                            className={cn(
                              "inline-flex size-7 shrink-0 items-center justify-center rounded-md border",
                              "bg-[color-mix(in_oklab,var(--color-cat)_12%,var(--card))]",
                              "border-[color-mix(in_oklab,var(--color-cat)_24%,transparent)]",
                              "text-[color-mix(in_oklab,var(--color-cat)_88%,var(--foreground))]",
                              "dark:bg-[color-mix(in_oklab,var(--color-cat)_22%,var(--card))]",
                              "dark:border-[color-mix(in_oklab,var(--color-cat)_40%,transparent)]",
                              "dark:text-[color-mix(in_oklab,var(--color-cat)_62%,white)]"
                            )}
                            style={
                              {
                                "--color-cat": group.category.color,
                              } as CSSProperties
                            }
                            aria-hidden="true"
                          >
                            <CategoryIcon className="size-3.5" />
                          </span>
                          <h2
                            id={`category-section-${group.categoryId}`}
                            className="text-sm font-semibold text-foreground"
                          >
                            {group.category.name?.toUpperCase()}
                          </h2>
                          <span
                            className={cn(
                              "inline-flex items-center gap-1 font-mono text-xs tabular-nums",
                              isCleared
                                ? "text-success"
                                : "text-muted-foreground"
                            )}
                          >
                            {isCleared && (
                              <CheckIcon
                                className="size-3.5"
                                aria-hidden="true"
                              />
                            )}
                            {group.solved}/{group.total}
                          </span>
                          <div
                            className="h-px flex-1 bg-border/60"
                            aria-hidden="true"
                          />
                        </div>

                        <div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
                          {group.challenges.map(renderChallengeCard)}
                        </div>
                      </section>
                    );
                  })}
                </div>
              ) : (
                <div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
                  {filteredChallenges.map(renderChallengeCard)}
                </div>
              ))}

            {!loading && filteredChallenges.length === 0 && (
              <div className="flex min-h-64 flex-col items-center justify-center gap-3 text-muted-foreground">
                <LibraryIcon className="size-8 opacity-60" />
                <p className="text-sm">
                  {hasActiveFilters && totalChallenges > 0
                    ? t("game:challenge.no_match")
                    : t("game:challenge.empty")}
                </p>
                {hasActiveFilters && totalChallenges > 0 && (
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => {
                      setCategory("all");
                      setHideSolved(false);
                    }}
                  >
                    {t("game:search.clear")}
                  </Button>
                )}
              </div>
            )}
          </div>
        </main>
      </div>
    </>
  );
}
