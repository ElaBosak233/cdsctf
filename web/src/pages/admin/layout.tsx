import {
  FlagIcon,
  HousePlugIcon,
  LibraryIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { Link, Outlet, useLocation } from "react-router";

import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/utils";

export default function Layout() {
  const location = useLocation();
  const pathname = location.pathname;

  const options = [
    {
      link: "/admin",
      name: "主页",
      icon: <HousePlugIcon />,
    },
    {
      link: "/admin/platform",
      name: "平台",
      icon: <TypeIcon />,
    },
    {
      link: "/admin/challenges",
      name: "题库",
      icon: <LibraryIcon />,
    },
    {
      link: "/admin/games",
      name: "比赛",
      icon: <FlagIcon />,
    },
    {
      link: "/admin/users",
      name: "用户",
      icon: <UserRoundIcon />,
    },
    // {
    //   link: "/admin/email",
    //   name: "邮箱",
    //   icon: <MailCheckIcon />,
    // },
  ];

  return (
    <div className={cn(["flex", "flex-1", "min-h-0"])}>
      <div
        className={cn([
          "w-16",
          "h-[calc(100vh-64px)]",
          "sticky",
          "top-16",
          "bg-card/30",
          "border-r-1",
          "p-4",
          "flex",
          "flex-col",
          "items-center",
          "gap-4",
        ])}
      >
        {options?.map((option) => (
          <Tooltip key={option.link}>
            <TooltipTrigger>
              <Button
                icon={option.icon}
                square
                size={"sm"}
                variant={pathname === option?.link ? "tonal" : "ghost"}
                asChild
              >
                <Link to={option.link} />
              </Button>
            </TooltipTrigger>
            <TooltipContent side={"right"}>{option.name}</TooltipContent>
          </Tooltip>
        ))}
      </div>
      <div className={cn(["flex-1", "flex", "flex-col", "min-h-0"])}>
        <Outlet />
      </div>
    </div>
  );
}
