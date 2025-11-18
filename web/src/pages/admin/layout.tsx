import {
  BotIcon,
  FlagIcon,
  HousePlugIcon,
  LibraryIcon,
  MailCheckIcon,
  TypeIcon,
  UserRoundIcon,
} from "lucide-react";
import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation, useNavigate } from "react-router";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Group } from "@/models/user";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);

  useEffect(() => {
    if (!user) {
      navigate(
        `/account/login?redirect=${encodeURIComponent(location.pathname + location.search)}`,
        { replace: true }
      );
    } else if ((user.group ?? 0) < Group.Admin) {
      navigate("/", { replace: true });
    }
  }, [user, navigate, location.pathname, location.search]);

  if (!user || (user.group ?? 0) < Group.Admin) {
    return null;
  }

  const options = [
    {
      link: "/admin",
      name: t("admin.home._"),
      icon: <HousePlugIcon />,
    },
    {
      link: "/admin/platform",
      name: t("admin.platform._"),
      icon: <TypeIcon />,
    },
    {
      link: "/admin/challenges",
      name: t("challenge._"),
      icon: <LibraryIcon />,
    },
    {
      link: "/admin/games",
      name: t("game._"),
      icon: <FlagIcon />,
    },
    {
      link: "/admin/users",
      name: t("user._"),
      icon: <UserRoundIcon />,
    },
    {
      link: "/admin/mailbox",
      name: t("admin.mailbox._"),
      icon: <MailCheckIcon />,
    },
    {
      link: "/admin/captcha",
      name: t("admin.captcha._"),
      icon: <BotIcon />,
    },
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
          "border-r",
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
