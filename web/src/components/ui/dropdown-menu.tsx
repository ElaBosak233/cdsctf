import * as RadixDropdownMenu from "@radix-ui/react-dropdown-menu";
import { cva, type VariantProps } from "class-variance-authority";
import { CheckIcon, ChevronRight, CircleIcon } from "lucide-react";
import type * as React from "react";

import { cn } from "@/utils";

function DropdownMenu({
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Root>) {
  return <RadixDropdownMenu.Root data-slot="dropdown-menu" {...props} />;
}

function DropdownMenuTrigger({
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Trigger>) {
  return (
    <RadixDropdownMenu.Trigger data-slot="dropdown-menu-trigger" {...props} />
  );
}

const DropdownMenuGroup = RadixDropdownMenu.Group;

function DropdownMenuPortal({
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Portal>) {
  return (
    <RadixDropdownMenu.Portal data-slot="dropdown-menu-portal" {...props} />
  );
}

function DropdownMenuSub({
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Sub>) {
  return <RadixDropdownMenu.Sub data-slot="dropdown-menu-sub" {...props} />;
}

function DropdownMenuRadioGroup({
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.RadioGroup>) {
  return (
    <RadixDropdownMenu.RadioGroup
      data-slot="dropdown-menu-radio-group"
      {...props}
    />
  );
}

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
          "min-w-32",
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

function DropdownMenuContent({
  className,
  sideOffset = 4,
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Content>) {
  return (
    <RadixDropdownMenu.Portal>
      <RadixDropdownMenu.Content
        data-slot="dropdown-menu-content"
        sideOffset={sideOffset}
        className={cn(
          [
            "z-50",
            "min-w-32",
            "overflow-hidden",
            "rounded-md",
            "border",
            "bg-popover",
            "p-1",
            "text-popover-foreground",
            "shadow-md",
            "mx-2",
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
  );
}

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
    VariantProps<typeof dropdownMenuItemVariants> {}

function DropdownMenuItem(props: DropdownMenuItemProps) {
  const { inset, className, ref, ...rest } = props;

  return (
    <RadixDropdownMenu.Item
      ref={ref}
      data-slot="dropdown-menu-item"
      data-inset={inset}
      className={cn(dropdownMenuItemVariants({ inset, className }))}
      {...rest}
    />
  );
}

function DropdownMenuCheckboxItem({
  className,
  children,
  checked,
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.CheckboxItem>) {
  return (
    <RadixDropdownMenu.CheckboxItem
      data-slot="dropdown-menu-checkbox-item"
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
      <span className="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
        <RadixDropdownMenu.ItemIndicator>
          <CheckIcon className="size-4" />
        </RadixDropdownMenu.ItemIndicator>
      </span>
      {children}
    </RadixDropdownMenu.CheckboxItem>
  );
}

function DropdownMenuRadioItem({
  className,
  children,
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.RadioItem>) {
  return (
    <RadixDropdownMenu.RadioItem
      data-slot="dropdown-menu-radio-item"
      className={cn(
        "focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm py-1.5 pr-2 pl-8 text-sm outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        className
      )}
      {...props}
    >
      <span className="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
        <RadixDropdownMenu.ItemIndicator>
          <CircleIcon className="size-2 fill-current" />
        </RadixDropdownMenu.ItemIndicator>
      </span>
      {children}
    </RadixDropdownMenu.RadioItem>
  );
}

function DropdownMenuLabel({
  className,
  inset,
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Label> & {
  inset?: boolean;
}) {
  return (
    <RadixDropdownMenu.Label
      data-slot="dropdown-menu-label"
      data-inset={inset}
      className={cn(
        "px-2 py-1.5 text-sm font-medium data-inset:pl-8",
        className
      )}
      {...props}
    />
  );
}

function DropdownMenuSeparator({
  className,
  ...props
}: React.ComponentProps<typeof RadixDropdownMenu.Separator>) {
  return (
    <RadixDropdownMenu.Separator
      data-slot="dropdown-menu-separator"
      className={cn("bg-border -mx-1 my-1 h-px", className)}
      {...props}
    />
  );
}

function DropdownMenuShortcut({
  className,
  ...props
}: React.ComponentProps<"span">) {
  return (
    <span
      data-slot="dropdown-menu-shortcut"
      className={cn(
        "text-muted-foreground ml-auto text-xs tracking-widest",
        className
      )}
      {...props}
    />
  );
}

export {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
};
