import { Select as BaseSelect } from "@base-ui/react/select";
import { cva } from "class-variance-authority";
import { CheckIcon, ChevronDownIcon, ChevronUpIcon } from "lucide-react";
import * as React from "react";

import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";
import { FieldContext } from "./field";

type SelectProps = Omit<
  React.ComponentProps<typeof BaseSelect.Root>,
  "onValueChange"
> & {
  onValueChange?: (value: string) => void;
};

type SelectItemOption = {
  value: unknown;
  label: React.ReactNode;
};

function collectSelectItems(children: React.ReactNode): SelectItemOption[] {
  const items: SelectItemOption[] = [];

  React.Children.forEach(children, (child) => {
    if (!React.isValidElement(child)) return;

    if (child.type === SelectItem) {
      const props = child.props as React.ComponentProps<typeof BaseSelect.Item>;
      if (props.value !== undefined) {
        items.push({ value: props.value, label: props.children });
      }
      return;
    }

    items.push(
      ...collectSelectItems(
        (child.props as { children?: React.ReactNode }).children
      )
    );
  });

  return items;
}

function Select({ onValueChange, children, items, ...props }: SelectProps) {
  const derivedItems = collectSelectItems(children);

  return (
    <BaseSelect.Root
      data-slot="select"
      items={items ?? (derivedItems.length > 0 ? derivedItems : undefined)}
      {...props}
      onValueChange={(value) => onValueChange?.(String(value ?? ""))}
    >
      {children}
    </BaseSelect.Root>
  );
}

function SelectGroup(props: React.ComponentProps<typeof BaseSelect.Group>) {
  return <BaseSelect.Group data-slot="select-group" {...props} />;
}
function SelectValue(props: React.ComponentProps<typeof BaseSelect.Value>) {
  return <BaseSelect.Value data-slot="select-value" {...props} />;
}

const selectTriggerVariants = cva(
  cn(
    "flex-1 flex w-0 rounded-md justify-between items-center",
    "border bg-input px-3 py-2 text-base duration-0",
    "ring-offset-input data-[popup-open]:border-input data-[popup-open]:outline-hidden",
    "focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-0",
    "data-[popup-open]:ring-2 data-[popup-open]:ring-ring data-[popup-open]:ring-offset-2",
    "disabled:cursor-not-allowed disabled:opacity-50",
    "md:text-sm cursor-pointer *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 *:data-[slot=select-value]:truncate [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&>span]:line-clamp-1"
  ),
  {
    variants: {
      size: { sm: "h-10", md: "h-12" },
      icon: { true: "rounded-l-none border-l-0" },
    },
    defaultVariants: { size: "md", icon: false },
  }
);

function SelectTrigger({
  className,
  children,
  ...props
}: React.ComponentProps<typeof BaseSelect.Trigger>) {
  const context = React.useContext(FieldContext);
  const { size, hasIcon } = context;
  return (
    <BaseSelect.Trigger
      data-slot="select-trigger"
      className={cn(selectTriggerVariants({ icon: hasIcon, size, className }))}
      {...props}
    >
      {children}
      <BaseSelect.Icon
        render={<ChevronDownIcon className="size-4 opacity-50" />}
      />
    </BaseSelect.Trigger>
  );
}

function SelectContent({
  className,
  children,
  side = "bottom",
  sideOffset = 6,
  align = "center",
  alignOffset = 0,
  ...props
}: React.ComponentProps<typeof BaseSelect.Popup> &
  Pick<
    React.ComponentProps<typeof BaseSelect.Positioner>,
    "side" | "sideOffset" | "align" | "alignOffset"
  >) {
  return (
    <BaseSelect.Portal>
      <BaseSelect.Positioner
        side={side}
        sideOffset={sideOffset}
        align={align}
        alignOffset={alignOffset}
        alignItemWithTrigger={false}
        className="isolate z-50 outline-hidden"
      >
        <BaseSelect.Popup
          data-slot="select-content"
          className={cn(
            "bg-input text-input-foreground relative z-50 box-border w-(--anchor-width) max-w-(--anchor-width) max-h-(--available-height) overflow-hidden rounded-md border border-border shadow-md origin-(--transform-origin) transition-[scale,opacity] duration-100 ease-out data-starting-style:scale-[0.98] data-starting-style:opacity-0 data-ending-style:scale-[0.98] data-ending-style:opacity-0 data-[side=none]:translate-y-px data-[side=none]:w-[calc(var(--anchor-width)+1.75rem)] data-[side=none]:max-w-none data-[side=none]:data-starting-style:transition-none data-[side=none]:data-ending-style:transition-none",
            className
          )}
          {...props}
        >
          <ScrollArea
            vertical
            horizontal={false}
            className="h-full max-h-(--available-height) w-full"
          >
            <BaseSelect.List className="h-auto max-h-(--available-height) p-1">
              {children}
            </BaseSelect.List>
          </ScrollArea>
        </BaseSelect.Popup>
      </BaseSelect.Positioner>
    </BaseSelect.Portal>
  );
}

function SelectLabel({
  className,
  ...props
}: React.ComponentProps<typeof BaseSelect.GroupLabel>) {
  return (
    <BaseSelect.GroupLabel
      data-slot="select-label"
      className={cn("px-2 py-1.5 text-sm font-medium", className)}
      {...props}
    />
  );
}
function SelectItem({
  className,
  children,
  ...props
}: React.ComponentProps<typeof BaseSelect.Item>) {
  return (
    <BaseSelect.Item
      data-slot="select-item"
      className={cn(
        "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2 text-sm outline-hidden select-none hover:bg-foreground/5 focus:bg-accent focus:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        className
      )}
      {...props}
    >
      <span className="absolute right-2 flex size-3.5 items-center justify-center">
        <BaseSelect.ItemIndicator>
          <CheckIcon className="size-4" />
        </BaseSelect.ItemIndicator>
      </span>
      <BaseSelect.ItemText>{children}</BaseSelect.ItemText>
    </BaseSelect.Item>
  );
}
function SelectSeparator({
  className,
  ...props
}: React.ComponentProps<typeof BaseSelect.Separator>) {
  return (
    <BaseSelect.Separator
      data-slot="select-separator"
      className={cn("bg-border pointer-events-none -mx-1 my-1 h-px", className)}
      {...props}
    />
  );
}
function SelectScrollUpButton({
  className,
  ...props
}: React.ComponentProps<typeof BaseSelect.ScrollUpArrow>) {
  return (
    <BaseSelect.ScrollUpArrow
      data-slot="select-scroll-up-button"
      className={cn(
        "flex cursor-default items-center justify-center py-1",
        className
      )}
      {...props}
    >
      <ChevronUpIcon className="size-4" />
    </BaseSelect.ScrollUpArrow>
  );
}
function SelectScrollDownButton({
  className,
  ...props
}: React.ComponentProps<typeof BaseSelect.ScrollDownArrow>) {
  return (
    <BaseSelect.ScrollDownArrow
      data-slot="select-scroll-down-button"
      className={cn(
        "flex cursor-default items-center justify-center py-1",
        className
      )}
      {...props}
    >
      <ChevronDownIcon className="size-4" />
    </BaseSelect.ScrollDownArrow>
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
