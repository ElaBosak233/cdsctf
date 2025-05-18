import { Flag } from "lucide-react";
import * as React from "react";
import { useLocation } from "react-router";

import { ChallengeStatus } from "@/api/challenges";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { ChallengeMini } from "@/models/challenge";
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
          <TooltipContent sideOffset={0}>已解决</TooltipContent>
        </Tooltip>
      )}
      <Badge
        variant={"tonal"}
        className={cn([
          "bg-[var(--color-badge)]/10",
          "text-[var(--color-badge)]",
        ])}
        style={
          {
            "--color-badge": category?.color,
          } as React.CSSProperties
        }
      >
        {category?.name?.toUpperCase()}
      </Badge>
      <h3
        className={cn([
          "my-2",
          "text-xl",
          "text-ellipsis",
          "overflow-hidden",
          "text-nowrap",
          "max-w-3/4",
        ])}
      >
        {digest?.title}
      </h3>
      <Separator className={"my-3"} />
      <div className={cn(["flex", "justify-between", "items-center", "h-5"])}>
        <Tooltip>
          <TooltipTrigger asChild>
            <span className={cn(["text-sm"])}>
              {debug ? "N" : status?.solved_times || 0} 次解出
            </span>
          </TooltipTrigger>
          {!!status?.solved_times && (
            <TooltipContent
              side={"bottom"}
              className={cn([
                "flex",
                "flex-col",
                "gap-1",
                "py-3",
                "px-4",
                "rounded-xl",
              ])}
            >
              {status?.bloods?.map((blood, index) => (
                <div
                  key={index}
                  className={cn(["flex", "items-center", "gap-3"])}
                >
                  <span className={cn(["font-semibold"])}>
                    {getOrdinal(index + 1)}
                  </span>
                  <div className={cn(["flex", "flex-col"])}>
                    <span className={cn(["text-sm"])}>
                      {pathname.startsWith("/games")
                        ? blood?.team_name
                        : blood?.user_name}
                    </span>
                    <span className={cn(["text-secondary"])}>
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
