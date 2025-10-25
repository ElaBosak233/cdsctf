import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { LockIcon, UserRoundIcon } from "lucide-react";
import { useMemo } from "react";
import { Link, Outlet, useLocation, useParams } from "react-router";
import { getUsers } from "@/api/admin/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { Context } from "./context";

export default function Layout() {
  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const configStore = useConfigStore();
  const { user_id } = useParams<{ user_id: string }>();
  const { data: user } = useQuery({
    queryKey: ["admin", "user", user_id, sharedStore.refresh],
    queryFn: async () => {
      const res = await getUsers({
        id: Number(user_id),
      });
      return res?.data?.[0];
    },
    enabled: !!user_id,
    placeholderData: keepPreviousData,
  });

  const options = useMemo(() => {
    return [
      {
        link: `/admin/users/${user_id}`,
        name: "基本信息",
        icon: <UserRoundIcon />,
      },
      {
        link: `/admin/users/${user_id}/password`,
        name: "修改密码",
        icon: <LockIcon />,
      },
    ];
  }, [user_id]);

  return (
    <>
      <title>{`${user?.name} - ${configStore?.config?.meta?.title}`}</title>
      <Context.Provider value={{ user }}>
        <div
          className={cn([
            "flex",
            "flex-col",
            "xl:flex-row",
            "flex-1",
            "gap-10",
            "xl:mx-30",
          ])}
        >
          <div
            className={cn([
              "space-y-6",
              "h-fit",
              "my-10",
              "mx-10",
              "xl:mx-0",
              "xl:my-0",
              "xl:w-64",
              "xl:sticky",
              "xl:top-25",
            ])}
          >
            <div
              className={cn([
                "flex",
                "flex-wrap",
                "justify-center",
                "gap-3",
                "select-none",
              ])}
            >
              <UserRoundIcon />
              用户编辑
            </div>
            <Card className={cn(["flex", "flex-col", "p-5", "gap-3"])}>
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
            </Card>
          </div>
          <Card
            className={cn([
              "flex-1",
              "p-10",
              "border-y-0",
              "rounded-none",
              "flex",
              "flex-col",
            ])}
          >
            <Outlet />
          </Card>
        </div>
      </Context.Provider>
    </>
  );
}
