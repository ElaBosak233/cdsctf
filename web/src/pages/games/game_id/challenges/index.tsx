import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { LibraryIcon } from "lucide-react";
import { useQueryState } from "nuqs";
import { useEffect, useMemo } from "react";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { getChallengeStatus } from "@/api/challenges";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Separator } from "@/components/ui/separator";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { useTickerTime } from "@/hooks/use-ticker-time";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { NoticeCard } from "./notice-card";
import { TeamCard } from "./team-card";

export default function Index() {
  const { currentGame, selfTeam: selfGameTeam } = useGameStore();
  const [category, setCategory] = useQueryState("category", {
    defaultValue: "all",
  });
  const navigate = useNavigate();
  const now = useTickerTime();

  const {
    data: gameChallengesData,
    error,
    isLoading: challengeLoading,
  } = useQuery({
    queryKey: ["game_challenges", currentGame?.id],
    queryFn: () =>
      getGameChallenges({
        game_id: currentGame?.id,
      }),
    select: (response) => {
      const challenges = response.data;
      return challenges?.sort((a, b) => {
        if (a.challenge_category === b.challenge_category) {
          return (a.pts ?? 0) - (b.pts ?? 0);
        }
        return (a.challenge_category ?? 0) - (b.challenge_category ?? 0);
      });
    },
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
      toast.error("你没有权限查看本场比赛的题目");
    }
  }, [error, navigate, currentGame?.id]);

  const { data: challengeStatus, isLoading: statusLoading } = useQuery({
    queryKey: [
      "game_challenge_status",
      gameChallenges?.map((gameChallenge) => gameChallenge.challenge_id!),
      currentGame?.id,
      selfGameTeam?.id,
      currentGame?.id,
    ],
    queryFn: () =>
      getChallengeStatus({
        challenge_ids:
          gameChallenges?.map((gameChallenge) => gameChallenge.challenge_id!) ||
          [],
        team_id: selfGameTeam?.id,
        game_id: currentGame?.id,
      }),
    select: (response) => response.data,
    refetchInterval: 15000,
  });

  const loading = useMemo(() => {
    return statusLoading || challengeLoading;
  }, [statusLoading, challengeLoading]);

  return (
    <>
      <title>{`题目 - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col-reverse",
          "lg:flex-row",
          "justify-evenly",
          "my-10",
          "mx-10",
          "xl:mx-20",
          "gap-10",
        ])}
      >
        <div className={cn(["flex-1", "flex", "flex-col", "gap-8"])}>
          <div className={cn(["flex", "gap-5", "flex-wrap"])}>
            {categories?.length > 0 && (
              <Button
                icon={<LibraryIcon />}
                onClick={() => {
                  setCategory("all");
                }}
                className={cn(["flex-1", "min-w-fit"])}
                variant={category === "all" ? "solid" : "ghost"}
              >
                ALL
              </Button>
            )}
            {categories?.map((c) => {
              const Icon = c.icon!;

              return (
                <>
                  <Separator orientation="vertical" />
                  <Button
                    key={c.id}
                    icon={<Icon />}
                    className={cn(["flex-1", "min-w-fit"])}
                    variant={c?.id?.toString() === category ? "solid" : "ghost"}
                    onClick={() => {
                      setCategory(String(c.id));
                    }}
                  >
                    {c.name?.toUpperCase()}
                  </Button>
                </>
              );
            })}
          </div>

          <div className={cn(["flex-1", "relative"])}>
            <LoadingOverlay loading={loading} />
            <div
              className={cn([
                "w-full",
                "grid",
                "sm:grid-cols-2",
                "lg:grid-cols-3",
                "xl:grid-cols-4",
                "2xl:grid-cols-5",
                "gap-4",
                "relative",
              ])}
            >
              {gameChallenges?.map((gameChallenge, index) => (
                <Dialog key={index}>
                  <DialogTrigger>
                    <ChallengeCard
                      digest={{
                        id: gameChallenge.challenge_id,
                        title: gameChallenge.challenge_title,
                        category: gameChallenge.challenge_category,
                      }}
                      status={challengeStatus?.[gameChallenge.challenge_id!]}
                    />
                  </DialogTrigger>
                  <DialogContent>
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
              ))}
            </div>
          </div>
        </div>
        <div
          className={cn([
            "lg:sticky",
            "lg:top-26",
            "lg:w-80",
            "lg:h-[calc(100vh-144px)]",
          ])}
        >
          <div
            className={cn([
              "flex",
              "flex-col",
              "gap-6",
              "lg:h-[calc(100vh-9rem)]",
            ])}
          >
            <div
              className={cn([
                "flex",
                "justify-center",
                "items-center",
                "text-md",
                "text-muted-foreground",
                "select-none",
                "gap-3",
              ])}
            >
              <span
                className={cn([
                  "size-1.5",
                  "rounded-full",
                  "bg-success",
                  Date.now() / 1000 > Number(currentGame?.frozen_at) &&
                    "bg-error",
                ])}
                aria-hidden="true"
              />
              <span>
                {(() => {
                  const startTime = new Date(
                    Number(currentGame?.started_at) * 1000
                  );
                  const freezeTime = new Date(
                    Number(currentGame?.frozen_at) * 1000
                  );
                  const endTime = new Date(
                    Number(currentGame?.ended_at) * 1000
                  );

                  const diff = (target: Date) =>
                    Math.max(
                      0,
                      Math.floor((target.getTime() - now.getTime()) / 1000)
                    );
                  const formatTime = (seconds: number) => {
                    const h = Math.floor(seconds / 3600);
                    const m = Math.floor((seconds % 3600) / 60);
                    const s = seconds % 60;
                    return `${h.toString().padStart(2, "0")} 时 ${m.toString().padStart(2, "0")} 分 ${s
                      .toString()
                      .padStart(2, "0")} 秒`;
                  };

                  if (now < startTime) {
                    const remain = diff(startTime);
                    return `距开始还有 ${formatTime(remain)}`;
                  } else if (now < freezeTime) {
                    const remain = diff(freezeTime);
                    return `距冻结还有 ${formatTime(remain)}`;
                  } else if (now < endTime) {
                    const remain = diff(endTime);
                    return `距结束还有 ${formatTime(remain)}`;
                  } else {
                    return "比赛已结束";
                  }
                })()}
              </span>
            </div>
            <Separator />
            <TeamCard />
            <NoticeCard />
          </div>
        </div>
      </div>
    </>
  );
}
