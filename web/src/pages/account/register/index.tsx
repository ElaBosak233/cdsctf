import { Card } from "@/components/ui/card";
import { LogIn, UserRoundPlus, UserRoundPlusIcon } from "lucide-react";
import { useConfigStore } from "@/storages/config";
import { cn } from "@/utils";
import { Button } from "@/components/ui/button";
import { RegisterForm } from "./register-form";

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
                            <UserRoundPlusIcon />
                            注册
                        </div>
                        <div
                            className={cn([
                                "text-sm",
                                "text-secondary-foreground",
                            ])}
                        >
                            注册以继续浏览 {configStore?.config?.meta?.title}
                        </div>
                        <div className={cn(["pt-6"])}>
                            <RegisterForm />
                        </div>
                    </div>
                </div>
            </Card>
        </div>
    );
}
