import * as React from "react";
import { cn } from "@/utils";
import { cva } from "class-variance-authority";
import { Button } from "./button";
import { TagsField } from "./tags-field";

const FieldContext = React.createContext<{
    size?: "sm" | "md";
    disabled?: boolean;
    hasIcon?: boolean;
    hasExtraButton?: boolean;
    autoHeight?: boolean;
}>({});

interface FieldRootProps extends React.ComponentProps<"div"> {
    size?: "sm" | "md";
    disabled?: boolean;
}
function FieldRoot(props: FieldRootProps) {
    const { className, size, disabled, children, ref, ...rest } = props;

    let hasIcon = false;
    let hasExtraButton = false;
    let autoHeight = false;

    React.Children.forEach(children, (child) => {
        if (!React.isValidElement(child)) return;

        const type = child.type;

        if (type === FieldIcon) hasIcon = true;
        if (type === FieldButton) hasExtraButton = true;
        if (type === TagsField) autoHeight = true;
    });

    return (
        <FieldContext.Provider
            value={{ size, disabled, hasIcon, hasExtraButton, autoHeight }}
        >
            <div className={cn(["flex", "items-center"], className)} {...rest}>
                {children}
            </div>
        </FieldContext.Provider>
    );
}

interface FieldIconProps {
    className?: string;
    children?: React.ReactNode;
}

function FieldIcon(props: FieldIconProps) {
    const { className, children, ...rest } = props;
    const { size, autoHeight } = React.useContext(FieldContext);

    return (
        <div
            className={cn(iconVariants({ size, autoHeight, className }))}
            {...rest}
        >
            {children}
        </div>
    );
}

const iconVariants = cva(
    [
        "select-none",
        "rounded-l-md",
        "flex",
        "items-center",
        "justify-center",
        "gap-2",
        "bg-primary/20",
        "text-foreground",
        "[&_svg]:pointer-events-none",
        "[&_svg]:shrink-0",
        "[&_svg]:size-4",
    ],
    {
        variants: {
            size: {
                sm: ["h-10", "min-w-10"],
                md: ["h-12", "min-w-12"],
            },
            autoHeight: {
                true: "h-full",
            },
        },
        defaultVariants: {
            size: "md",
            autoHeight: false,
        },
    }
);

interface FieldButtonProps extends React.ComponentProps<typeof Button> {}

function FieldButton(props: FieldButtonProps) {
    const { className, children, ref, ...rest } = props;
    const { size } = React.useContext(FieldContext);

    return (
        <Button
            ref={ref}
            size={size}
            type={"button"}
            className={cn(extraBtnSection({ size }), className)}
            {...rest}
        >
            {children}
        </Button>
    );
}

const extraBtnSection = cva(
    [
        "rounded-l-none",
        "flex",
        "shrink-0",
        "flex-row",
        "items-center",
        "justify-center",
        "aspect-square",
        "bg-primary/20",
        "hover:bg-primary/30",
    ],
    {
        variants: {
            size: {
                sm: "h-10",
                md: "h-12",
            },
        },
        defaultVariants: {
            size: "md",
        },
    }
);

export { FieldRoot as Field, FieldIcon, FieldButton, FieldContext };
