import { useQuery } from "@tanstack/react-query";
import { AwardIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Outlet, useParams } from "react-router";
import { getUser } from "@/api/users/user_id";
import { getUserStatistics } from "@/api/users/user_id/statistics";
import { Avatar } from "@/components/ui/avatar";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";
import { getCategory } from "@/utils/category";
import { parseRouteNumericId } from "@/utils/query";
import { parseTimestamp } from "@/utils/time";
import { Context } from "./context";

function useUserQuery(userId: number | undefined) {
  return useQuery({
    queryKey: ["user", userId],
    queryFn: () =>
      getUser({
        id: userId!,
      }),
    select: (response) => response.user,
    enabled: userId != null,
  });
}

function useUserStatisticsQuery(userId: number | undefined) {
  return useQuery({
    queryKey: ["user-statistics", userId],
    queryFn: () => getUserStatistics(userId!),
    select: (response) => response.statistics,
    enabled: userId != null,
  });
}

export default function Layout() {
  const { t } = useTranslation();

  const { user_id } = useParams<{ user_id: string }>();
  const parsedUserId = parseRouteNumericId(user_id);
  const { data: user } = useUserQuery(parsedUserId);
  const {
    data: statistics,
    isPending: statisticsPending,
    isError: statisticsError,
  } = useUserStatisticsQuery(parsedUserId);
  const rankedCategories = [...(statistics?.solved_by_category ?? [])]
    .sort((a, b) => b.solved - a.solved)
    .slice(0, 5);
  const maxSolved = rankedCategories[0]?.solved ?? 1;

  return (
    <Context.Provider
      value={{ user, statistics, statisticsPending, statisticsError }}
    >
      <div className={cn(["flex", "flex-1", "min-h-0"])}>
        <div
          className={cn([
            "hidden",
            "lg:sticky",
            "lg:top-16",
            "lg:h-(--app-content-height)",
            "lg:w-1/5",
            "lg:shrink-0",
            "lg:self-start",
            "bg-card/30",
            "backdrop-blur-sm",
            "lg:flex",
            "flex-col",
            "gap-5",
            "p-10",
            "border-r",
          ])}
        >
          <div className={cn(["flex", "flex-row", "items-center", "gap-5"])}>
            <Avatar
              className={cn("h-12", "w-12")}
              src={user?.avatar_hash && `/api/media?hash=${user?.avatar_hash}`}
              fallback={user?.name?.charAt(0)}
            />
            <div className={cn(["flex", "flex-col", "flex-1", "min-w-0"])}>
              <p
                className={cn([
                  "text-lg",
                  "font-semibold",
                  "text-ellipsis",
                  "whitespace-nowrap",
                  "overflow-hidden",
                ])}
              >
                {user?.name}
              </p>
              <p
                className={cn([
                  "text-md",
                  "text-muted-foreground",
                  "text-ellipsis",
                  "whitespace-nowrap",
                  "overflow-hidden",
                ])}
              >
                {`# ${user?.username}`}
              </p>
            </div>
          </div>
          <Separator />
          {statistics ? (
            <div className="grid gap-5">
              <div>
                <div className="min-w-0">
                  <div className="mb-1 flex items-center gap-1.5 text-xs text-muted-foreground">
                    <AwardIcon className="size-3.5" />
                    <span>{t("user:statistics.total")}</span>
                  </div>
                  <p className="font-mono text-xl font-semibold tabular-nums">
                    {statistics.solved.toLocaleString()}
                  </p>
                </div>
              </div>
              {rankedCategories.length ? (
                <div className="grid gap-3">
                  <p className="text-xs font-medium text-muted-foreground">
                    {t("user:statistics.top_categories")}
                  </p>
                  {rankedCategories.map((item) => {
                    const category = getCategory(item.category);
                    return (
                      <div key={item.category} className="grid gap-1.5">
                        <div className="flex items-center justify-between gap-2 text-xs">
                          <span className="truncate">{category.name}</span>
                          <span className="font-mono tabular-nums text-muted-foreground">
                            {item.solved}
                          </span>
                        </div>
                        <Progress
                          value={(item.solved / maxSolved) * 100}
                          aria-label={category.name}
                          aria-valuetext={String(item.solved)}
                          className="gap-0"
                        />
                      </div>
                    );
                  })}
                </div>
              ) : null}
            </div>
          ) : null}
          <div className={cn(["flex-1"])} />
          <span
            className={cn([
              "text-secondary-foreground",
              "text-center",
              "text-md",
              "select-none",
            ])}
          >
            {`${t("user:created_at")} ${parseTimestamp(user?.created_at)?.toLocaleDateString() ?? "-"}`}
          </span>
        </div>
        <div className={cn(["flex-1", "min-w-0", "flex", "flex-col"])}>
          <Outlet />
        </div>
      </div>
    </Context.Provider>
  );
}
