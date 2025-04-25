import { Separator } from "@/components/ui/separator";
import { cn } from "@/utils";
import { CircleOff } from "lucide-react";

export default function E404() {
    return (
        <div
            className={cn([
                "flex-1",
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "gap-7",
                "text-foreground",
            ])}
        >
            <CircleOff className={cn(["size-32"])} />
            <div className={cn(["flex", "gap-2", "text-xl"])}>
                <span>404</span>
                <Separator orientation={"vertical"} />
                <span>Not Found</span>
            </div>
            <span className={cn(["text-secondary-foreground"])}>
                哎呀，好像走错地方了呢
            </span>
        </div>
    );
}
