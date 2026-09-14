import { Menu as BaseMenu } from "@base-ui/react/menu";
import { CheckIcon, ChevronRight, CircleIcon } from "lucide-react";
import * as React from "react";

import { cn } from "@/utils";

function DropdownMenu(props: any) {
  return <BaseMenu.Root data-slot="dropdown-menu" {...props} />;
}
function DropdownMenuPortal(props: any) {
  return <BaseMenu.Portal data-slot="dropdown-menu-portal" {...props} />;
}
function DropdownMenuTrigger({ asChild, children, ...props }: any) {
  const child = React.isValidElement(children)
    ? (children as React.ReactElement<{ children?: React.ReactNode }>)
    : undefined;
  return (
    <BaseMenu.Trigger
      data-slot="dropdown-menu-trigger"
      render={asChild ? child : undefined}
      {...props}
    >
      {asChild && child ? child.props.children : children}
    </BaseMenu.Trigger>
  );
}
function DropdownMenuGroup(props: any) {
  return <BaseMenu.Group data-slot="dropdown-menu-group" {...props} />;
}
function DropdownMenuSub(props: any) {
  return <BaseMenu.SubmenuRoot data-slot="dropdown-menu-sub" {...props} />;
}
function DropdownMenuRadioGroup(props: any) {
  return (
    <BaseMenu.RadioGroup data-slot="dropdown-menu-radio-group" {...props} />
  );
}

function DropdownMenuContent({
  className,
  side = "bottom",
  sideOffset = 4,
  align = "start",
  alignOffset = 0,
  children,
  ...props
}: any) {
  return (
    <BaseMenu.Portal>
      <BaseMenu.Positioner
        side={side}
        sideOffset={sideOffset}
        align={align}
        alignOffset={alignOffset}
        className="isolate z-50"
      >
        <BaseMenu.Popup
          data-slot="dropdown-menu-content"
          className={cn(
            "z-50 min-w-32 overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0",
            className
          )}
          {...props}
        >
          {children}
        </BaseMenu.Popup>
      </BaseMenu.Positioner>
    </BaseMenu.Portal>
  );
}
function DropdownMenuSubContent({ className, children, ...props }: any) {
  return (
    <DropdownMenuContent
      data-slot="dropdown-menu-sub-content"
      side="right"
      align="start"
      alignOffset={-3}
      className={cn("min-w-32 shadow-lg", className)}
      {...props}
    >
      {children}
    </DropdownMenuContent>
  );
}
function DropdownMenuSubTrigger({
  className,
  inset,
  children,
  asChild,
  ...props
}: any) {
  const child = React.isValidElement(children)
    ? (children as React.ReactElement<{ children?: React.ReactNode }>)
    : undefined;
  return (
    <BaseMenu.SubmenuTrigger
      data-slot="dropdown-menu-sub-trigger"
      data-inset={inset}
      render={
        asChild && child
          ? (renderProps: React.HTMLAttributes<HTMLElement>) =>
              React.cloneElement(child, {
                ...renderProps,
                children: child.props.children,
              })
          : undefined
      }
      className={cn(
        "flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none focus:bg-foreground/10 data-open:bg-foreground/10",
        inset && "pl-8",
        className
      )}
      {...props}
    >
      {asChild && child ? (
        child.props.children
      ) : (
        <>
          {children}
          <ChevronRight className="ml-auto size-4" />
        </>
      )}
    </BaseMenu.SubmenuTrigger>
  );
}

const itemClass =
  "relative flex cursor-default select-none items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden transition-colors focus:bg-primary/5 data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0";
function DropdownMenuItem({
  className,
  inset,
  asChild,
  children,
  ...props
}: any) {
  const child = React.isValidElement(children)
    ? (children as React.ReactElement<{ children?: React.ReactNode }>)
    : undefined;
  return (
    <BaseMenu.Item
      data-slot="dropdown-menu-item"
      data-inset={inset}
      render={
        asChild && child
          ? (renderProps: React.HTMLAttributes<HTMLElement>) =>
              React.cloneElement(child, {
                ...renderProps,
                children: child.props.children,
              })
          : undefined
      }
      className={cn(itemClass, inset && "pl-8", className)}
      {...props}
    >
      {asChild && child ? child.props.children : children}
    </BaseMenu.Item>
  );
}
function DropdownMenuCheckboxItem({
  className,
  children,
  checked,
  ...props
}: any) {
  return (
    <BaseMenu.CheckboxItem
      data-slot="dropdown-menu-checkbox-item"
      checked={checked}
      className={cn(itemClass, "pl-8", className)}
      {...props}
    >
      <span className="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
        <BaseMenu.CheckboxItemIndicator>
          <CheckIcon className="size-4" />
        </BaseMenu.CheckboxItemIndicator>
      </span>
      {children}
    </BaseMenu.CheckboxItem>
  );
}
function DropdownMenuRadioItem({ className, children, ...props }: any) {
  return (
    <BaseMenu.RadioItem
      data-slot="dropdown-menu-radio-item"
      className={cn(itemClass, "pl-8", className)}
      {...props}
    >
      <span className="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
        <BaseMenu.RadioItemIndicator>
          <CircleIcon className="size-2 fill-current" />
        </BaseMenu.RadioItemIndicator>
      </span>
      {children}
    </BaseMenu.RadioItem>
  );
}
function DropdownMenuLabel({ className, inset, ...props }: any) {
  return (
    <BaseMenu.GroupLabel
      data-slot="dropdown-menu-label"
      data-inset={inset}
      className={cn(
        "px-2 py-1.5 text-sm font-medium",
        inset && "pl-8",
        className
      )}
      {...props}
    />
  );
}
function DropdownMenuSeparator({ className, ...props }: any) {
  return (
    <BaseMenu.Separator
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
