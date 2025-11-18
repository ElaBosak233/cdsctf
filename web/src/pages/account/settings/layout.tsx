import { InfoIcon, LockIcon, MailsIcon, UserRoundXIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, Outlet, useLocation } from "react-router";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";

export default function Layout() {
  const { t } = useTranslation();

  const location = useLocation();
  const pathname = location.pathname;

  const options = [
    {
      link: `/account/settings`,
      name: t("user.settings.info"),
      icon: <InfoIcon />,
    },
    {
      link: `/account/settings/emails`,
      name: t("user.settings.email"),
      icon: <MailsIcon />,
    },
    {
      link: `/account/settings/password`,
      name: t("user.settings.password"),
      icon: <LockIcon />,
    },
    // {
    //   link: `/account/settings/oauth`,
    //   name: "第三方认证服务",
    //   icon: <HandshakeIcon />,
    // },
  ];

  return (
    <div className={cn(["flex", "flex-1"])}>
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
          "top-16",
          "h-[calc(100vh-64px)]",
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
            {t("user.settings.delete")}
          </Link>
        </Button>
      </div>
      <div className={cn(["flex-1", "flex", "flex-col"])}>
        <Outlet />
      </div>
    </div>
  );
}
