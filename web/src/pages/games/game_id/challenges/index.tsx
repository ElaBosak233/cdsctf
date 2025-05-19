import { useEffect, useState } from "react";

import { NoticeCard } from "./notice-card";
import { TeamCard } from "./team-card";

import { ChallengeStatus, getChallengeStatus } from "@/api/challenges";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { GameChallenge } from "@/models/game_challenge";
import { useGameStore } from "@/storages/game";
import { cn } from "@/utils";

export default function Index() {
  const { currentGame, selfTeam: selfGameTeam } = useGameStore();

  const [gameChallenges, setGameChallenges] = useState<Array<GameChallenge>>();
  const [challengeStatus, setChallengeStatus] =
    useState<Record<string, ChallengeStatus>>();
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    if (!currentGame) return;

    setLoading(true);
    getGameChallenges({
      game_id: currentGame.id!,
    })
      .then((res) => {
        setGameChallenges(res.data);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [currentGame]);

  useEffect(() => {
    if (!gameChallenges) return;

    getChallengeStatus({
      challenge_ids: gameChallenges?.map(
        (gameChallenge) => gameChallenge.challenge_id!
      ),
      team_id: selfGameTeam?.id,
      game_id: currentGame?.id,
    }).then((res) => {
      setChallengeStatus(res.data);
    });
  }, [gameChallenges]);

  return (
    <>
      <title>{`题目 - ${currentGame?.title}`}</title>
      <div
        className={cn([
          "flex",
          "flex-col-reverse",
          "lg:flex-row",
          "justify-evenly",
          "mx-10",
          "lg:mx-20",
          "xl:mx-[10vw]",
          "2xl:mx-[15vw]",
          "my-10",
          "gap-10",
        ])}
      >
        <div
          className={cn(["lg:w-3/4", "flex", "flex-col", "gap-8", "relative"])}
        >
          <LoadingOverlay loading={loading} />
          <div
            className={cn([
              "w-full",
              "grid",
              "sm:grid-cols-2",
              "lg:grid-cols-3",
              "xl:grid-cols-4",
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
                  />
                </DialogContent>
              </Dialog>
            ))}
          </div>
        </div>
        <div className={cn(["lg:sticky", "lg:w-1/4", "lg:top-16"])}>
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
