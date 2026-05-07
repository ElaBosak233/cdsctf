import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { Outlet, useParams } from "react-router";
import { getUser } from "@/api/users/user_id";
import { Avatar } from "@/components/ui/avatar";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
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

export default function Layout() {
  const { t } = useTranslation();

  const { user_id } = useParams<{ user_id: string }>();
  const { data: user } = useUserQuery(parseRouteNumericId(user_id));

  return (
    <Context.Provider value={{ user }}>
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
                  "overflow-ellipsis",
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
                  "overflow-ellipsis",
                  "whitespace-nowrap",
                  "overflow-hidden",
                ])}
              >
                {`# ${user?.username}`}
              </p>
            </div>
          </div>
          <Separator />
          <div className={cn(["flex-1"])} />
          <span
            className={cn([
              "text-secondary-foreground",
              "text-center",
              "text-md",
              "select-none",
            ])}
          >
            {`${t("user:created_at")} ${new Date(Number(user?.created_at) * 1000).toLocaleDateString()}`}
          </span>
        </div>
        <div className={cn(["flex-1", "min-w-0", "flex", "flex-col"])}>
          <Outlet />
        </div>
      </div>
    </Context.Provider>
  );
}
