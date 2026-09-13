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
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
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
    <SidebarProvider
      defaultOpen={false}
      className="min-h-(--app-content-height)"
    >
      <Sidebar collapsible="icon" className="top-16 h-(--app-content-height)">
        <SidebarContent className="p-[14px]">
          <SidebarGroup className="p-0">
            <SidebarMenu>
              {options.map((option) => (
                <SidebarMenuItem key={option.link}>
                  <SidebarMenuButton
                    asChild
                    isActive={isSubRoute(option.link, pathname, "/admin")}
                    className="h-9 justify-start px-2.5 [&>span]:group-data-[collapsible=icon]:hidden"
                  >
                    <Link to={option.link}>
                      {option.icon}
                      <span>{option.name}</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter className="p-[14px]">
          <SidebarTrigger
            aria-label="Toggle administration navigation"
            className="w-full justify-start px-2.5"
          />
        </SidebarFooter>
      </Sidebar>
      <SidebarInset className="min-h-(--app-content-height)">
        <ScrollableNav className="lg:hidden">
          {options.map((option) => (
            <Button
              key={option.link}
              icon={option.icon}
              size="sm"
              className="shrink-0"
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
      </SidebarInset>
    </SidebarProvider>
  );
}
