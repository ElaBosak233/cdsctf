import { cn } from "@/utils";
import { TeamCard } from "./team-card";
import { useEffect, useState } from "react";
import { GameChallenge } from "@/models/game_challenge";
import { getGameChallenges } from "@/api/games/game_id/challenges";
import { useGameStore } from "@/storages/game";
import { ChallengeStatus, getChallengeStatus } from "@/api/challenges";
import { ChallengeCard } from "@/components/widgets/challenge-card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { ChallengeDialog } from "@/components/widgets/challenge-dialog";
import { NoticeCard } from "./notice-card";

export default function Index() {
    const { currentGame, selfTeam: selfGameTeam } = useGameStore();
    const [gameChallenges, setGameChallenges] =
        useState<Array<GameChallenge>>();
    const [challengeStatus, setChallengeStatus] =
        useState<Record<string, ChallengeStatus>>();
    const [total, setTotal] = useState<number>(0);
    const [page, setPage] = useState<number>(1);
    const [size, setSize] = useState<number>(20);

    const [selectedChallenge, setSelectedChallenge] = useState<GameChallenge>();
    const [challengeDialogOpen, setChallengeDialogOpen] =
        useState<boolean>(false);

    useEffect(() => {
        getGameChallenges({
            game_id: currentGame?.id!,
            is_enabled: true,
            page,
            size,
        }).then((res) => {
            setTotal(total);
            setGameChallenges(res.data);
        });
    }, [currentGame?.id, size, page]);

    useEffect(() => {
        if (gameChallenges) {
            getChallengeStatus({
                challenge_ids: gameChallenges?.map(
                    (gameChallenge) => gameChallenge?.challenge_id!
                ),
                team_id: selfGameTeam?.id,
                game_id: currentGame?.id,
            }).then((res) => {
                setChallengeStatus(res.data);
            });
        }
    }, [gameChallenges]);

    return (
        <>
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
                <div className={cn(["lg:w-3/4"])}>
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
                            <ChallengeCard
                                key={index}
                                challenge={gameChallenge?.challenge}
                                status={
                                    challengeStatus?.[
                                        gameChallenge?.challenge_id!
                                    ]
                                }
                                onClick={() => {
                                    setSelectedChallenge(gameChallenge);
                                    setChallengeDialogOpen(true);
                                }}
                            />
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
            <Dialog
                open={challengeDialogOpen}
                onOpenChange={setChallengeDialogOpen}
            >
                <DialogContent>
                    <ChallengeDialog
                        challenge={selectedChallenge?.challenge}
                        gameTeam={selfGameTeam}
                        frozenAt={selectedChallenge?.frozen_at}
                    />
                </DialogContent>
            </Dialog>
        </>
    );
}
