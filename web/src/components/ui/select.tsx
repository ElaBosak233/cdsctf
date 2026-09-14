import { Select as BaseSelect } from "@base-ui/react/select";
import { cva } from "class-variance-authority";
import { CheckIcon, ChevronDownIcon, ChevronUpIcon } from "lucide-react";
import * as React from "react";

import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";
import { FieldContext } from "./field";

type SelectProps = {
  className?: string;
  placeholder?: string;
  options?: Array<{ value: string; content?: React.ReactNode }>;
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: any) => void;
  children?: React.ReactNode;
  [key: string]: any;
};

function Select({
  placeholder,
  options,
  className,
  children,
  ...props
}: SelectProps) {
  const itemToStringLabel = (item: unknown) => {
    const option = options?.find((candidate) => candidate.value === item);
    if (!option) return item == null ? "" : String(item);

    const toText = (node: React.ReactNode): string => {
      if (node == null || typeof node === "boolean") return "";
      if (typeof node === "string" || typeof node === "number") {
        return String(node);
      }
      if (Array.isArray(node)) return node.map(toText).join("");
      if (React.isValidElement<{ children?: React.ReactNode }>(node)) {
        return toText(node.props.children);
      }
      return "";
    };

    return toText(option.content) || option.value;
  };

  return (
    <BaseSelect.Root
      data-slot="select"
      itemToStringLabel={itemToStringLabel}
      {...(props as any)}
    >
      {children ?? (
        <>
          <SelectTrigger className={className}>
            <SelectValue
              placeholder={
                <span className="text-primary/80">{placeholder}</span>
              }
            />
          </SelectTrigger>
          <SelectContent>
            {options?.map((option) => (
              <SelectItem key={option.value} value={option.value}>
                {option.content ?? option.value}
              </SelectItem>
            ))}
          </SelectContent>
        </>
      )}
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
  "flex-1 flex w-0 rounded-md justify-between items-center border bg-input px-3 py-2 text-base ring-offset-input focus:border-input focus:outline-hidden focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm cursor-pointer *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 *:data-[slot=select-value]:truncate [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&>span]:line-clamp-1",
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
  sideOffset = 4,
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
            "bg-input text-input-foreground relative z-50 box-border w-[var(--anchor-width)] max-w-[var(--anchor-width)] max-h-[var(--available-height)] overflow-hidden rounded-md border border-border shadow-md origin-[var(--transform-origin)] transition-[scale,opacity] duration-100 ease-out data-starting-style:scale-[0.98] data-starting-style:opacity-0 data-ending-style:scale-[0.98] data-ending-style:opacity-0 data-[side=none]:translate-y-px data-[side=none]:w-[calc(var(--anchor-width)+1.75rem)] data-[side=none]:max-w-none data-[side=none]:data-starting-style:transition-none data-[side=none]:data-ending-style:transition-none",
            className
          )}
          {...props}
        >
          <ScrollArea
            vertical
            horizontal={false}
            className="h-full max-h-[var(--available-height)] w-full"
          >
            <BaseSelect.List className="h-auto max-h-[var(--available-height)] p-1">
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
