import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/utils";

const badgeVariants = cva(
    [
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-full",
        "border",
        "w-fit",
        "whitespace-nowrap",
        "shrink-0",
        "[&>svg]:size-3",
        "gap-1",
        "[&>svg]:pointer-events-none",
        "focus-visible:border-ring",
        "focus-visible:ring-ring/50",
        "focus-visible:ring-[3px]",
        "aria-invalid:ring-error/20",
        "dark:aria-invalid:ring-error/40",
        "aria-invalid:border-error",
        "transition-[color,box-shadow]",
        "overflow-auto",
    ],
    {
        variants: {
            variant: {
                solid: "border-transparent bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
                tonal: "border-transparent bg-foreground/10 text-primary [a&]:hover:bg-foreground/30",
                outline:
                    "text-foreground [a&]:hover:bg-foreground/10 [a&]:hover:text-primary/30",
            },
            size: {
                sm: ["px-2", "py-0.5", "text-xs", "font-medium"],
                md: ["px-4", "py-1", "text-sm", "font-medium"],
                lg: ["px-6", "py-2", "text-md", "font-medium"],
            },
        },
        defaultVariants: {
            variant: "solid",
            size: "sm",
        },
    }
);

function Badge({
    size,
    className,
    variant,
    asChild = false,
    ...props
}: React.ComponentProps<"span"> &
    VariantProps<typeof badgeVariants> & { asChild?: boolean }) {
    const Comp = asChild ? Slot : "span";

    return (
        <Comp
            data-slot="badge"
            className={cn(badgeVariants({ size, variant, className }))}
            {...props}
        />
    );
}

export { Badge, badgeVariants };
