import { cn } from "@/utils";
import { LoaderCircleIcon } from "lucide-react";

function HydrateFallback() {
    return (
        <div
            className={cn([
                "h-screen",
                "flex",
                "flex-col",
                "justify-center",
                "items-center",
                "gap-3",
            ])}
        >
            <LoaderCircleIcon className={cn(["animate-spin", "size-10"])} />
            <span>加载中</span>
        </div>
    );
}

export { HydrateFallback };
