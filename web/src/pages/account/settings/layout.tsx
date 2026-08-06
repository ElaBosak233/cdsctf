import {
  IdCardIcon,
  InfoIcon,
  LockIcon,
  MailsIcon,
  UserRoundXIcon,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { ScrollableNav } from "@/components/ui/scrollable-nav";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;

  const options = [
    {
      link: `/account/settings`,
      name: t("user:settings.info"),
      icon: <InfoIcon />,
    },
    {
      link: `/account/settings/emails`,
      name: t("user:settings.email"),
      icon: <MailsIcon />,
    },
    {
      link: `/account/settings/password`,
      name: t("user:settings.password"),
      icon: <LockIcon />,
    },
    {
      link: `/account/settings/idps`,
      name: t("user:settings.idp"),
      icon: <IdCardIcon />,
    },
  ];

  const deleteOption = {
    link: "/account/settings/delete",
    name: t("user:settings.delete"),
    icon: <UserRoundXIcon />,
  };
  const allOptions = [...options, deleteOption];

  return (
    <div
      className={cn(["flex", "flex-col", "lg:flex-row", "min-h-0", "flex-1"])}
    >
      <ScrollableNav className={cn(["lg:hidden"])}>
        {allOptions.map((option) => (
          <Button
            key={option.link}
            icon={option.icon}
            size="sm"
            className={cn(["shrink-0"])}
            variant={pathname === option.link ? "tonal" : "ghost"}
            level={option.link === deleteOption.link ? "error" : "primary"}
            asChild
          >
            <Link to={option.link}>{option.name}</Link>
          </Button>
        ))}
      </ScrollableNav>
      <div
        className={cn([
          "hidden",
          "lg:w-1/5",
          "bg-card/30",
          "backdrop-blur-sm",
          "lg:flex",
          "flex-col",
          "gap-3",
          "p-5",
          "border-r",
          "lg:sticky",
          "lg:top-16",
          "h-(--app-content-height)",
        ])}
      >
        {options?.map((option, index) => (
          <Button
            key={index}
            size={"lg"}
            className={cn(["justify-start"])}
            icon={option.icon}
            variant={pathname === option.link ? "tonal" : "ghost"}
            asChild
          >
            <Link to={option.link}>{option.name}</Link>
          </Button>
        ))}
        <Separator />
        <div className={cn(["flex-1"])} />
        <Button
          size={"lg"}
          className={cn(["justify-start"])}
          icon={<UserRoundXIcon />}
          level={"error"}
          variant={pathname === "/account/settings/delete" ? "tonal" : "ghost"}
          asChild
        >
          <Link to={"/account/settings/delete"}>
            {t("user:settings.delete")}
          </Link>
        </Button>
      </div>
      <div className={cn(["flex-1", "min-h-0", "flex", "flex-col"])}>
        <Outlet />
      </div>
    </div>
  );
}
