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

function useChallengeQuery(
  challengeId: number | undefined,
  debug: boolean = false
) {
  return useQuery({
    queryKey: ["challenge", challengeId, debug],
    queryFn: () =>
      debug
        ? getChallengeDebug({ id: challengeId! })
        : getChallenge({ id: challengeId! }),
    select: (response) => response.challenge,
    enabled: challengeId != null && Number.isFinite(challengeId),
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
    <Context.Provider
      value={{ challenge: challenge ?? digest, team: gameTeam, debug }}
    >
      <Card
        className={cn([
          "w-screen",
          "md:w-3xl",
          "rounded-elevated",
          "shadow-lg",
          "overflow-hidden",
          "flex",
          "flex-col",
          "min-h-128",
        ])}
        {...rest}
      >
        {/* Header */}
        <div
          className={cn([
            "p-6",
            "pb-4",
            "flex",
            "flex-col",
            "gap-3",
            "shrink-0",
          ])}
        >
          <div
            className={cn(["flex", "items-start", "justify-between", "gap-3"])}
          >
            <div className={cn(["flex", "items-start", "gap-3.5"])}>
              <div
                className={cn([
                  "flex items-center justify-center",
                  "size-10 rounded-badge",
                  "shrink-0",
                  "bg-(--badge-color)/10 text-(--badge-color)",
                ])}
                style={
                  { "--badge-color": category?.color } as React.CSSProperties
                }
              >
                <CategoryIcon className={cn(["size-5"])} />
              </div>
              <div className={cn(["flex", "flex-col", "gap-1", "pt-0.5"])}>
                <h3
                  className={cn([
                    "text-sm",
                    "font-semibold",
                    "text-foreground",
                  ])}
                >
                  {digest?.title}
                </h3>
                <span
                  className={cn([
                    "text-xs",
                    "text-muted-foreground/80",
                    "uppercase",
                  ])}
                >
                  {category?.name}
                </span>
              </div>
            </div>
            {frozenAt && (
              <div className={cn(["shrink-0", "pt-1"])}>
                <FrozenBadge frozenAt={frozenAt} />
              </div>
            )}
          </div>
          <Separator />
        </div>

        {/* Description */}
        <div className={cn(["px-6", "flex-1", "flex", "flex-col", "min-h-0"])}>
          <ScrollArea
            className={cn([
              "flex-1",
              "max-h-144",
              "overflow-auto",
              "-mx-2",
              "px-2",
            ])}
          >
            <LoadingOverlay loading={isLoading} />
            <div className={cn(["pb-6"])}>
              <Typography>
                <MarkdownRender src={challenge?.description} />
              </Typography>
            </div>
          </ScrollArea>
        </div>

        {/* Attachment section */}
        {challenge?.has_attachment && (
          <div className={cn(["px-6", "pb-4", "shrink-0"])}>
            <AttachmentSection />
          </div>
        )}

        {/* Instance section */}
        {challenge?.has_instance && (
          <div className={cn(["px-6", "pb-4", "shrink-0"])}>
            <InstanceSection />
          </div>
        )}

        {/* Submit section */}
        <div
          className={cn([
            "px-6",
            "pb-6",
            "flex",
            "flex-col",
            "gap-3",
            "shrink-0",
          ])}
        >
          <Separator />
          <SubmitSection />
        </div>
      </Card>
    </Context.Provider>
  );
}

export { ChallengeDialog };
