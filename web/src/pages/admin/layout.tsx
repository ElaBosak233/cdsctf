import {
  BotIcon,
  FlagIcon,
  GaugeIcon,
  IdCardIcon,
  LibraryIcon,
  MailCheckIcon,
  PencilLineIcon,
  UserRoundIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/utils";
import { isSubRoute } from "@/utils/route";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;

  const options = [
    {
      link: "/admin",
      name: t("admin:home._"),
      icon: <GaugeIcon />,
    },
    {
      link: "/admin/platform",
      name: t("admin:platform._"),
      icon: <PencilLineIcon />,
    },
    {
      link: "/admin/challenges",
      name: t("challenge:_"),
      icon: <LibraryIcon />,
    },
    {
      link: "/admin/games",
      name: t("game:_"),
      icon: <FlagIcon />,
    },
    {
      link: "/admin/users",
      name: t("user:_"),
      icon: <UserRoundIcon />,
    },
    {
      link: "/admin/idps",
      name: t("admin:idp._"),
      icon: <IdCardIcon />,
    },
    {
      link: "/admin/mailbox",
      name: t("admin:mailbox._"),
      icon: <MailCheckIcon />,
    },
    {
      link: "/admin/captcha",
      name: t("admin:captcha._"),
      icon: <BotIcon />,
    },
  ];

  return (
    <div className={cn(["flex", "flex-1", "min-h-0"])}>
      <div
        className={cn([
          "hidden",
          "lg:flex",
          "w-16",
          "h-(--app-content-height)",
          "sticky",
          "top-16",
          "bg-card/30",
          "border-r",
          "p-4",
          "flex-col",
          "items-center",
          "gap-4",
        ])}
      >
        {options?.map((option) => {
          return (
            <Tooltip key={option.link}>
              <TooltipTrigger>
                <Button
                  icon={option.icon}
                  square
                  size={"sm"}
                  variant={
                    isSubRoute(option.link, pathname, "/admin")
                      ? "tonal"
                      : "ghost"
                  }
                  asChild
                >
                  <Link to={option.link} />
                </Button>
              </TooltipTrigger>
              <TooltipContent side={"right"}>{option.name}</TooltipContent>
            </Tooltip>
          );
        })}
      </div>
      <div
        className={cn([
          "flex-1",
          "min-w-0",
          "flex",
          "flex-col",
          "min-h-0",
          "min-h-(--app-content-height)",
        ])}
      >
        <ScrollableNav className={cn(["lg:hidden"])}>
          {options.map((option) => (
            <Button
              key={option.link}
              icon={option.icon}
              size="sm"
              className={cn(["shrink-0"])}
              variant={
                isSubRoute(option.link, pathname, "/admin") ? "tonal" : "ghost"
              }
              asChild
            >
              <Link to={option.link}>{option.name}</Link>
            </Button>
          ))}
        </ScrollableNav>
        <Outlet />
      </div>
    </div>
  );
}
