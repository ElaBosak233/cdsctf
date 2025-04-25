import { Card } from "@/components/ui/card";
import { MailIcon } from "lucide-react";
import { cn } from "@/utils";
import { ForgetForm } from "./forget-form";

export default function Index() {
    return (
        <div
            className={cn(["flex-1", "flex", "items-center", "justify-center"])}
        >
            <Card
                className={cn([
                    "p-2",
                    "w-[36rem]",
                    "flex",
                    "flex-col",
                    "space-y-1.5",
                    "p-8",
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
                    <MailIcon />
                    忘记密码
                </div>
                <div className={cn(["text-sm", "text-secondary-foreground"])}>
                    下次不要再忘记了哦
                </div>
                <div className={cn(["pt-6"])}>
                    <ForgetForm />
                </div>
            </Card>
        </div>
    );
}
