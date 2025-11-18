import { Flag } from "lucide-react";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { useLocation } from "react-router";
import type { ChallengeStatus } from "@/api/challenges";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { ChallengeMini } from "@/models/challenge";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { getOrdinal } from "@/utils/math";

type ChallengeCardProps = React.ComponentProps<"div"> & {
  digest?: ChallengeMini;
  status?: ChallengeStatus;
  debug?: boolean;
};

function ChallengeCard(props: ChallengeCardProps) {
  const { digest, status, debug = false, className, ...rest } = props;

  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;

  const category = React.useMemo(
    () => getCategory(digest?.category || 1),
    [digest?.category]
  );
  const CategoryIcon = category.icon!;

  return (
    <Card
      className={cn(
        [
          "w-full",
          "relative",
          "select-none",
          "p-5",
          "overflow-hidden",
          "rounded-xl",
          "hover:bg-card/40",
          "dark:hover:bg-card/70",
          "shadow-sm",
          "transition-colors",
          "duration-200",
          "cursor-pointer",
        ],
        className
      )}
      {...rest}
    >
      <span
        className={cn([
          "absolute",
          "right-1/10",
          "top-1/2",
          "-translate-y-1/2",
          "translate-x-1/2",
          "opacity-5",
          "size-36",
        ])}
      >
        <CategoryIcon className={cn(["size-36"])} />
      </span>
      {!debug && status?.is_solved && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Flag
              className={cn(["absolute", "top-[10%]", "right-[7%]", "size-5"])}
              fill={category?.color}
              color={category?.color}
            />
          </TooltipTrigger>
          <TooltipContent onClick={(e) => e.stopPropagation()} sideOffset={0}>
            {t("submission.solved")}
          </TooltipContent>
        </Tooltip>
      )}
      <Badge
        variant={"tonal"}
        className={cn(["bg-(--color-badge)/10", "text-(--color-badge)"])}
        style={
          {
            "--color-badge": category?.color,
          } as React.CSSProperties
        }
      >
        {category?.name?.toUpperCase()}
      </Badge>
      <h3 className={cn(["my-2", "text-xl", "truncate"])}>{digest?.title}</h3>
      <Separator className={"my-3"} />
      <div className={cn(["flex", "justify-between", "items-center", "h-5"])}>
        <Tooltip>
          <TooltipTrigger asChild>
            <span className={cn(["text-sm"])}>
              {t("submission.solves", {
                count: debug ? NaN : status?.solved_times || 0,
              })}
            </span>
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
                  key={index}
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
                      {new Date(
                        Number(blood?.created_at) * 1000
                      ).toLocaleString()}
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
