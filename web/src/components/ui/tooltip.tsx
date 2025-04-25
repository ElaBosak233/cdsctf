import * as React from "react";
import * as RadixTooltip from "@radix-ui/react-tooltip";

import { cn } from "@/utils";

interface TooltipProps
    extends Omit<
        React.ComponentProps<typeof RadixTooltip.Provider>,
        "children"
    > {
    children?: React.ComponentProps<typeof RadixTooltip.Root>["children"];
    slotProps?: {
        root?: Partial<React.ComponentProps<typeof RadixTooltip.Root>>;
    };
}

function Tooltip(props: TooltipProps) {
    const { delayDuration = 0, children, slotProps, ...rest } = props;

    return (
        <RadixTooltip.Provider
            data-slot="tooltip-provider"
            delayDuration={delayDuration}
            {...rest}
        >
            <RadixTooltip.Root data-slot="tooltip" {...slotProps?.root}>
                {children}
            </RadixTooltip.Root>
        </RadixTooltip.Provider>
    );
}

function TooltipTrigger({
    ...props
}: React.ComponentProps<typeof RadixTooltip.Trigger>) {
    return <RadixTooltip.Trigger data-slot="tooltip-trigger" {...props} />;
}

function TooltipContent({
    className,
    sideOffset = 0,
    children,
    ...props
}: React.ComponentProps<typeof RadixTooltip.Content>) {
    return (
        <RadixTooltip.Portal>
            <RadixTooltip.Content
                data-slot="tooltip-content"
                sideOffset={sideOffset}
                className={cn(
                    "bg-primary text-primary-foreground animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit rounded-md px-3 py-1.5 text-xs text-balance",
                    className
                )}
                {...props}
            >
                {children}
                <RadixTooltip.Arrow className="bg-primary fill-primary z-50 size-2.5 translate-y-[calc(-50%_-_2px)] rotate-45 rounded-[2px]" />
            </RadixTooltip.Content>
        </RadixTooltip.Portal>
    );
}

export { Tooltip, TooltipTrigger, TooltipContent };
