import { Dialog as BaseDialog } from "@base-ui/react/dialog";
import React from "react";

import { cn } from "@/utils";

type DialogProps = React.ComponentProps<typeof BaseDialog.Root> & {};

function Dialog(props: DialogProps) {
  const { ...rest } = props;

  return <BaseDialog.Root data-slot="dialog" {...rest} />;
}

function DialogTrigger({
  ...props
}: React.ComponentProps<typeof BaseDialog.Trigger> & { asChild?: boolean }) {
  const { asChild, children, render, ...rest } = props;
  const child = React.isValidElement(children) ? children : undefined;

  return (
    <BaseDialog.Trigger
      data-slot="dialog-trigger"
      // Base UI's render prop replaces the trigger element. This also keeps
      // Button/Card triggers from becoming invalid nested interactive elements.
      render={render ?? child}
      {...rest}
    >
      {render || child || asChild ? undefined : children}
    </BaseDialog.Trigger>
  );
}

type DialogContentProps = React.ComponentProps<typeof BaseDialog.Popup> & {
  size?: "default" | "wide" | "preview";
  slotProps?: {
    title?: React.ComponentProps<typeof BaseDialog.Title>;
  };
};

function DialogContent(props: DialogContentProps) {
  const { children, className, size = "default", slotProps, ...rest } = props;

  const sizeClass = {
    default: "max-w-xl",
    wide: "max-w-2xl",
    preview: "max-w-5xl",
  }[size];

  return (
    <DialogPortal data-slot="dialog-portal">
      <DialogOverlay />
      <BaseDialog.Popup
        aria-describedby={undefined}
        data-slot="dialog-content"
        className={cn([
          "data-open:animate-in",
          "data-closed:animate-out",
          "data-closed:fade-out-0",
          "data-open:fade-in-0",
          "data-closed:zoom-out-95",
          "data-open:zoom-in-95",
          "outline-hidden",
          "fixed",
          "top-1/2",
          "left-1/2",
          "z-50",
          "grid",
          "w-[calc(100%-2rem)]",
          sizeClass,
          "-translate-x-1/2",
          "-translate-y-1/2",
          "duration-200",
          className,
        ])}
        {...rest}
      >
        <BaseDialog.Title
          className={cn(["hidden", slotProps?.title?.className])}
          {...slotProps?.title}
        />
        {children}
      </BaseDialog.Popup>
    </DialogPortal>
  );
}

function DialogPortal({
  ...props
}: React.ComponentProps<typeof BaseDialog.Portal>) {
  return <BaseDialog.Portal data-slot="dialog-portal" {...props} />;
}

function DialogOverlay({
  className,
  ...props
}: React.ComponentProps<typeof BaseDialog.Backdrop>) {
  return (
    <BaseDialog.Backdrop
      data-slot="dialog-overlay"
      className={cn(
        [
          "data-open:animate-in",
          "data-closed:animate-out",
          "data-closed:fade-out-0",
          "data-open:fade-in-0",
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

export { Dialog, DialogContent, DialogTrigger };
