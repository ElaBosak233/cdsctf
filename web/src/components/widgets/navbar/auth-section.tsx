import { StatusCodes } from "http-status-codes";
import { LogOutIcon, SettingsIcon, UserRoundIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate } from "react-router";

import { logout } from "@/api/users";
import { Avatar } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";

function AuthSection() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const authStore = useAuthStore();

  function handleLogout() {
    logout()
      .then((res) => {
        if (res.code === StatusCodes.OK) {
          navigate("/account/login");
        }
      })
      .finally(() => {
        authStore.setUser(undefined);
      });
  }

  if (authStore?.user?.id) {
    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button square>
            <Avatar
              className={cn("h-8", "w-8")}
              src={
                authStore?.user?.has_avatar &&
                `/api/users/${authStore?.user?.id}/avatar`
              }
              fallback={authStore?.user?.name?.charAt(0)}
            />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-42">
          <DropdownMenuItem
            className={cn(["flex", "items-center", "gap-2"])}
            asChild
          >
            <Link to={`/users/${authStore?.user?.id}`}>
              <Avatar
                className={cn("h-8", "w-8")}
                src={
                  authStore?.user?.has_avatar &&
                  `/api/users/${authStore?.user?.id}/avatar`
                }
                fallback={authStore?.user?.name?.charAt(0)}
              />
              <div className={cn(["flex", "flex-col"])}>
                <p className={cn(["text-sm"])}>{authStore?.user?.name}</p>
                <p className={cn(["text-xs", "text-muted-foreground"])}>
                  {`# ${authStore?.user?.username}`}
                </p>
              </div>
            </Link>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild>
            <Link to={"/account/settings"}>
              <SettingsIcon />
              {t("account.setting")}
            </Link>
          </DropdownMenuItem>
          <DropdownMenuItem
            className={cn("text-error", "hover:text-error")}
            onClick={handleLogout}
          >
            <LogOutIcon />
            {t("account.logout")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  }

  return (
    <Button asChild icon={<UserRoundIcon />}>
      <Link to={"/account/login"}>{t("account.login._")}</Link>
    </Button>
  );
}

export { AuthSection };
