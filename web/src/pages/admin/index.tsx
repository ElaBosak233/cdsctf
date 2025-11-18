import { keepPreviousData, useQuery } from "@tanstack/react-query";
import {
  FlagIcon,
  HousePlugIcon,
  LibraryIcon,
  SendIcon,
  UserRoundIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { getStatistics } from "@/api/admin/configs";
import { Card } from "@/components/ui/card";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";

function useStatisticsQuery() {
  return useQuery({
    queryKey: ["statistics"],
    queryFn: () => getStatistics(),
    select: (response) => response.data,
    placeholderData: keepPreviousData,
  });
}

type StatCardProps = {
  icon: React.ReactNode;
  label: string;
  value: number | string | undefined;
};

function StatCard({ icon, label, value }: StatCardProps) {
  return (
    <Card
      className={cn([
        "p-4",
        "flex",
        "flex-col",
        "justify-between",
        "shadow-sm",
        "hover:shadow-lg",
        "transition-all",
        "rounded-xl",
      ])}
    >
      <div className={cn(["flex", "items-center", "justify-between", "mb-2"])}>
        <span className={cn(["text-sm", "text-muted-foreground"])}>
          {label}
        </span>
        <span className={cn(["text-muted-foreground"])}>{icon}</span>
      </div>
      <div className={cn(["text-2xl", "font-bold"])}>{value ?? "—"}</div>
    </Card>
  );
}

export default function AdminDashboard() {
  const { t } = useTranslation();

  const configStore = useConfigStore();

  const { data: statistics } = useStatisticsQuery();

  return (
    <>
      <title>{`${t("admin.home._")} - ${configStore?.config?.meta?.title}`}</title>
      <div className={cn(["p-10", "xl:mx-60", "lg:mx-30"])}>
        <h1
          className={cn([
            "text-2xl",
            "font-bold",
            "flex",
            "gap-2",
            "items-center",
            "mb-6",
          ])}
        >
          <HousePlugIcon />
          {t("admin.home._")}
        </h1>
        <div
          className={cn([
            "grid",
            "grid-cols-1",
            "sm:grid-cols-2",
            "lg:grid-cols-4",
            "gap-6",
          ])}
        >
          <StatCard
            icon={<UserRoundIcon className={cn(["size-5"])} />}
            label={t("admin.home.statistics.users")}
            value={statistics?.users}
          />
          <StatCard
            icon={<FlagIcon className={cn(["size-5"])} />}
            label={t("admin.home.statistics.games")}
            value={statistics?.games}
          />
          <StatCard
            icon={<LibraryIcon className={cn(["size-5"])} />}
            label={t("admin.home.statistics.challenges")}
            value={statistics?.challenges?.total}
          />
          <StatCard
            icon={<SendIcon className={cn(["size-5"])} />}
            label={t("admin.home.statistics.submissions")}
            value={statistics?.submissions?.total}
          />
        </div>
      </div>
    </>
  );
}
