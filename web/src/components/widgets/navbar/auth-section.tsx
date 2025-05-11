import { logout } from "@/api/users";
import { Button } from "@/components/ui/button";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { Avatar } from "@/components/ui/avatar";
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuSeparator,
    DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { LogOut, LogOutIcon, SettingsIcon, UserRoundIcon } from "lucide-react";
import { useNavigate, Link } from "react-router";
import { StatusCodes } from "http-status-codes";
import { useTranslation } from "react-i18next";

function AuthSection() {
    const { t } = useTranslation("account");
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
                            src={`/api/users/${authStore?.user?.id}/avatar`}
                            fallback={authStore?.user?.name?.charAt(0)}
                        />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent className="w-42">
                    <DropdownMenuItem
                        className={cn(["flex", "items-center", "gap-2"])}
                    >
                        <Avatar
                            className={cn("h-8", "w-8")}
                            src={`/api/users/${authStore?.user?.id}/avatar`}
                            fallback={authStore?.user?.name?.charAt(0)}
                        />
                        <div className={cn(["flex", "flex-col"])}>
                            <p className={cn(["text-sm"])}>
                                {authStore?.user?.name}
                            </p>
                            <p
                                className={cn([
                                    "text-xs",
                                    "text-muted-foreground",
                                ])}
                            >
                                {`# ${authStore?.user?.username}`}
                            </p>
                        </div>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem icon={<SettingsIcon />} asChild>
                        <Link to={"/account/settings"}>{t("setting")}</Link>
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        icon={<LogOutIcon />}
                        className={cn("text-error", "hover:text-error")}
                        onClick={handleLogout}
                    >
                        {t("logout")}
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        );
    }

    return (
        <Button asChild icon={<UserRoundIcon />}>
            <Link to={"/account/login"}>{t("login._")}</Link>
        </Button>
    );
}

export { AuthSection };
