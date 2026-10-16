import {
  CalendarDaysIcon,
  ChartNoAxesCombinedIcon,
  HashIcon,
  ThumbsDownIcon,
  UserRoundPenIcon,
} from "lucide-react";
import { useContext } from "react";
import { useTranslation } from "react-i18next";
import { Avatar } from "@/components/ui/avatar";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { MarkdownRender } from "@/components/ui/markdown-render";
import { Separator } from "@/components/ui/separator";
import { Typography } from "@/components/ui/typography";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { parseTimestamp } from "@/utils/time";
import { CategoryChart } from "./_blocks/category-chart";
import { RecentGamesChart } from "./_blocks/recent-games-chart";
import { Context } from "./context";

export default function Index() {
  const { t } = useTranslation();

  const configStore = useConfigStore();
  const { user, statistics, statisticsPending, statisticsError } =
    useContext(Context);
  const description = user?.description?.trim();

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <div
        className={cn([
          "p-4",
          "sm:p-8",
          "lg:p-12",
          "flex",
          "flex-col",
          "flex-1",
          "gap-5",
          "min-w-0",
          "min-h-0",
        ])}
      >
        <div
          className={cn([
            "lg:hidden",
            "flex",
            "flex-col",
            "gap-4",
            "sm:flex-row",
            "sm:items-center",
          ])}
        >
          <Avatar
            className="size-16 shrink-0 sm:size-20"
            src={user?.avatar_hash && `/api/media?hash=${user.avatar_hash}`}
            fallback={user?.name?.charAt(0)}
          />
          <div className="min-w-0 flex-1">
            <h1 className="truncate text-2xl font-semibold sm:text-3xl">
              {user?.name ?? "-"}
            </h1>
            <div className="mt-1 flex flex-wrap gap-x-4 gap-y-1 text-sm text-muted-foreground">
              <span className="inline-flex items-center gap-1.5 font-mono">
                <HashIcon className="size-3.5" />
                {user?.username ?? "-"}
              </span>
              <span className="inline-flex items-center gap-1.5">
                <CalendarDaysIcon className="size-3.5" />
                {t("user:created_at")}{" "}
                {parseTimestamp(user?.created_at)?.toLocaleDateString() ?? "-"}
              </span>
            </div>
          </div>
        </div>
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2">
            <ChartNoAxesCombinedIcon />
            <span>{t("user:statistics.overview")}</span>
          </div>
          <Separator />
        </div>
        <div className="grid min-w-0 gap-5 xl:grid-cols-2">
          <CategoryChart
            solved={statistics?.solved}
            solvedByCategory={statistics?.solved_by_category}
            loading={statisticsPending}
            errorLabel={
              statisticsError ? t("user:statistics.error") : undefined
            }
            title={t("user:statistics.title")}
            description={t("user:statistics.description")}
            emptyLabel={t("user:statistics.empty")}
            totalLabel={t("user:statistics.total")}
          />
          <RecentGamesChart
            games={statistics?.games}
            loading={statisticsPending}
            errorLabel={
              statisticsError ? t("user:statistics.error") : undefined
            }
            title={t("user:statistics.recent_games")}
            description={`${t("user:statistics.solved")} / ${t("user:statistics.total_challenges")}`}
            emptyLabel={t("user:statistics.recent_games_empty")}
            solvedLabel={t("user:statistics.solved")}
            remainingLabel={t("user:statistics.remaining")}
          />
        </div>
        <div className={cn(["flex", "flex-col", "gap-3"])}>
          <div className={cn(["flex", "gap-2", "items-center"])}>
            <UserRoundPenIcon />
            <span>{t("user:description")}</span>
          </div>
          <Separator />
        </div>
        <div className={cn(["relative", "min-h-48", "flex-1"])}>
          <LoadingOverlay loading={!user} />
          {user && description ? (
            <Typography>
              <MarkdownRender src={user.description} />
            </Typography>
          ) : user ? (
            <div
              className={cn([
                "absolute",
                "inset-0",
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "gap-5",
                "select-none",
                "text-center",
                "text-muted-foreground",
              ])}
            >
              <ThumbsDownIcon className={cn(["size-12"])} />
              <span>{t("user:description_empty")}</span>
            </div>
          ) : null}
        </div>
      </div>
    </>
  );
}
