import React, { useEffect, useMemo, useState } from "react";
import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { Challenge, ChallengeMini } from "@/models/challenge";
import { Separator } from "@/components/ui/separator";
import { MarkdownRender } from "@/components/utils/markdown-render";
import { Context } from "./context";
import { SubmitSection } from "./submit-section";
import { Team } from "@/models/team";
import { EnvSection } from "./env-section";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { DownloadIcon, SnowflakeIcon } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { getChallenge } from "@/api/challenges/challenge_id";
import { getChallenge as getChallengeDebug } from "@/api/admin/challenges/challenge_id";
import { getCategory } from "@/utils/category";
import { Typography } from "@/components/ui/typography";

interface ChallengeDialogProps extends React.ComponentProps<typeof Card> {
    digest?: ChallengeMini;
    gameTeam?: Team;
    frozenAt?: number;
    debug?: boolean;
}

function ChallengeDialog(props: ChallengeDialogProps) {
    const { digest, gameTeam, frozenAt, debug = false, ...rest } = props;

    const [challenge, setChallenge] = useState<Challenge>();

    useEffect(() => {
        if (!digest?.id) return;

        const fetchChallenge = async () => {
            const res = debug
                ? await getChallengeDebug({ id: digest.id })
                : await getChallenge({ id: digest.id });
            setChallenge(res.data);
        };

        fetchChallenge();
    }, [digest?.id, debug]);

    const category = useMemo(
        () => getCategory(digest?.category!),
        [digest?.category]
    );
    const CategoryIcon = category?.icon!;

    return (
        <Context.Provider value={{ challenge: digest, team: gameTeam }}>
            <Card
                className={cn([
                    "p-6",
                    "min-h-128",
                    "w-screen",
                    "md:w-3xl",
                    "flex",
                    "flex-col",
                    "gap-5",
                ])}
                {...rest}
            >
                <div className={cn("flex", "flex-col", "gap-3")}>
                    <div
                        className={cn([
                            "flex",
                            "items-center",
                            "justify-between",
                        ])}
                    >
                        <div className={cn(["flex", "gap-3", "items-center"])}>
                            <CategoryIcon
                                color={category?.color}
                                className={cn(["size-5"])}
                            />
                            <h3>{digest?.title}</h3>
                        </div>
                        {frozenAt && (
                            <Badge className={cn(["flex"])}>
                                <SnowflakeIcon />
                                <span>
                                    {`冻结 ${new Date(frozenAt * 1000).toLocaleString()}`}
                                </span>
                            </Badge>
                        )}
                    </div>
                    <Separator />
                </div>
                <ScrollArea
                    className={cn(["flex-1", "max-h-144", "overflow-auto"])}
                >
                    <Typography>
                        <MarkdownRender src={challenge?.description} />
                    </Typography>
                </ScrollArea>
                {challenge?.has_attachment && (
                    <div className={cn(["flex"])}>
                        <Button asChild icon={<DownloadIcon />} size={"sm"}>
                            <a
                                target={"_blank"}
                                href={
                                    debug
                                        ? `/api/admin/challenges/${digest?.id}/attachment`
                                        : `/api/challenges/${digest?.id}/attachment`
                                }
                            >
                                附件
                            </a>
                        </Button>
                    </div>
                )}
                {challenge?.is_dynamic && <EnvSection />}
                {!debug && (
                    <div className={cn("flex", "flex-col", "gap-3")}>
                        <Separator />
                        <SubmitSection />
                    </div>
                )}
            </Card>
        </Context.Provider>
    );
}

export { ChallengeDialog };
