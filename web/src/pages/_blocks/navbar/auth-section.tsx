import { LogOutIcon, SettingsIcon, UserRoundIcon } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, useLocation, useNavigate } from "react-router";

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
import { clearAuthenticatedUser, useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { getLoginTarget, getLoginUrl } from "@/utils/redirect";

function AuthSection() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  const user = useAuthStore((state) => state.user);

  async function handleLogout() {
    try {
      await logout();
    } finally {
      clearAuthenticatedUser();
      navigate("/account/login", { replace: true });
    }
  }

  if (user?.id) {
    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button square>
            <Avatar
              className={cn("h-8", "w-8")}
              src={user.avatar_hash && `/api/media?hash=${user.avatar_hash}`}
              fallback={user.name?.charAt(0)}
            />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent sideOffset={20} className="w-42">
          <DropdownMenuItem
            className={cn(["flex", "items-center", "gap-2"])}
            asChild
          >
            <Link to={`/users/${user.id}`}>
              <Avatar
                className={cn("h-8", "w-8")}
                src={user.avatar_hash && `/api/media?hash=${user.avatar_hash}`}
                fallback={user.name?.charAt(0)}
              />
              <div className={cn(["flex", "flex-col"])}>
                <p className={cn(["text-sm", "line-clamp-2"])}>{user.name}</p>
                <p
                  className={cn([
                    "text-xs",
                    "text-muted-foreground",
                    "line-clamp-1",
                  ])}
                >
                  {`# ${user.username}`}
                </p>
              </div>
            </Link>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild>
            <Link to={"/account/settings"}>
              <SettingsIcon />
              {t("account:setting")}
            </Link>
          </DropdownMenuItem>
          <DropdownMenuItem
            className={cn("text-error", "hover:text-error")}
            onClick={handleLogout}
          >
            <LogOutIcon />
            {t("account:logout")}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  }

  return (
    <Button asChild icon={<UserRoundIcon />}>
      <Link to={getLoginUrl(getLoginTarget(location))}>
        {t("account:login._")}
      </Link>
    </Button>
  );
}

export { AuthSection };
