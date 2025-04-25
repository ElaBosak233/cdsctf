import * as React from "react";
import * as RadixScrollArea from "@radix-ui/react-scroll-area";

import { cn } from "@/utils";

function ScrollArea({
    className,
    children,
    ...props
}: React.ComponentProps<typeof RadixScrollArea.Root>) {
    return (
        <RadixScrollArea.Root
            data-slot="scroll-area"
            className={cn("relative", className)}
            {...props}
        >
            <RadixScrollArea.Viewport
                data-slot="scroll-area-viewport"
                className={cn([
                    "ring-ring/10",
                    "dark:ring-ring/20",
                    "dark:outline-ring/40",
                    "outline-ring/50",
                    "size-full",
                    "rounded-[inherit]",
                    "[&>div]:!block",
                    "transition-[color,box-shadow]",
                    "focus-visible:ring-4",
                    "focus-visible:outline-1",
                ])}
            >
                {children}
            </RadixScrollArea.Viewport>
            <ScrollBar />
            <RadixScrollArea.Corner />
        </RadixScrollArea.Root>
    );
}

function ScrollBar({
    className,
    orientation = "vertical",
    ...props
}: React.ComponentProps<typeof RadixScrollArea.ScrollAreaScrollbar>) {
    return (
        <RadixScrollArea.ScrollAreaScrollbar
            data-slot="scroll-area-scrollbar"
            orientation={orientation}
            className={cn(
                "flex touch-none p-px transition-colors select-none z-10",
                orientation === "vertical" &&
                    "h-full w-2 border-l border-l-transparent",
                orientation === "horizontal" &&
                    "h-2 flex-col border-t border-t-transparent",
                className
            )}
            {...props}
        >
            <RadixScrollArea.ScrollAreaThumb
                data-slot="scroll-area-thumb"
                className="bg-foreground/30 relative flex-1 rounded-full hover:bg-foreground/40 active:bg-foreground/50 transition-colors"
            />
        </RadixScrollArea.ScrollAreaScrollbar>
    );
}

export { ScrollArea, ScrollBar };
