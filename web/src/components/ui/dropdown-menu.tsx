import * as React from "react";
import * as RadixDropdownMenu from "@radix-ui/react-dropdown-menu";
import { Check, ChevronRight, Circle, LucideIcon } from "lucide-react";

import { cn } from "@/utils";
import { cva, VariantProps } from "class-variance-authority";
import { Slottable } from "@radix-ui/react-slot";

const DropdownMenu = RadixDropdownMenu.Root;

function DropdownMenuTrigger(
    props: RadixDropdownMenu.DropdownMenuTriggerProps
) {
    const { ...rest } = props;
    return <RadixDropdownMenu.Trigger {...rest} />;
}

const DropdownMenuGroup = RadixDropdownMenu.Group;

const DropdownMenuPortal = RadixDropdownMenu.Portal;

const DropdownMenuSub = RadixDropdownMenu.Sub;

const DropdownMenuRadioGroup = RadixDropdownMenu.RadioGroup;

function DropdownMenuSubTrigger({
    className,
    inset,
    children,
    ref,
    ...rest
}: React.ComponentProps<typeof RadixDropdownMenu.SubTrigger> & {
    inset?: boolean;
}) {
    return (
        <RadixDropdownMenu.SubTrigger
            ref={ref}
            className={cn(
                [
                    "flex",
                    "cursor-default",
                    "gap-2",
                    "select-none",
                    "items-center",
                    "rounded-sm",
                    "px-2",
                    "py-1.5",
                    "text-sm",
                    "outline-hidden",
                    "focus:bg-foreground/10",
                    "data-[state=open]:bg-foreground/10",
                    "[&_svg]:pointer-events-none",
                    "[&_svg]:size-4",
                    "[&_svg]:shrink-0",
                ],
                inset && "pl-8",
                className
            )}
            {...rest}
        >
            {children}
            <ChevronRight className="ml-auto" />
        </RadixDropdownMenu.SubTrigger>
    );
}
DropdownMenuSubTrigger.displayName = RadixDropdownMenu.SubTrigger.displayName;

function DropdownMenuSubContent(
    props: React.ComponentProps<typeof RadixDropdownMenu.SubContent>
) {
    const { className, ref, ...rest } = props;
    return (
        <RadixDropdownMenu.SubContent
            ref={ref}
            className={cn(
                [
                    "z-50",
                    "min-w-[8rem]",
                    "overflow-hidden",
                    "rounded-md",
                    "border",
                    "bg-popover",
                    "p-1",
                    "text-popover-foreground",
                    "shadow-lg",
                    "data-[state=open]:animate-in",
                    "data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0",
                    "data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95",
                    "data-[state=open]:zoom-in-95",
                    "data-[side=bottom]:slide-in-from-top-2",
                    "data-[side=left]:slide-in-from-right-2",
                    "data-[side=right]:slide-in-from-left-2",
                    "data-[side=top]:slide-in-from-bottom-2",
                ],
                className
            )}
            {...rest}
        />
    );
}

DropdownMenuSubContent.displayName = RadixDropdownMenu.SubContent.displayName;

const DropdownMenuContent = React.forwardRef<
    React.ElementRef<typeof RadixDropdownMenu.Content>,
    React.ComponentPropsWithoutRef<typeof RadixDropdownMenu.Content>
>(({ className, sideOffset = 4, ...props }, ref) => (
    <RadixDropdownMenu.Portal>
        <RadixDropdownMenu.Content
            ref={ref}
            sideOffset={sideOffset}
            className={cn(
                [
                    "z-50",
                    "min-w-[8rem]",
                    "overflow-hidden",
                    "rounded-md",
                    "border",
                    "bg-popover",
                    "p-1",
                    "text-popover-foreground",
                    "shadow-md",
                    "data-[state=open]:animate-in",
                    "data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0",
                    "data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95",
                    "data-[state=open]:zoom-in-95",
                    "data-[side=bottom]:slide-in-from-top-2",
                    "data-[side=left]:slide-in-from-right-2",
                    "data-[side=right]:slide-in-from-left-2",
                    "data-[side=top]:slide-in-from-bottom-2",
                ],
                className
            )}
            {...props}
        />
    </RadixDropdownMenu.Portal>
));
DropdownMenuContent.displayName = RadixDropdownMenu.Content.displayName;

const dropdownMenuItemVariants = cva(
    [
        "relative",
        "flex",
        "cursor-default",
        "select-none",
        "items-center",
        "gap-2",
        "rounded-sm",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-hidden",
        "transition-colors",
        "focus:bg-primary/5",
        "data-disabled:pointer-events-none",
        "data-disabled:opacity-50",
        "[&_svg]:pointer-events-none",
        "[&_svg]:size-4",
        "[&_svg]:shrink-0",
    ],
    {
        variants: {
            inset: {
                true: ["pl-8"],
            },
        },
        defaultVariants: {
            inset: false,
        },
    }
);

interface DropdownMenuItemProps
    extends React.ComponentProps<typeof RadixDropdownMenu.Item>,
        VariantProps<typeof dropdownMenuItemVariants> {
    icon?: LucideIcon;
}

