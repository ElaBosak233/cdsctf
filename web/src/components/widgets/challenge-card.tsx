import { Flag, LockIcon } from "lucide-react";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { useLocation } from "react-router";
import type { ChallengeStatus } from "@/api/challenges";
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
          <TooltipTrigger render={<span className={cn(["text-sm"])} />}>
            {t("submission:solves", {
              count: debug ? NaN : status?.solved_times || 0,
            })}
          </TooltipTrigger>
          {!!status?.solved_times && (
            <TooltipContent
              side={"bottom"}
              className={cn([
                "flex",
                "flex-col",
                "gap-2",
                "px-4",
                "py-2.5",
                "rounded-lg",
              ])}
              onClick={(e) => e.stopPropagation()}
            >
              {status?.bloods?.map((blood, index) => (
                <div
                  key={blood.id}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <div className={cn(["flex", "flex-col"])}>
                    <div className={cn(["flex", "gap-2"])}>
                      <span className={cn(["font-semibold", "font-mono"])}>
                        {getOrdinal(index + 1)}
                      </span>
                      <span className={cn(["text-xs"])}>
                        {pathname.startsWith("/games")
                          ? blood?.team_name
                          : blood?.user_name}
                      </span>
                    </div>
                    <span className={cn(["text-secondary", "text-xs"])}>
                      {parseTimestamp(blood?.created_at)?.toLocaleString() ??
                        "-"}
                    </span>
                  </div>
                </div>
              ))}
            </TooltipContent>
          )}
        </Tooltip>

        {!!status?.pts && (
          <span className={cn(["font-mono"])}>
            {pathname.startsWith("/games") && status?.pts} pts
          </span>
        )}
      </div>
    </Card>
  );
}

export { ChallengeCard };
