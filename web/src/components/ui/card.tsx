import * as React from "react";

import { cn } from "@/utils";

function Card(props: React.ComponentProps<"div">) {
    const { className, ref, ...rest } = props;
    return (
        <div
            ref={ref}
            className={cn(
                [
                    "rounded-lg",
                    "border",
                    "bg-card",
                    "text-card-foreground",
                    "shadow-xs",
                ],
                className
            )}
            {...rest}
        />
    );
}

export { Card };
