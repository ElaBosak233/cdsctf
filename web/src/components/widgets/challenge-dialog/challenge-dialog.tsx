import React, { useEffect, useMemo, useState } from "react";
import { Card } from "../../ui/card";
import { cn } from "@/utils";
import { Challenge, ChallengeMini } from "@/models/challenge";
import { useCategoryStore } from "@/storages/category";
import { Separator } from "../../ui/separator";
import { MarkdownRender } from "../../utils/markdown-render";
import { Context } from "./context";
import { SubmitSection } from "./submit-section";
import { Team } from "@/models/team";
import { EnvSection } from "./env-section";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { DownloadIcon, SnowflakeIcon } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { getChallenge } from "@/api/challenges/challenge_id";

interface ChallengeDialogProps extends React.ComponentProps<typeof Card> {
    digest?: ChallengeMini;
    gameTeam?: Team;
    frozenAt?: number;
    debug?: boolean;
}

function ChallengeDialog(props: ChallengeDialogProps) {
    const { digest, gameTeam, frozenAt, debug = false, ...rest } = props;
    const { getCategory } = useCategoryStore();

    const [challenge, setChallenge] = useState<Challenge>();

    useEffect(() => {
        getChallenge({ id: digest?.id }).then((res) => {
            setChallenge(res.data);
        });
    }, []);

    const category = useMemo(
        () => getCategory(digest?.category),
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
                    <MarkdownRender src={challenge?.description} />
                </ScrollArea>
                {challenge?.has_attachment && (
                    <div className={cn(["flex"])}>
                        <Button asChild icon={DownloadIcon} size={"sm"}>
                            <a
                                target={"_blank"}
                                href={`/api/challenges/${digest?.id}/attachment`}
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
