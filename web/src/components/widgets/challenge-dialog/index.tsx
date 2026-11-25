import { useQuery } from "@tanstack/react-query";
import { BellIcon } from "lucide-react";
import type React from "react";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { getChallenge } from "@/api/challenges/challenge_id";
import { getChallenge as getChallengeDebug } from "@/api/management/challenges/challenge_id";
import { Card } from "@/components/ui/card";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import type { ChallengeSummary } from "@/models/challenge";
import type { GameNoticeView } from "@/models/game_notice";
import type { PlayerTeamView } from "@/models/team";
import { isNoticeEdited } from "@/storages/notice";
import type { Timestamp } from "@/types";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { parseTimestamp } from "@/utils/time";
import { AttachmentSection } from "./attachment-section";
import { Context } from "./context";
import { FrozenBadge } from "./frozen-badge";
import { InstanceSection } from "./instance-section";
import { SubmitSection } from "./submit-section";

type ChallengeDialogProps = React.ComponentProps<typeof Card> & {
  digest?: Pick<ChallengeSummary, "id" | "title" | "category">;
  gameTeam?: PlayerTeamView;
  frozenAt?: Timestamp | null;
  notices?: GameNoticeView[];
  debug?: boolean;
  cheated?: boolean;
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
  const {
    digest,
    gameTeam,
    frozenAt,
    notices,
    debug = false,
    cheated = false,
    ...rest
  } = props;

  const { t } = useTranslation();
  const { data: challenge, isLoading } = useChallengeQuery(digest?.id, debug);

  const category = useMemo(
    () => getCategory(digest?.category || 1),
    [digest?.category]
  );
  const CategoryIcon = category.icon!;

  return (
    <Context.Provider
      value={{ challenge: challenge ?? digest, team: gameTeam, debug, cheated }}
    >
      <Card
        className={cn([
          "w-full",
          "max-w-4xl",
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
              {notices && notices.length > 0 && (
                <section
                  className="mb-5 overflow-hidden rounded-lg border border-warning/30 bg-[color-mix(in_oklab,var(--warning)_7%,var(--card))]"
                  aria-label={t("game:notice.challenge_notices_title")}
                >
                  <div className="flex items-center gap-2 border-b border-warning/20 px-4 py-2.5">
                    <BellIcon
                      className="size-3.5 shrink-0 text-warning"
                      aria-hidden="true"
                    />
                    <span className="text-xs font-semibold text-foreground">
                      {t("game:notice.challenge_notices_title")}
                    </span>
                    {notices.length > 1 && (
                      <span className="rounded-full bg-warning/15 px-1.5 py-0.5 font-mono text-[11px] font-medium text-warning tabular-nums">
                        {notices.length}
                      </span>
                    )}
                  </div>
                  <div className="divide-y divide-warning/15">
                    {notices.map((notice) => (
                      <article key={notice.id} className="px-4 py-3">
                        <div className="mb-1.5 flex flex-wrap items-baseline justify-between gap-2">
                          <h4 className="text-sm font-semibold text-foreground">
                            {notice.title}
                          </h4>
                          <span className="font-mono text-[11px] text-muted-foreground tabular-nums">
                            {parseTimestamp(
                              notice.created_at
                            )?.toLocaleString() ?? "-"}
                            {isNoticeEdited(notice) &&
                              ` · ${t("game:notice.edited")}`}
                          </span>
                        </div>
                        <Typography className="prose-sm">
                          <MarkdownRender src={notice.content} />
                        </Typography>
                      </article>
                    ))}
                  </div>
                </section>
              )}
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
