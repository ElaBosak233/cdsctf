"use client";

import * as React from "react";
import * as RadixLabel from "@radix-ui/react-label";

import { cn } from "@/utils";

function Label({
    className,
    ...props
}: React.ComponentProps<typeof RadixLabel.Root>) {
    return (
        <RadixLabel.Root
            data-slot="label"
            className={cn(
                [
                    "text-sm",
                    "leading-none",
                    "font-medium",
                    "select-none",
                    "group-data-[disabled=true]:pointer-events-none",
                    "group-data-[disabled=true]:opacity-50",
                    "peer-disabled:cursor-not-allowed",
                    "peer-disabled:opacity-50",
                ],
                className
            )}
            {...props}
        />
    );
}

export { Label };
