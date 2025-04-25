import * as React from "react";
import * as RadixAvatar from "@radix-ui/react-avatar";

import { cn } from "@/utils";
import { cva, VariantProps } from "class-variance-authority";

const avatarVariants = cva(
    ["relative", "flex", "h-10", "w-10", "shrink-0", "overflow-hidden"],
    {
        variants: {
            square: {
                false: "rounded-full",
                true: "rounded-sm",
            },
        },
        defaultVariants: {
            square: false,
        },
    }
);

export interface AvatarProps
    extends React.ComponentProps<typeof RadixAvatar.Root>,
        VariantProps<typeof avatarVariants> {
    src: string;
    fallback?: React.ReactNode;
}

function Avatar(props: AvatarProps) {
    const { src, fallback, square, className, ref, ...rest } = props;

    return (
        <RadixAvatar.Root
            ref={ref}
            className={cn(avatarVariants({ square, className }))}
            {...rest}
        >
            <AvatarImage src={src} />
            <AvatarFallback>{fallback}</AvatarFallback>
        </RadixAvatar.Root>
    );
}

function AvatarImage({
    className,
    ref,
    ...rest
}: React.ComponentProps<typeof RadixAvatar.Image>) {
    return (
        <RadixAvatar.Image
            ref={ref}
            className={cn(
                ["aspect-square", "h-full", "w-full", "object-cover"],
                className
            )}
            draggable={false}
            {...rest}
        />
    );
}

function AvatarFallback({
    className,
    ref,
    ...rest
}: React.ComponentProps<typeof RadixAvatar.Fallback>) {
    return (
        <RadixAvatar.Fallback
            ref={ref}
            className={cn(
                [
                    "flex",
                    "h-full",
                    "w-full",
                    "items-center",
                    "justify-center",
                    "bg-foreground/10",
                ],
                className
            )}
            {...rest}
        />
    );
}

export { Avatar };
