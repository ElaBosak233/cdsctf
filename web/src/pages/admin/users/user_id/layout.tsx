import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { LockIcon, MailIcon, UserRoundIcon } from "lucide-react";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { getUser } from "@/api/admin/users/user_id";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "./context";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const { user_id } = useParams<{ user_id: string }>();
  const userId = parseRouteNumericId(user_id);
  const { data: user } = useQuery({
    queryKey: ["admin", "user", userId, sharedStore.refresh],
    queryFn: async () => {
      const res = await getUser({ id: userId! });
      return res.user;
    },
    enabled: userId != null,
    placeholderData: keepPreviousData,
  });

  const options = useMemo(() => {
    return [
      {
        link: `/admin/users/${user_id}`,
        name: t("user:edit.info"),
        icon: <UserRoundIcon />,
      },
      {
        link: `/admin/users/${user_id}/emails`,
        name: t("user:edit.email"),
        icon: <MailIcon />,
      },
      {
        link: `/admin/users/${user_id}/password`,
        name: t("user:edit.password"),
        icon: <LockIcon />,
      },
    ];
  }, [user_id, t]);

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <Context.Provider value={{ user }}>
        <div
          className={cn([
            "flex",
            "flex-col",
            "xl:flex-row",
            "xl:min-h-(--app-content-height)",
            "flex-1",
            "min-h-0",
            "xl:pl-64",
          ])}
        >
          <nav
            className={cn([
              "xl:hidden",
              "flex",
              "flex-row",
              "flex-wrap",
              "gap-2",
              "p-3",
              "border-b",
              "bg-card/30",
              "shrink-0",
            ])}
          >
            {options?.map((option, index) => (
              <Button
                key={index}
                icon={option?.icon}
                variant={pathname === option?.link ? "tonal" : "ghost"}
                size="sm"
                className={cn(["shrink-0"])}
                asChild
              >
                <Link to={option?.link}>{option?.name}</Link>
              </Button>
            ))}
          </nav>
          <aside
            className={cn([
              "hidden",
              "xl:flex",
              "xl:fixed",
              "xl:left-16",
              "xl:top-16",
              "xl:z-10",
              "xl:h-(--app-content-height)",
              "xl:w-64",
              "xl:flex-col",
              "xl:border-r",
              "xl:bg-card/30",
              "xl:backdrop-blur-sm",
              "py-6",
              "px-4",
              "gap-4",
              "my-6",
              "mx-4",
              "xl:my-0",
              "xl:mx-0",
            ])}
          >
            <div
              className={cn([
                "flex",
                "items-center",
                "gap-2",
                "px-2",
                "text-sm",
                "font-medium",
                "text-muted-foreground",
              ])}
            >
              <UserRoundIcon className="size-4" />
              {t("user:edit._")}
            </div>
            <nav className={cn(["flex", "flex-col", "gap-1"])}>
              {options?.map((option, index) => (
                <Button
                  key={index}
                  icon={option?.icon}
                  variant={pathname === option?.link ? "tonal" : "ghost"}
                  className={cn(["justify-start"])}
                  asChild
                >
                  <Link to={option?.link}>{option?.name}</Link>
                </Button>
              ))}
            </nav>
          </aside>
          <Card
            className={cn([
              "flex-1",
              "min-w-0",
              "min-h-0",
              "p-10",
              "border-y-0",
              "rounded-none",
              "flex",
              "flex-col",
              "xl:rounded-l-none",
            ])}
          >
            <Outlet />
          </Card>
        </div>
      </Context.Provider>
    </>
  );
}
