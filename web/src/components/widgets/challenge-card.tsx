import { Challenge } from "@/models/challenge";
import { cn } from "@/utils";
import * as React from "react";
import { Flag } from "lucide-react";
import { Card } from "../ui/card";
import { Separator } from "../ui/separator";
import { useCategoryStore } from "@/storages/category";
import { Badge } from "../ui/badge";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";
import { ChallengeStatus } from "@/api/challenges";
import { useLocation } from "react-router";
import { getOrdinal } from "@/utils/math";

interface ChallengeCardProps extends React.ComponentProps<"div"> {
    challenge?: Challenge;
    status?: ChallengeStatus;
    debug?: boolean;
}

function ChallengeCard(props: ChallengeCardProps) {
    const { challenge, status, debug = false, className, ...rest } = props;
    const { getCategory } = useCategoryStore();
    const location = useLocation();
    const pathname = location.pathname;

    const category = React.useMemo(() => {
        return getCategory(challenge?.category);
    }, [challenge?.category]);
    const CategoryIcon = category?.icon!;

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
                    "hover:bg-card-foreground/10",
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
                            className={cn([
                                "absolute",
                                "top-[10%]",
                                "right-[7%]",
                                "size-5",
                            ])}
                            fill={category.color}
                            color={category.color}
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
                        "--color-badge": category.color,
                    } as React.CSSProperties
                }
            >
                {category.name?.toUpperCase()}
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
                {challenge?.title}
            </h3>
            <Separator className={"my-3"} />
            <div
                className={cn([
                    "flex",
                    "justify-between",
                    "items-center",
                    "h-5",
                ])}
            >
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
                                "px-5",
                            ])}
                        >
                            {status?.bloods?.map((blood, index) => (
                                <div
                                    className={cn([
                                        "flex",
                                        "items-center",
                                        "gap-3",
                                    ])}
                                >
                                    <span className={cn(["font-semibold"])}>
                                        {getOrdinal(index + 1)}
                                    </span>
                                    <div className={cn(["flex", "flex-col"])}>
                                        <span className={cn(["text-sm"])}>
                                            {pathname.startsWith("/games")
                                                ? blood?.team?.name
                                                : blood?.user?.nickname}
                                        </span>
                                        <span
                                            className={cn(["text-secondary"])}
                                        >
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
