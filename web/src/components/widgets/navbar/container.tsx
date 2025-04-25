import { cn } from "@/utils";
import { Button } from "../../ui/button";
import { AppearanceDropdown } from "./apperance-dropdown";
import { AuthSection } from "./auth-section";
import { TitleSection } from "./title-section";
import { TabSection } from "./tab-section";
import { useAuthStore } from "@/storages/auth";
import { Group } from "@/models/user";
import { Link } from "react-router";
import { Settings } from "lucide-react";

function Container() {
    const authStore = useAuthStore();

    return (
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
                        <Button asChild icon={Settings} size={"sm"} square>
                            <Link to={"/admin/platform"} />
                        </Button>
                    )}
                    <AuthSection />
                </div>
            </div>
        </header>
    );
}

export { Container };
