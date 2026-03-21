import { useQuery } from "@tanstack/react-query";
import type React from "react";
import { useMemo } from "react";
import { getChallenge as getChallengeDebug } from "@/api/admin/challenges/challenge_id";
import { getChallenge } from "@/api/challenges/challenge_id";
import { Card } from "@/components/ui/card";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import type { ChallengeMini } from "@/models/challenge";
import type { Team } from "@/models/team";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { AttachmentSection } from "./attachment-section";
import { Context } from "./context";
import { FrozenBadge } from "./frozen-badge";
import { InstanceSection } from "./instance-section";
import { SubmitSection } from "./submit-section";

type ChallengeDialogProps = React.ComponentProps<typeof Card> & {
  digest?: ChallengeMini;
  gameTeam?: Team;
  frozenAt?: number;
  debug?: boolean;
};

function useChallengeQuery(challengeId?: number, debug: boolean = false) {
  return useQuery({
    queryKey: ["challenge", challengeId, debug],
    queryFn: () =>
      debug
        ? getChallengeDebug({ id: challengeId! })
        : getChallenge({ id: challengeId! }),
    select: (response) => response.items,
    enabled: !!challengeId,
  });
}

function ChallengeDialog(props: ChallengeDialogProps) {
  const { digest, gameTeam, frozenAt, debug = false, ...rest } = props;

  const { data: challenge, isLoading } = useChallengeQuery(digest?.id, debug);

  const category = useMemo(
    () => getCategory(digest?.category || 1),
    [digest?.category]
  );
  const CategoryIcon = category.icon!;

  return (
    <Context.Provider value={{ challenge: digest, team: gameTeam, debug }}>
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
          <div className={cn(["flex", "items-center", "justify-between"])}>
            <div className={cn(["flex", "gap-3", "items-center"])}>
              <CategoryIcon
                color={category?.color}
                className={cn(["size-5"])}
              />
              <h3>{digest?.title}</h3>
            </div>
            {frozenAt && <FrozenBadge frozenAt={frozenAt} />}
          </div>
          <Separator />
        </div>
        <ScrollArea className={cn(["flex-1", "max-h-144", "overflow-auto"])}>
          <LoadingOverlay loading={isLoading} />
          <Typography>
            <MarkdownRender src={challenge?.description} />
          </Typography>
        </ScrollArea>
        {challenge?.has_attachment && <AttachmentSection />}
        {challenge?.has_instance && <InstanceSection />}
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
