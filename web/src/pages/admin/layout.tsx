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
import {
  hasPermission,
  MANAGEMENT_CHALLENGE_PERMISSION,
  MANAGEMENT_CONFIG_PERMISSION,
  MANAGEMENT_GAME_PERMISSION,
  MANAGEMENT_IDP_PERMISSION,
  MANAGEMENT_USER_PERMISSION,
} from "@/models/user";
import { useAuthStore } from "@/storages/auth";
import { isSubRoute } from "@/utils/route";

export default function Layout() {
  const { t } = useTranslation();
  const { user } = useAuthStore();

  const location = useLocation();
  const pathname = location.pathname;

  const options = [
    {
      link: "/admin",
      name: t("admin:home._"),
      icon: <GaugeIcon />,
      permission: undefined,
    },
    {
      link: "/admin/platform",
      name: t("admin:platform._"),
      icon: <PencilLineIcon />,
      permission: MANAGEMENT_CONFIG_PERMISSION,
    },
    {
      link: "/admin/challenges",
      name: t("challenge:_"),
      icon: <LibraryIcon />,
      permission: MANAGEMENT_CHALLENGE_PERMISSION,
    },
    {
      link: "/admin/games",
      name: t("game:_"),
      icon: <FlagIcon />,
      permission: MANAGEMENT_GAME_PERMISSION,
    },
    {
      link: "/admin/users",
      name: t("user:_"),
      icon: <UserRoundIcon />,
      permission: MANAGEMENT_USER_PERMISSION,
    },
    {
      link: "/admin/idps",
      name: t("admin:idp._"),
      icon: <IdCardIcon />,
      permission: MANAGEMENT_IDP_PERMISSION,
    },
    {
      link: "/admin/mailbox",
      name: t("admin:mailbox._"),
      icon: <MailCheckIcon />,
      permission: MANAGEMENT_CONFIG_PERMISSION,
    },
    {
      link: "/admin/captcha",
      name: t("admin:captcha._"),
      icon: <BotIcon />,
      permission: MANAGEMENT_CONFIG_PERMISSION,
    },
  ].filter(
    ({ permission }) =>
      !permission || hasPermission(user?.permissions, permission)
  );

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
                    render={<Link to={option.link} />}
                    isActive={isSubRoute(option.link, pathname, "/admin")}
                    className="h-9 justify-start px-2.5 [&>span]:group-data-[collapsible=icon]:hidden"
                  >
                    {option.icon}
                    <span>{option.name}</span>
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
              render={<Link to={option.link} />}
            >
              {option.name}
            </Button>
          ))}
        </ScrollableNav>
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
  );
}
