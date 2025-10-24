import { useQuery } from "@tanstack/react-query";
import { StatusCodes } from "http-status-codes";
import { HTTPError } from "ky";
import { useEffect, useMemo } from "react";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import { getChallengeStatus } from "@/api/challenges";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";
import { NoticeCard } from "./notice-card";
import { TeamCard } from "./team-card";

export default function Index() {
  const { currentGame, selfTeam: selfGameTeam } = useGameStore();
  const navigate = useNavigate();

  const {
    data: gameChallenges,
    error,
    isFetching: challengeLoading,
  } = useQuery({
    queryKey: ["game_challenges", currentGame?.id],
    queryFn: () =>
      getGameChallenges({
        game_id: currentGame?.id,
      }),
    select: (response) => response.data,
  });

  useEffect(() => {
    if (!(error instanceof HTTPError)) return;

    if (error.response.status === StatusCodes.FORBIDDEN) {
      navigate(`/games/${currentGame?.id}`);
      toast.error("你没有权限查看本场比赛的题目");
    }
  }, [error, navigate, currentGame?.id]);

  const { data: challengeStatus, isFetching: statusLoading } = useQuery({
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
        <div
          className={cn(["flex-1", "flex", "flex-col", "gap-8", "relative"])}
        >
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
        <div className={cn(["lg:sticky", "lg:top-16", "lg:w-80"])}>
          <div
            className={cn([
              "flex",
              "flex-col",
              "gap-5",
              "lg:h-[calc(100vh-9rem)]",
            ])}
          >
            <TeamCard />
            <NoticeCard />
          </div>
        </div>
      </div>
    </>
  );
}
