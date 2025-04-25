import * as React from "react";
import * as RadixSelect from "@radix-ui/react-select";
import { CheckIcon, ChevronDownIcon, ChevronUpIcon } from "lucide-react";

import { cn } from "@/utils";
import { cva } from "class-variance-authority";
import { FieldContext } from "./field";

interface SelectProps extends React.ComponentProps<typeof RadixSelect.Root> {
    className?: string;
    placeholder?: string;
    options?: Array<{
        value: string;
        content?: React.ReactNode;
    }>;
}

function Select(props: SelectProps) {
    const { placeholder, options, className, children, ...rest } = props;

    return (
        <RadixSelect.Root data-slot="select" {...rest}>
            <SelectTrigger className={className}>
                <SelectValue
                    placeholder={
                        <span className={cn(["text-primary/80"])}>
                            {placeholder}
                        </span>
                    }
                />
            </SelectTrigger>
            <SelectContent>
                {options?.map((option, index) => (
                    <SelectItem key={index} value={option.value}>
                        {option?.content ?? option?.value}
                    </SelectItem>
                ))}
            </SelectContent>
        </RadixSelect.Root>
    );
}

function SelectGroup({
    ...props
}: React.ComponentProps<typeof RadixSelect.Group>) {
    return <RadixSelect.Group data-slot="select-group" {...props} />;
}

function SelectValue({
    ...props
}: React.ComponentProps<typeof RadixSelect.Value>) {
    return <RadixSelect.Value data-slot="select-value" {...props} />;
}

const selectTriggerVariants = cva(
    [
        "flex-1",
        "flex",
        "w-0",
        "rounded-md",
        "justify-between",
        "items-center",
        "border",
        "border-input",
        "bg-input",
        "px-3",
        "py-2",
        "text-base",
        "ring-offset-input",
        "focus:outline-hidden",
        "focus:ring-2",
        "focus:ring-ring",
        "focus:ring-offset-2",
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
        "md:text-sm",
        "cursor-pointer",
        "*:data-[slot=select-value]:flex",
        "*:data-[slot=select-value]:items-center",
        "*:data-[slot=select-value]:gap-2",
        "[&_svg]:pointer-events-none",
        "[&_svg]:shrink-0",
        "[&_svg:not([class*='size-'])]:size-4",
        "[&>span]:line-clamp-1",
    ],
    {
        variants: {
            size: {
                sm: "h-10",
                md: "h-12",
            },
            icon: {
                true: "rounded-l-none",
            },
        },
        defaultVariants: {
            size: "md",
            icon: false,
        },
    }
);

interface SelectTrigger
    extends React.ComponentProps<typeof RadixSelect.Trigger> {}

function SelectTrigger(props: SelectTrigger) {
    const context = React.useContext(FieldContext);
    const { className, children, ...rest } = props;
    const { size, hasIcon } = context;

    return (
        <RadixSelect.Trigger
            data-slot="select-trigger"
            className={cn(selectTriggerVariants({ icon: hasIcon, size }))}
            {...rest}
        >
            {children}
            <RadixSelect.Icon asChild>
                <ChevronDownIcon className={cn(["size-4", "opacity-50"])} />
            </RadixSelect.Icon>
        </RadixSelect.Trigger>
    );
}

function SelectContent({
    className,
    children,
    position = "popper",
    ...props
}: React.ComponentProps<typeof RadixSelect.Content>) {
    return (
        <RadixSelect.Portal>
            <RadixSelect.Content
                data-slot="select-content"
                className={cn(
                    [
                        "bg-popover",
                        "text-popover-foreground",
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
                        "relative",
                        "z-50",
                        "max-h-96",
                        "min-w-[8rem]",
                        "overflow-hidden",
                        "rounded-md",
                        "border",
                        "shadow-md",
                    ],
                    position === "popper" && [
                        "data-[side=bottom]:translate-y-1",
                        "data-[side=left]:-translate-x-1",
                        "data-[side=right]:translate-x-1",
                        "data-[side=top]:-translate-y-1",
                    ],
                    className
                )}
                position={position}
                {...props}
            >
                <SelectScrollUpButton />
                <RadixSelect.Viewport
                    className={cn(
                        "p-1",
                        position === "popper" && [
                            "h-[var(--radix-select-trigger-height)]",
                            "w-full",
                            "min-w-[var(--radix-select-trigger-width)]",
                            "scroll-my-1",
                        ]
                    )}
                >
                    {children}
                </RadixSelect.Viewport>
                <SelectScrollDownButton />
            </RadixSelect.Content>
        </RadixSelect.Portal>
    );
}

