import * as React from "react";
import * as RadixDialog from "@radix-ui/react-dialog";

import { cn } from "@/utils";

interface DialogProps extends React.ComponentProps<typeof RadixDialog.Root> {}

function Dialog(props: DialogProps) {
    const { ...rest } = props;

    return <RadixDialog.Root data-slot="dialog" {...rest} />;
}

function DialogTrigger({
    ...props
}: Omit<React.ComponentProps<typeof RadixDialog.Trigger>, "asChild">) {
    return (
        <RadixDialog.Trigger data-slot="dialog-trigger" asChild {...props} />
    );
}

interface DialogContentProps
    extends Omit<React.ComponentProps<typeof RadixDialog.Content>, "asChild"> {
    slotProps?: {
        title?: React.ComponentProps<typeof RadixDialog.DialogTitle>;
    };
}

function DialogContent(props: DialogContentProps) {
    const { children, className, slotProps, ...rest } = props;

    return (
        <DialogPortal data-slot="dialog-portal">
            <DialogOverlay />
            <RadixDialog.Content
                aria-describedby={undefined}
                data-slot="dialog-content"
                className={cn([
                    "data-[state=open]:animate-in",
                    "data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0",
                    "data-[state=open]:fade-in-0",
                    "data-[state=closed]:zoom-out-95",
                    "data-[state=open]:zoom-in-95",
                    "outline-hidden",
                    "fixed",
                    "top-1/2",
                    "left-1/2",
                    "z-50",
                    "grid",
                    "-translate-x-1/2",
                    "-translate-y-1/2",
                    "duration-200",
                    className,
                ])}
                {...rest}
            >
                <RadixDialog.DialogTitle
                    className={cn(["hidden", slotProps?.title?.className])}
                    {...slotProps?.title}
                />
                {children}
            </RadixDialog.Content>
        </DialogPortal>
    );
}

function DialogPortal({
    ...props
}: React.ComponentProps<typeof RadixDialog.Portal>) {
    return <RadixDialog.Portal data-slot="dialog-portal" {...props} />;
}

function DialogOverlay({
    className,
    ...props
}: React.ComponentProps<typeof RadixDialog.Overlay>) {
    return (
        <RadixDialog.Overlay
            data-slot="dialog-overlay"
            className={cn(
                [
                    "data-[state=open]:animate-in",
                    "data-[state=closed]:animate-out",
                    "data-[state=closed]:fade-out-0",
                    "data-[state=open]:fade-in-0",
                    "fixed",
                    "inset-0",
                    "z-50",
                    "bg-black/80",
                ],
                className
            )}
            {...props}
        />
    );
}

export { Dialog, DialogTrigger, DialogContent };
