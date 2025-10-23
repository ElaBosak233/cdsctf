import { useQuery } from "@tanstack/react-query";
import { Outlet, useParams } from "react-router";
import { getUser } from "@/api/users/user_id";
import { Avatar } from "@/components/ui/avatar";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";
import { Context } from "./context";

function useUserQuery(userId?: number) {
  return useQuery({
    queryKey: ["user", userId],
    queryFn: () =>
      getUser({
        id: userId!,
      }),
    select: (response) => response.data,
    enabled: !!userId,
  });
}

export default function Layout() {
  const { user_id } = useParams<{ user_id: string }>();
  const { data: user } = useUserQuery(Number(user_id));

  return (
    <Context.Provider value={{ user }}>
      <div className={cn(["flex", "flex-1"])}>
        <div
          className={cn([
            "hidden",
            "lg:w-1/5",
            "bg-card/30",
            "backdrop-blur-sm",
            "lg:flex",
            "flex-col",
            "gap-5",
            "p-10",
            "border-r-1",
            "lg:sticky",
            "top-16",
            "h-[calc(100vh-64px)]",
          ])}
        >
          <div className={cn(["flex", "flex-row", "items-center", "gap-5"])}>
            <Avatar
              className={cn("h-12", "w-12")}
              src={user?.has_avatar && `/api/users/${user?.id}/avatar`}
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
            {`注册于 ${new Date(Number(user?.created_at) * 1000).toLocaleDateString()}`}
          </span>
        </div>
        <div className={cn(["flex-1", "flex", "flex-col"])}>
          <Outlet />
        </div>
      </div>
    </Context.Provider>
  );
}