function SelectLabel({
    className,
    ...props
}: React.ComponentProps<typeof RadixSelect.Label>) {
    return (
        <RadixSelect.Label
            data-slot="select-label"
            className={cn(
                ["px-2", "py-1.5", "text-sm", "font-medium"],
                className
            )}
            {...props}
        />
    );
}

function SelectItem({
    className,
    children,
    ...props
}: React.ComponentProps<typeof RadixSelect.Item>) {
    return (
        <RadixSelect.Item
            data-slot="select-item"
            className={cn(
                [
                    "hover:bg-foreground/5",
                    "focus:bg-accent",
                    "focus:text-accent-foreground",
                    "[&_svg:not([class*='text-'])]:text-muted-foreground",
                    "relative",
                    "flex",
                    "cursor-default",
                    "items-center",
                    "gap-2",
                    "rounded-sm",
                    "py-1.5",
                    "pr-8",
                    "pl-2",
                    "text-sm",
                    "outline-hidden",
                    "select-none",
                    "data-[disabled]:pointer-events-none",
                    "data-[disabled]:opacity-50",
                    "[&_svg]:pointer-events-none",
                    "[&_svg]:shrink-0",
                    "[&_svg:not([class*='size-'])]:size-4",
                    "*:[span]:last:flex",
                    "*:[span]:last:items-center",
                    "*:[span]:last:gap-2",
                ],
                className
            )}
            {...props}
        >
            <span
                className={cn([
                    "absolute",
                    "right-2",
                    "flex",
                    "size-3.5",
                    "items-center",
                    "justify-center",
                ])}
            >
                <RadixSelect.ItemIndicator>
                    <CheckIcon className="size-4" />
                </RadixSelect.ItemIndicator>
            </span>
            <RadixSelect.ItemText>{children}</RadixSelect.ItemText>
        </RadixSelect.Item>
    );
}

function SelectSeparator({
    className,
    ...props
}: React.ComponentProps<typeof RadixSelect.Separator>) {
    return (
        <RadixSelect.Separator
            data-slot="select-separator"
            className={cn(
                ["bg-border", "pointer-events-none", "-mx-1", "my-1", "h-px"],
                className
            )}
            {...props}
        />
    );
}

function SelectScrollUpButton({
    className,
    ...props
}: React.ComponentProps<typeof RadixSelect.ScrollUpButton>) {
    return (
        <RadixSelect.ScrollUpButton
            data-slot="select-scroll-up-button"
            className={cn(
                [
                    "flex",
                    "cursor-default",
                    "items-center",
                    "justify-center",
                    "py-1",
                ],
                className
            )}
            {...props}
        >
            <ChevronUpIcon className="size-4" />
        </RadixSelect.ScrollUpButton>
    );
}

function SelectScrollDownButton({
    className,
    ...props
}: React.ComponentProps<typeof RadixSelect.ScrollDownButton>) {
    return (
        <RadixSelect.ScrollDownButton
            data-slot="select-scroll-down-button"
            className={cn(
                [
                    "flex",
                    "cursor-default",
                    "items-center",
                    "justify-center",
                    "py-1",
                ],
                className
            )}
            {...props}
        >
            <ChevronDownIcon className="size-4" />
        </RadixSelect.ScrollDownButton>
    );
}

export {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectScrollDownButton,
    SelectScrollUpButton,
    SelectSeparator,
    SelectTrigger,
    SelectValue,
};
