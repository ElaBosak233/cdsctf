import { Card } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { LogIn, UserRoundPlus } from "lucide-react";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import { LoginForm } from "./login-form";
import { Link } from "react-router";

export default function Index() {
    const configStore = useConfigStore();

    return (
        <div
            className={cn(["flex-1", "flex", "items-center", "justify-center"])}
        >
            <Card
                className={cn(["p-2", "w-[50rem]", "flex", "justify-between"])}
            >
                <div className={cn(["flex-1/2", "flex", "flex-col"])}>
                    <div
                        className={cn([
                            "flex",
                            "flex-col",
                            "space-y-1.5",
                            "p-6",
                        ])}
                    >
                        <div
                            className={cn([
                                "text-2xl",
                                "font-semibold",
                                "leading-none",
                                "tracking-tight",
                                "flex",
                                "gap-2",
                                "items-center",
                            ])}
                        >
                            <LogIn />
                            登录
                        </div>
                        <div
                            className={cn([
                                "text-sm",
                                "text-secondary-foreground",
                            ])}
                        >
                            登录以继续浏览 {configStore?.config?.meta?.title}
                        </div>
                        <div className={cn(["pt-6"])}>
                            <LoginForm />
                        </div>
                    </div>
                </div>
                <Separator
                    orientation={"vertical"}
                    className={cn(["h-81", "my-auto"])}
                />
                <div className={cn(["flex-1/2", "flex", "flex-col", "p-6"])}>
                    <div
                        className={cn([
                            "flex",
                            "flex-col",
                            "flex-1",
                            "items-center",
                            "justify-center",
                            "select-none",
                        ])}
                    >
                        <img
                            alt="logo"
                            decoding={"async"}
                            src={"/api/configs/logo"}
                            draggable={false}
                            className={cn([
                                "drop-shadow-md",
                                "aspect-square",
                                "h-[8rem]",
                            ])}
                        />
                        <span className={cn(["text-2xl", "font-semibold"])}>
                            {configStore?.config?.meta?.title}
                        </span>
                    </div>
                    {configStore?.config?.auth?.is_registration_enabled && (
                        <Button
                            asChild
                            className={cn("w-full")}
                            size={"lg"}
                            variant={"tonal"}
                            icon={UserRoundPlus}
                        >
                            <Link to={"/account/register"}>
                                还没有账号？注册！
                            </Link>
                        </Button>
                    )}
                </div>
            </Card>
        </div>
    );
}
