import { Link, useLocation, useParams } from "react-router";
import { Context } from "./context";
import { useMemo } from "react";
import { useAuthStore } from "@/storages/auth";
import { cn } from "@/utils";
import { SettingsIcon } from "lucide-react";
import { AppearanceDropdown } from "./apperance-dropdown";
import { AuthSection } from "./auth-section";
import { TabSection } from "./tab-section";
import { TitleSection } from "./title-section";
import { Button } from "@/components/ui/button";
import { Group } from "@/models/user";

function Navbar() {
    const authStore = useAuthStore();
    const location = useLocation();
    const pathname = location.pathname;
    const { game_id } = useParams<{ game_id?: string }>();

    const mode = useMemo(() => {
        if (pathname.startsWith("/games") && game_id) {
            return "game";
        }
        return "default";
    }, [pathname, game_id]);

    return (
        <Context.Provider value={{ mode }}>
            <header
                className={cn([
                    "sticky",
                    "top-0",
                    "h-16",
                    "bg-card/80",
                    "backdrop-blur-xs",
                    "select-none",
                    "border-b-[1px]",
                    "flex",
                    "items-center",
                    "z-10",
                ])}
            >
                <div
                    className={cn([
                        "container",
                        "ml-auto",
                        "mr-auto",
                        "pl-5",
                        "pr-5",
                        "max-w-[1300px]",
                        "flex",
                        "items-center",
                        "justify-between",
                    ])}
                >
                    <div className={cn(["flex", "gap-10"])}>
                        <Button asChild size={"lg"} className={"px-5"}>
                            <TitleSection />
                        </Button>
                        <div
                            className={cn([
                                "hidden",
                                "lg:flex",
                                "gap-3",
                                "items-center",
                            ])}
                        >
                            <TabSection />
                        </div>
                    </div>
                    <div className={cn(["flex", "gap-3", "items-center"])}>
                        <AppearanceDropdown />
                        {authStore?.user?.group === Group.Admin && (
                            <Button
                                asChild
                                icon={SettingsIcon}
                                size={"sm"}
                                square
                            >
                                <Link to={"/admin/platform"} />
                            </Button>
                        )}
                        <AuthSection />
                    </div>
                </div>
            </header>
        </Context.Provider>
    );
}

export { Navbar };
