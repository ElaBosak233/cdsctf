import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { Clock3Icon, LibraryIcon } from "lucide-react";
import { useQueryState } from "nuqs";
import { useEffect, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { queryChallengeStatus } from "@/api/challenges";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { useTickerTime } from "@/hooks/use-ticker-time";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { NoticeCard } from "./_blocks/notice-card";
import { TeamCard } from "./_blocks/team-card";

export default function Index() {
  const { t } = useTranslation();

  const { currentGame, selfTeam: selfGameTeam } = useGameStore();
  const [category, setCategory] = useQueryState("category", {
    defaultValue: "all",
  });
  const navigate = useNavigate();
  const now = useTickerTime();

  const gameId = currentGame?.id;

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
      const challenges = response.challenges;
      return challenges?.sort((a, b) => {
        if (a.challenge_category === b.challenge_category) {
          return (a.pts ?? 0) - (b.pts ?? 0);
        }
        return (a.challenge_category ?? 0) - (b.challenge_category ?? 0);
      });
    },
    enabled: gameId != null,
  });

  const categories = useMemo(() => {
    return Array.from(
      new Set(
        gameChallengesData?.map((gameChallenge) =>
          getCategory(gameChallenge.challenge_category!)
        )
      )
    );
  }, [gameChallengesData]);

  const gameChallenges = useMemo(() => {
    if (!category || category === "all") {
      return gameChallengesData;
    }
    return gameChallengesData?.filter(
      (gameChallengesData) =>
        gameChallengesData.challenge_category === Number(category)
    );
  }, [category, gameChallengesData]);

  useEffect(() => {
    if (!(error instanceof HTTPError)) return;

    if (error.response.status === StatusCodes.FORBIDDEN) {
      navigate(`/games/${currentGame?.id}`);
      toast.error(t("game:challenges.no_permission"));
    }
  }, [error, navigate, currentGame?.id, t]);

  const teamId = selfGameTeam?.id;
  const challengeIds =
    gameChallenges
      ?.map((gc) => gc.challenge_id)
      .filter((id): id is number => id != null) ?? [];

  const { data: challengeStatus, isLoading: statusLoading } = useQuery({
    queryKey: ["game_challenge_status", challengeIds, gameId, teamId],
    queryFn: () =>
      queryChallengeStatus({
        challenge_ids: challengeIds,
        team_id: teamId!,
        game_id: gameId!,
      }),
    select: (response) => response.statuses,
    refetchInterval: 15000,
    enabled: gameId != null && teamId != null && challengeIds.length > 0,
  });

  const loading = useMemo(() => {
    return statusLoading || challengeLoading;
  }, [statusLoading, challengeLoading]);

  const remainingLabel = useMemo(() => {
    if (!currentGame) return "";

    const startTime = new Date(Number(currentGame?.started_at) * 1000);
    const freezeTime = new Date(Number(currentGame?.frozen_at) * 1000);
    const endTime = new Date(Number(currentGame?.ended_at) * 1000);

    const remaining = (target: Date) => {
      const secondsTotal = Math.max(
        0,
        Math.floor((target.getTime() - now.getTime()) / 1000)
      );
      return {
        hours: Math.floor(secondsTotal / 3600),
        minutes: Math.floor((secondsTotal % 3600) / 60),
        seconds: secondsTotal % 60,
      };
    };

    if (now < startTime) {
      return t("game:status.upcoming.remaining", remaining(startTime));
    }
    if (now < freezeTime) {
      return t("game:status.ongoing.remaining", remaining(freezeTime));
    }
    if (now < endTime) {
      return t("game:status.frozen.remaining", remaining(endTime));
    }
    return t("game:status.ended.remaining");
  }, [currentGame, now, t]);

  const statusTone = useMemo(() => {
    if (!currentGame) return "bg-muted-foreground";
    const nowSeconds = now.getTime() / 1000;
    if (nowSeconds > Number(currentGame.ended_at)) return "bg-error";
    if (nowSeconds > Number(currentGame.frozen_at)) return "bg-warning";
    if (nowSeconds < Number(currentGame.started_at)) return "bg-info";
    return "bg-success";
  }, [currentGame, now]);

  const statusKey = useMemo(() => {
    if (!currentGame) return "ongoing";
    const nowSeconds = now.getTime() / 1000;
    if (nowSeconds > Number(currentGame.ended_at)) return "ended";
    if (nowSeconds > Number(currentGame.frozen_at)) return "frozen";
    if (nowSeconds < Number(currentGame.started_at)) return "upcoming";
    return "ongoing";
  }, [currentGame, now]);

  const statusSurface = useMemo(() => {
    if (statusKey === "ended") return "bg-error/10";
    if (statusKey === "frozen") return "bg-warning/10";
    if (statusKey === "upcoming") return "bg-info/10";
    return "bg-success/10";
  }, [statusKey]);

  return (
    <>
      <title>{`${t("challenge:_")} - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col",
          "w-full",
          "max-w-[1600px]",
          "flex-1",
          "min-h-0",
          "mx-auto",
          "gap-5",
          "px-4",
          "py-5",
          "sm:px-6",
          "lg:px-10",
          "lg:py-8",
          "xl:px-14",
        ])}
      >
        <section
          className={cn([
            "relative",
            "z-10",
            "flex",
            "flex-col",
            "rounded-lg",
            "bg-card/70",
            "shadow-sm",
            "backdrop-blur-md",
            "lg:flex-row",
            "lg:items-center",
          ])}
          aria-label={t("challenge:_")}
        >
          <div className={cn(["flex", "min-w-0", "items-center", "lg:flex-1"])}>
            <div className={cn(["pl-2", "sm:pl-3"])}>
              <NoticeCard />
            </div>
            <ScrollableNav
              className={cn([
                "h-15",
                "min-w-0",
                "flex-1",
                "border-b-0",
                "bg-transparent",
                "ml-2",
                "mr-3",
                "lg:mr-4",
              ])}
              contentClassName={cn(["gap-1", "pl-0", "pr-2"])}
              aria-label={t("challenge:_")}
            >
              {categories?.length > 0 && (
                <Button
                  icon={<LibraryIcon />}
                  size={"sm"}
                  className={cn(["shrink-0", "min-w-fit"])}
                  onClick={() => {
                    setCategory("all");
                  }}
                  variant={category === "all" ? "solid" : "ghost"}
                  aria-pressed={category === "all"}
                >
                  {t("game:filter.all")}
                </Button>
              )}
              {categories?.map((c) => {
                const Icon = c.icon!;

                return (
                  <Button
                    key={c.id}
                    icon={<Icon />}
                    size={"sm"}
                    className={cn(["shrink-0", "min-w-fit"])}
                    variant={c?.id?.toString() === category ? "solid" : "ghost"}
                    onClick={() => {
                      setCategory(String(c.id));
                    }}
                    aria-pressed={c?.id?.toString() === category}
                  >
                    {c.name?.toUpperCase()}
                  </Button>
                );
              })}
            </ScrollableNav>
          </div>

          <div
            className={cn([
              "min-w-0",
              "px-3",
              "pb-3",
              "lg:shrink-0",
              "lg:py-2",
              "lg:pl-2",
            ])}
          >
            <TeamCard />
          </div>
        </section>

        <main className={cn(["min-w-0", "flex-1", "flex", "flex-col"])}>
          <div className={cn(["relative", "flex-1"])}>
            <LoadingOverlay loading={loading} />
            <div
              className={cn([
                "w-full",
                "grid",
                "grid-cols-1",
                "sm:grid-cols-2",
                "md:grid-cols-3",
                "lg:grid-cols-4",
                "xl:grid-cols-5",
                "2xl:grid-cols-6",
                "gap-4",
                "relative",
              ])}
            >
              {gameChallenges?.map((gameChallenge, index) => {
                const status = challengeStatus?.[gameChallenge.challenge_id!];
                const isCheated = status?.cheated ?? false;

                return isCheated ? (
                  <ChallengeCard
                    key={index}
                    digest={{
                      id: gameChallenge.challenge_id,
                      title: gameChallenge.challenge_title,
                      category: gameChallenge.challenge_category,
                    }}
                    status={status}
                  />
                ) : (
                  <Dialog key={index}>
                    <DialogTrigger>
                      <ChallengeCard
                        digest={{
                          id: gameChallenge.challenge_id,
                          title: gameChallenge.challenge_title,
                          category: gameChallenge.challenge_category,
                        }}
                        status={status}
                      />
                    </DialogTrigger>
                    <DialogContent size="preview">
                      <ChallengeDialog
                        digest={{
                          id: gameChallenge.challenge_id,
                          title: gameChallenge.challenge_title,
                          category: gameChallenge.challenge_category,
                        }}
                        gameTeam={selfGameTeam}
                        frozenAt={gameChallenge?.frozen_at}
                      />
                    </DialogContent>
                  </Dialog>
                );
              })}
            </div>
            {!loading && gameChallenges?.length === 0 && (
              <div
                className={cn([
                  "flex",
                  "min-h-64",
                  "flex-col",
                  "items-center",
                  "justify-center",
                  "gap-3",
                  "text-muted-foreground",
                ])}
              >
                <LibraryIcon className={cn(["size-8"])} />
                <p className={cn(["text-sm"])}>{t("game:challenge.empty")}</p>
              </div>
            )}
          </div>
        </main>
      </div>
      <div
        className={cn([
          "fixed",
          "bottom-4",
          "left-4",
          "z-40",
          "flex",
          "items-center",
          "gap-2",
          "rounded-md",
          "px-3",
          "py-2",
          "shadow-lg",
          "backdrop-blur-md",
          "ring-1",
          "ring-border/60",
          "select-none",
          statusSurface,
          "sm:bottom-6",
          "sm:left-6",
        ])}
        aria-label={t(`game:status.${statusKey}._`)}
      >
        <span
          className={cn(["size-2", "shrink-0", "rounded-full", statusTone])}
          aria-hidden="true"
        />
        <Clock3Icon
          className={cn(["size-4", "shrink-0", "text-muted-foreground"])}
        />
        <div className={cn(["min-w-0", "whitespace-nowrap"])}>
          <span
            className={cn(["block", "text-[11px]", "text-muted-foreground"])}
          >
            {t(`game:status.${statusKey}._`)}
          </span>
          <span
            className={cn(["block", "font-mono", "text-xs", "tabular-nums"])}
          >
            {remainingLabel}
          </span>
        </div>
      </div>
    </>
  );
}
