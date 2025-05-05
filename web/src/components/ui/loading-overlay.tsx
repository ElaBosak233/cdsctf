import { cn } from "@/utils";
import { LoaderCircleIcon } from "lucide-react";

interface LoadingOverlayProps {
    loading: boolean;
    className?: string;
}

function LoadingOverlay(props: LoadingOverlayProps) {
    const { className } = props;

    return (
        <div
            className={cn(
                "inset-0 z-50 flex items-center justify-center backdrop-blur-sm",
                className
            )}
        >
            <LoaderCircleIcon
                className={cn("h-8 w-8 animate-spin text-primary")}
            />
        </div>
    );
}

export { LoadingOverlay };
