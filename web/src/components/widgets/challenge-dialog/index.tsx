import { useQuery } from "@tanstack/react-query";
import { DownloadIcon } from "lucide-react";
import type React from "react";
import { useMemo } from "react";
import { getChallenge as getChallengeDebug } from "@/api/admin/challenges/challenge_id";
import { getChallengeAttachments as getChallengeAttachmentsDebug } from "@/api/admin/challenges/challenge_id/attachments";
import { getChallenge } from "@/api/challenges/challenge_id";
import { getChallengeAttachments } from "@/api/challenges/challenge_id/attachments";
import { Button } from "@/components/ui/button";
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
import { Context } from "./context";
import { EnvSection } from "./env-section";
import { FrozenBadge } from "./frozen-badge";
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
    select: (response) => response.data,
    enabled: !!challengeId,
  });
}

function useChallengeAttachmentsQuery(
  challengeId?: number,
  hasAttachment?: boolean,
  debug: boolean = false
) {
  return useQuery({
    queryKey: ["challenge_attachments", challengeId],
    queryFn: () =>
      debug
        ? getChallengeAttachmentsDebug(challengeId!)
        : getChallengeAttachments(challengeId!),
    select: (response) => response.data,
    enabled: !!challengeId && hasAttachment,
  });
}

function ChallengeDialog(props: ChallengeDialogProps) {
  const { digest, gameTeam, frozenAt, debug = false, ...rest } = props;

  const { data: challenge, isLoading } = useChallengeQuery(digest?.id, debug);
  const { data: metadata } = useChallengeAttachmentsQuery(
    digest?.id,
    challenge?.has_attachment,
    debug
  );

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
        {challenge?.has_attachment && (
          <div className={cn(["flex", "gap-3", "flex-wrap"])}>
            {metadata?.map((m) => (
              <Button
                asChild
                icon={<DownloadIcon />}
                size={"sm"}
                key={m.filename}
              >
                <a
                  target={"_blank"}
                  href={
                    debug
                      ? `/api/admin/challenges/${digest?.id}/attachments/${m.filename}`
                      : `/api/challenges/${digest?.id}/attachments/${m.filename}`
                  }
                >
                  {m.filename}
                </a>
              </Button>
            ))}
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
