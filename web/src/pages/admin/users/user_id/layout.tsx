import { StatusCodes } from "http-status-codes";
import { LockIcon, UserRoundIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { Link, Outlet, useLocation, useParams } from "react-router";

import { Context } from "./context";

import { getUsers } from "@/api/admin/users";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { User } from "@/models/user";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";

export default function Layout() {
  const location = useLocation();
  const pathname = location.pathname;
  const sharedStore = useSharedStore();
  const { user_id } = useParams<{ user_id: string }>();
  const [user, setUser] = useState<User>();

  useEffect(() => {
    if (user_id) {
      getUsers({
        id: Number(user_id),
      }).then((res) => {
        if (res.code === StatusCodes.OK) {
          setUser(res?.data?.[0]);
        }
      });
    }
  }, [user_id, sharedStore?.refresh]);

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
  }, [user]);

  return (
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
            {options?.map((option, index) => {
              return (
                <Button
                  key={index}
                  icon={option?.icon}
                  variant={pathname === option?.link ? "tonal" : "ghost"}
                  className={cn(["justify-start"])}
                  asChild
                >
                  <Link to={option?.link}>{option?.name}</Link>
                </Button>
              );
            })}
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
  );
}
