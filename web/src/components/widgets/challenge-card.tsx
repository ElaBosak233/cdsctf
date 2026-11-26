import { Flag, LockIcon } from "lucide-react";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { useLocation } from "react-router";
import type { ChallengeStatus } from "@/api/challenges";
import { Avatar } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import type { CardProps } from "@/components/ui/card";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { ChallengeSummary } from "@/models/challenge";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { getOrdinal } from "@/utils/math";
import { parseTimestamp } from "@/utils/time";

const BLOOD_RANK_STYLES = [
  "border-amber-500/30 bg-amber-500/12 text-amber-700 dark:border-amber-400/30 dark:bg-amber-400/15 dark:text-amber-300",
  "border-slate-400/35 bg-slate-500/10 text-slate-600 dark:border-slate-400/30 dark:bg-slate-400/15 dark:text-slate-300",
  "border-orange-500/30 bg-orange-500/12 text-orange-700 dark:border-orange-400/30 dark:bg-orange-400/15 dark:text-orange-300",
] as const;

type ChallengeCardProps = CardProps & {
  digest?: Pick<ChallengeSummary, "id" | "title" | "category">;
  status?: ChallengeStatus;
  hasUnreadNotice?: boolean;
  debug?: boolean;
};

function ChallengeCard(props: ChallengeCardProps) {
  const {
    digest,
    status,
    hasUnreadNotice = false,
    debug = false,
    className,
    ...rest
  } = props;

  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;

  const category = React.useMemo(
    () => getCategory(digest?.category || 1),
    [digest?.category]
  );
  const CategoryIcon = category.icon!;

  const isCheated = !debug && Boolean(status?.cheated);
  const isSolved = !debug && !isCheated && Boolean(status?.solved);
  const isGameContext = pathname.startsWith("/games");
  const bloods = !debug ? (status?.bloods ?? []) : [];
  const hasBloods = bloods.length > 0;

  return (
    <Card
      className={cn(
        [
          "w-full",
          "text-left",
          "relative",
          "select-none",
          "p-5",
          "overflow-hidden",
          "rounded-xl",
          "hover:bg-card/40",
          "dark:hover:bg-card/70",
          "hover:border-foreground/20",
          "shadow-sm",
          "hover:shadow-md",
          "transition-all",
          "duration-200",
          "cursor-pointer",
          isSolved &&
            "border-success/35 bg-[color-mix(in_oklab,var(--success)_4.5%,var(--card))] hover:border-success/55",
          isCheated &&
            "border-error/35 bg-[color-mix(in_oklab,var(--error)_4.5%,var(--card))] hover:cursor-not-allowed hover:border-error/50",
        ],
        className
      )}
      {...rest}
    >
      <span
        className={cn([
          "pointer-events-none",
          "absolute",
          "right-1/10",
          "top-1/2",
          "-translate-y-1/2",
          "translate-x-1/2",
          "opacity-5",
          "size-36",
        ])}
        aria-hidden="true"
      >
        <CategoryIcon className={cn(["size-36"])} />
      </span>
      {isCheated ? (
        <Tooltip>
          <TooltipTrigger
            render={
              <LockIcon
                className={cn([
                  "absolute",
                  "top-[10%]",
                  "right-[7%]",
                  "size-5",
                  "text-error",
                ])}
              />
            }
          />
          <TooltipContent sideOffset={6} onClick={(e) => e.stopPropagation()}>
            {t("submission:cheated")}
          </TooltipContent>
        </Tooltip>
      ) : isSolved ? (
        <Tooltip>
          <TooltipTrigger
            render={
              <Flag
                className={cn([
                  "absolute",
                  "top-[10%]",
                  "right-[7%]",
                  "size-5",
                  "text-[color-mix(in_oklab,var(--color-badge)_88%,var(--foreground))]",
                  "dark:text-[color-mix(in_oklab,var(--color-badge)_62%,white)]",
                ])}
                style={
                  {
                    "--color-badge": category?.color,
                  } as React.CSSProperties
                }
                fill="currentColor"
              />
            }
          />
          <TooltipContent sideOffset={6} onClick={(e) => e.stopPropagation()}>
            {t("submission:solved")}
          </TooltipContent>
        </Tooltip>
      ) : !debug && hasUnreadNotice ? (
        <Tooltip>
          <TooltipTrigger
            render={
              <span
                className="absolute top-[10%] right-[7%] flex size-3.5 items-center justify-center"
                aria-label={t("game:notice.challenge_unread_tip")}
              >
                <span className="absolute inline-flex size-full rounded-full bg-warning/60 motion-safe:animate-ping" />
                <span className="relative inline-flex size-2.5 rounded-full bg-warning ring-2 ring-card" />
              </span>
            }
          />
          <TooltipContent sideOffset={6} onClick={(e) => e.stopPropagation()}>
            {t("game:notice.challenge_unread_tip")}
          </TooltipContent>
        </Tooltip>
      ) : null}
      <Badge
        variant="tonal"
        className={cn([
          "border",
          "bg-[color-mix(in_oklab,var(--color-badge)_12%,var(--card))]",
          "border-[color-mix(in_oklab,var(--color-badge)_24%,transparent)]",
          "text-[color-mix(in_oklab,var(--color-badge)_88%,var(--foreground))]",
          "dark:bg-[color-mix(in_oklab,var(--color-badge)_22%,var(--card))]",
          "dark:border-[color-mix(in_oklab,var(--color-badge)_40%,transparent)]",
          "dark:text-[color-mix(in_oklab,var(--color-badge)_62%,white)]",
        ])}
        style={
          {
            "--color-badge": category?.color,
          } as React.CSSProperties
        }
      >
        {category?.name?.toUpperCase()}
      </Badge>
      <h3 className={cn(["my-2", "text-xl", "truncate"])} title={digest?.title}>
        {digest?.title}
      </h3>
      <Separator className={"my-3"} />
      <div className={cn(["flex", "justify-between", "items-center", "h-5"])}>
        <Tooltip>
          <TooltipTrigger
            render={
              <span
                className={cn(
                  "text-sm",
                  hasBloods &&
                    "underline decoration-dotted decoration-muted-foreground/45 underline-offset-4 transition-colors hover:decoration-foreground/60"
                )}
              />
            }
          >
            {t("submission:solves", {
              count: debug ? NaN : status?.solved_times || 0,
            })}
          </TooltipTrigger>
          {hasBloods && (
            <TooltipContent
              side="bottom"
              align="start"
              showArrow={false}
              className="flex min-w-48 max-w-60 flex-col items-stretch gap-0 rounded-lg bg-popover px-2.5 py-1 text-popover-foreground shadow-md ring-1 ring-foreground/10"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="flex flex-col divide-y divide-border/60">
                {bloods.map((blood, index) => {
                  const name =
                    (isGameContext ? blood.team_name : blood.user_name) || "-";
                  const avatarHash = isGameContext
                    ? blood.team_avatar_hash
                    : blood.user_avatar_hash;
                  const rankTone =
                    BLOOD_RANK_STYLES[index] ??
                    "border-border bg-muted text-muted-foreground";

                  return (
                    <div
                      key={blood.id}
                      className="flex items-center gap-2 py-1.5"
                    >
                      <span
                        className={cn(
                          "inline-flex h-4.5 w-7 shrink-0 items-center justify-center rounded-sm border font-mono text-[10px] font-semibold tabular-nums",
                          rankTone
                        )}
                      >
                        {getOrdinal(index + 1)}
                      </span>
                      <Avatar
                        className="size-6 shrink-0 text-[10px] font-semibold"
                        src={
                          avatarHash
                            ? `/api/media?hash=${avatarHash}`
                            : undefined
                        }
                        fallback={name.charAt(0)}
                      />
                      <div className="flex min-w-0 flex-1 flex-col">
                        <span className="truncate text-xs leading-tight font-medium text-popover-foreground">
                          {name}
                        </span>
                        <span className="truncate font-mono text-[10px] leading-tight text-muted-foreground tabular-nums">
                          {parseTimestamp(blood.created_at)?.toLocaleString() ??
                            "-"}
                        </span>
                      </div>
                    </div>
                  );
                })}
              </div>
            </TooltipContent>
          )}
        </Tooltip>

        {!!status?.pts && (
          <span className={cn(["font-mono"])}>
            {isGameContext && status?.pts} pts
          </span>
        )}
      </div>
    </Card>
  );
}

export { ChallengeCard };