function DropdownMenuItem(props: DropdownMenuItemProps) {
    const { icon, inset, className, ref, children, ...rest } = props;

    const Icon = icon!;

    return (
        <RadixDropdownMenu.Item
            ref={ref}
            className={cn(dropdownMenuItemVariants({ inset, className }))}
            {...rest}
        >
            {!!icon && <Icon />}
            <Slottable>{children}</Slottable>
        </RadixDropdownMenu.Item>
    );
}

const DropdownMenuCheckboxItem = React.forwardRef<
    React.ElementRef<typeof RadixDropdownMenu.CheckboxItem>,
    React.ComponentPropsWithoutRef<typeof RadixDropdownMenu.CheckboxItem>
>(({ className, children, checked, ...props }, ref) => (
    <RadixDropdownMenu.CheckboxItem
        ref={ref}
        className={cn(
            [
                "relative",
                "flex",
                "cursor-default",
                "select-none",
                "items-center",
                "rounded-sm",
                "py-1.5",
                "pl-8",
                "pr-2",
                "text-sm",
                "outline-hidden",
                "transition-colors",
                "focus:bg-foreground/10",
                "focus:text-foreground",
                "data-disabled:pointer-events-none",
                "data-disabled:opacity-50",
            ],
            className
        )}
        checked={checked}
        {...props}
    >
        <span
            className={cn([
                "absolute",
                "left-2",
                "flex",
                "h-3.5",
                "w-3.5",
                "items-center",
                "justify-center",
            ])}
        >
            <RadixDropdownMenu.ItemIndicator>
                <Check className="h-4 w-4" />
            </RadixDropdownMenu.ItemIndicator>
        </span>
        {children}
    </RadixDropdownMenu.CheckboxItem>
));
DropdownMenuCheckboxItem.displayName =
    RadixDropdownMenu.CheckboxItem.displayName;

const DropdownMenuRadioItem = React.forwardRef<
    React.ElementRef<typeof RadixDropdownMenu.RadioItem>,
    React.ComponentPropsWithoutRef<typeof RadixDropdownMenu.RadioItem>
>(({ className, children, ...props }, ref) => (
    <RadixDropdownMenu.RadioItem
        ref={ref}
        className={cn(
            [
                "relative",
                "flex",
                "cursor-default",
                "select-none",
                "items-center",
                "rounded-sm",
                "py-1.5",
                "pl-8",
                "pr-2",
                "text-sm",
                "outline-hidden",
                "transition-colors",
                "focus:bg-foreground/10",
                "focus:text-foreground/10",
                "data-disabled:pointer-events-none",
                "data-disabled:opacity-50",
            ],
            className
        )}
        {...props}
    >
        <span
            className={cn([
                "absolute",
                "left-2",
                "flex",
                "h-3.5",
                "w-3.5",
                "items-center",
                "justify-center",
            ])}
        >
            <RadixDropdownMenu.ItemIndicator>
                <Circle className={cn(["h-2", "w-2", "fill-current"])} />
            </RadixDropdownMenu.ItemIndicator>
        </span>
        {children}
    </RadixDropdownMenu.RadioItem>
));
DropdownMenuRadioItem.displayName = RadixDropdownMenu.RadioItem.displayName;

const DropdownMenuLabel = React.forwardRef<
    React.ElementRef<typeof RadixDropdownMenu.Label>,
    React.ComponentPropsWithoutRef<typeof RadixDropdownMenu.Label> & {
        inset?: boolean;
    }
>(({ className, inset, ...props }, ref) => (
    <RadixDropdownMenu.Label
        ref={ref}
        className={cn(
            "px-2 py-1.5 text-sm font-semibold",
            inset && "pl-8",
            className
        )}
        {...props}
    />
));
DropdownMenuLabel.displayName = RadixDropdownMenu.Label.displayName;

const DropdownMenuSeparator = React.forwardRef<
    React.ElementRef<typeof RadixDropdownMenu.Separator>,
    React.ComponentPropsWithoutRef<typeof RadixDropdownMenu.Separator>
>(({ className, ...props }, ref) => (
    <RadixDropdownMenu.Separator
        ref={ref}
        className={cn(["-mx-1", "my-1", "h-px", "bg-border"], className)}
        {...props}
    />
));
DropdownMenuSeparator.displayName = RadixDropdownMenu.Separator.displayName;

const DropdownMenuShortcut = ({
    className,
    ...props
}: React.HTMLAttributes<HTMLSpanElement>) => {
    return (
        <span
            className={cn(
                ["ml-auto", "text-xs", "tracking-widest", "opacity-60"],
                className
            )}
            {...props}
        />
    );
};
DropdownMenuShortcut.displayName = "DropdownMenuShortcut";

export {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuCheckboxItem,
    DropdownMenuRadioItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuShortcut,
    DropdownMenuGroup,
    DropdownMenuPortal,
    DropdownMenuSub,
    DropdownMenuSubContent,
    DropdownMenuSubTrigger,
    DropdownMenuRadioGroup,
};
