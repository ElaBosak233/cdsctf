import { Dialog as BaseDialog } from "@base-ui/react/dialog";
import type React from "react";

import { cn } from "@/utils";

type DialogProps = React.ComponentProps<typeof BaseDialog.Root> & {};

function Dialog(props: DialogProps) {
  const { ...rest } = props;

  return <BaseDialog.Root data-slot="dialog" {...rest} />;
}

function DialogTrigger(props: React.ComponentProps<typeof BaseDialog.Trigger>) {
  return <BaseDialog.Trigger data-slot="dialog-trigger" {...props} />;
}

type DialogContentProps = React.ComponentProps<typeof BaseDialog.Popup> & {
  size?: "default" | "wide" | "preview";
  slotProps?: {
    title?: React.ComponentProps<typeof BaseDialog.Title>;
  };
};

type DialogHeaderProps = React.ComponentProps<"div"> & {
  title: React.ReactNode;
  description?: React.ReactNode;
  icon?: React.ReactNode;
  trailing?: React.ReactNode;
  level?: "primary" | "info" | "success" | "warning" | "error";
  iconClassName?: string;
  contentClassName?: string;
  titleClassName?: string;
  descriptionClassName?: string;
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

function DialogHeader(props: DialogHeaderProps) {
  const {
    title,
    description,
    icon,
    trailing,
    level = "primary",
    className,
    iconClassName,
    contentClassName,
    titleClassName,
    descriptionClassName,
    ...rest
  } = props;
  const hasDescription =
    description !== undefined && description !== null && description !== "";

  const levelClass = {
    primary: "bg-primary/10 text-primary",
    info: "bg-info/10 text-info",
    success: "bg-success/10 text-success",
    warning: "bg-warning/10 text-warning",
    error: "bg-error/10 text-error",
  }[level];

  return (
    <div
      data-slot="dialog-header"
      className={cn(
        [
          "flex",
          "min-w-0",
          "gap-3.5",
          hasDescription ? "items-start" : "items-center",
        ],
        className
      )}
      {...rest}
    >
      {icon && (
        <div
          data-slot="dialog-header-icon"
          className={cn(
            [
              "flex",
              "size-10",
              "shrink-0",
              "items-center",
              "justify-center",
              "rounded-badge",
              "shadow-xs",
              levelClass,
              "[&_svg]:size-5",
            ],
            iconClassName
          )}
        >
          {icon}
        </div>
      )}
      <div
        data-slot="dialog-header-content"
        className={cn(
          [
            "flex",
            "min-w-0",
            "flex-1",
            "flex-col",
            "gap-1",
            !hasDescription && "justify-center",
          ],
          contentClassName
        )}
      >
        <BaseDialog.Title
          data-slot="dialog-header-title"
          className={cn(
            ["text-base", "font-semibold", "text-foreground"],
            titleClassName
          )}
        >
          {title}
        </BaseDialog.Title>
        {hasDescription && (
          <BaseDialog.Description
            data-slot="dialog-header-description"
            className={cn(
              ["text-sm", "text-muted-foreground"],
              descriptionClassName
            )}
          >
            {description}
          </BaseDialog.Description>
        )}
      </div>
      {trailing && (
        <div className="ml-auto shrink-0" data-slot="dialog-header-trailing">
          {trailing}
        </div>
      )}
    </div>
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

export { Dialog, DialogContent, DialogHeader, DialogTrigger };
