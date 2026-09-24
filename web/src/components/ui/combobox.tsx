import { Combobox as BaseCombobox } from "@base-ui/react/combobox";
import { CheckIcon, ChevronDownIcon, XIcon } from "lucide-react";
import * as React from "react";

import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";
import { FieldContext, fieldClearButtonVariants } from "./field";
import { inputVariants } from "./text-field";

type ComboboxOption<Value = string> = {
  value: Value;
  content?: React.ReactNode;
};

type ComboboxProps<
  Value = string,
  Multiple extends boolean | undefined = false,
> = React.ComponentProps<typeof BaseCombobox.Root<Value, Multiple>> & {
  options?: ComboboxOption<Value>[];
  placeholder?: string;
  emptyText?: React.ReactNode;
  className?: string;
};

function getNodeText(node: React.ReactNode): string {
  if (node == null || typeof node === "boolean") return "";
  if (typeof node === "string" || typeof node === "number") return String(node);
  if (Array.isArray(node)) return node.map(getNodeText).join("");
  if (React.isValidElement<{ children?: React.ReactNode }>(node)) {
    return getNodeText(node.props.children);
  }
  return "";
}

function Combobox<
  Value = string,
  Multiple extends boolean | undefined = false,
>({
  options,
  placeholder,
  emptyText = "No items found.",
  className,
  children,
  ...props
}: ComboboxProps<Value, Multiple>) {
  const optionValues = options?.map((option) => option.value);
  const itemToStringLabel = (item: Value) => {
    const option = options?.find((candidate) =>
      Object.is(candidate.value, item)
    );
    return option
      ? getNodeText(option.content) || String(option.value)
      : String(item ?? "");
  };

  return (
    <BaseCombobox.Root
      data-slot="combobox"
      {...props}
      items={props.items ?? optionValues}
      itemToStringLabel={props.itemToStringLabel ?? itemToStringLabel}
    >
      {children ?? (
        <>
          <ComboboxInput className={className} placeholder={placeholder} />
          <ComboboxContent>
            <ComboboxEmpty>{emptyText}</ComboboxEmpty>
            <ComboboxList>
              {options?.map((option, index) => (
                <ComboboxItem
                  key={`${String(option.value)}-${index}`}
                  value={option.value}
                >
                  {option.content ?? String(option.value)}
                </ComboboxItem>
              ))}
            </ComboboxList>
          </ComboboxContent>
        </>
      )}
    </BaseCombobox.Root>
  );
}

function ComboboxInput({
  className,
  children,
  disabled = false,
  showTrigger = true,
  showClear = false,
  startContent,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Input> & {
  showTrigger?: boolean;
  showClear?: boolean;
  startContent?: React.ReactNode;
}) {
  const context = React.useContext(FieldContext);
  const { size, disabled: fieldDisabled, hasIcon } = context;
  const isDisabled = disabled || fieldDisabled;

  return (
    <BaseCombobox.InputGroup
      data-slot="combobox-input-group"
      className={cn(
        inputVariants({ size, icon: !!hasIcon, extraBtn: false }),
        "group/combobox-input p-0",
        "min-w-0",
        className
      )}
    >
      {startContent && (
        <span className="ml-2 flex shrink-0 items-center">{startContent}</span>
      )}
      <BaseCombobox.Input
        data-slot="combobox-input"
        disabled={isDisabled}
        className="h-full min-w-0 flex-1 bg-transparent px-3 py-2 outline-none placeholder:text-secondary-foreground/80"
        {...props}
      />
      {showClear && <ComboboxClear disabled={isDisabled} size={size} />}
      {showTrigger && (
        <ComboboxTrigger
          disabled={isDisabled}
          className="h-full rounded-l-none data-pressed:bg-transparent"
        />
      )}
      {children}
    </BaseCombobox.InputGroup>
  );
}

function ComboboxInputGroup({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.InputGroup>) {
  return (
    <BaseCombobox.InputGroup
      data-slot="combobox-input-group"
      className={cn("flex w-full items-center", className)}
      {...props}
    />
  );
}

function ComboboxTrigger({
  className,
  children,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Trigger>) {
  return (
    <BaseCombobox.Trigger
      data-slot="combobox-trigger"
      className={cn(
        "inline-flex size-10 shrink-0 items-center justify-center rounded-md text-muted-foreground hover:bg-foreground/5 disabled:pointer-events-none disabled:opacity-50",
        className
      )}
      {...props}
    >
      {children ?? <ChevronDownIcon className="size-4" />}
    </BaseCombobox.Trigger>
  );
}

function ComboboxClear({
  className,
  children,
  size,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Clear> & {
  size?: "sm" | "md";
}) {
  return (
    <BaseCombobox.Clear
      data-slot="combobox-clear"
      className={cn(fieldClearButtonVariants({ size }), className)}
      {...props}
    >
      {children ?? <XIcon className="size-4" />}
    </BaseCombobox.Clear>
  );
}

function ComboboxContent({
  className,
  children,
  side = "bottom",
  sideOffset = 6,
  align = "center",
  alignOffset = 0,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Popup> &
  Pick<
    React.ComponentProps<typeof BaseCombobox.Positioner>,
    "side" | "sideOffset" | "align" | "alignOffset"
  >) {
  return (
    <BaseCombobox.Portal>
      <BaseCombobox.Positioner
        side={side}
        sideOffset={sideOffset}
        align={align}
        alignOffset={alignOffset}
        className="isolate z-50 outline-hidden"
      >
        <BaseCombobox.Popup
          data-slot="combobox-content"
          className={cn(
            "bg-input text-input-foreground relative z-50 box-border h-auto w-(--anchor-width) max-h-(--available-height) max-w-(--anchor-width) overflow-hidden rounded-md border border-border shadow-md origin-(--transform-origin) transition-[scale,opacity] duration-100 ease-out data-starting-style:scale-[0.98] data-starting-style:opacity-0 data-ending-style:scale-[0.98] data-ending-style:opacity-0 data-[side=none]:translate-y-px data-[side=none]:w-[calc(var(--anchor-width)+1.75rem)] data-[side=none]:max-w-none data-[side=none]:data-starting-style:transition-none data-[side=none]:data-ending-style:transition-none",
            className
          )}
          {...props}
        >
          <ScrollArea
            vertical
            horizontal={false}
            className="h-auto max-h-(--available-height) w-full"
          >
            {children}
          </ScrollArea>
        </BaseCombobox.Popup>
      </BaseCombobox.Positioner>
    </BaseCombobox.Portal>
  );
}

function ComboboxList({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.List>) {
  return (
    <BaseCombobox.List
      data-slot="combobox-list"
      className={cn(
        "h-auto max-h-(--available-height) p-1 empty:hidden",
        className
      )}
      {...props}
    />
  );
}

function ComboboxItem({
  className,
  children,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Item>) {
  return (
    <BaseCombobox.Item
      data-slot="combobox-item"
      className={cn(
        "relative flex w-full cursor-default items-center gap-2 rounded-sm py-1.5 pr-8 pl-2",
        "text-sm outline-hidden select-none hover:bg-foreground/5 data-highlighted:bg-accent data-highlighted:text-accent-foreground data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        className
      )}
      {...props}
    >
      <span className="absolute right-2 flex size-3.5 items-center justify-center">
        <BaseCombobox.ItemIndicator>
          <CheckIcon className="size-4" />
        </BaseCombobox.ItemIndicator>
      </span>
      {children}
    </BaseCombobox.Item>
  );
}

function ComboboxEmpty({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Empty>) {
  return (
    <BaseCombobox.Empty
      data-slot="combobox-empty"
      className={cn(
        "px-2 py-3 text-center text-sm text-muted-foreground empty:hidden",
        className
      )}
      {...props}
    />
  );
}

function ComboboxGroup({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Group>) {
  return (
    <BaseCombobox.Group
      data-slot="combobox-group"
      className={cn("overflow-hidden p-1", className)}
      {...props}
    />
  );
}

function ComboboxLabel({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.GroupLabel>) {
  return (
    <BaseCombobox.GroupLabel
      data-slot="combobox-label"
      className={cn("px-2 py-1.5 text-sm font-medium", className)}
      {...props}
    />
  );
}

function ComboboxSeparator({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Separator>) {
  return (
    <BaseCombobox.Separator
      data-slot="combobox-separator"
      className={cn("bg-border pointer-events-none -mx-1 my-1 h-px", className)}
      {...props}
    />
  );
}

function ComboboxCollection(
  props: React.ComponentProps<typeof BaseCombobox.Collection>
) {
  return <BaseCombobox.Collection data-slot="combobox-collection" {...props} />;
}

function ComboboxValue(props: React.ComponentProps<typeof BaseCombobox.Value>) {
  return <BaseCombobox.Value data-slot="combobox-value" {...props} />;
}

function ComboboxIcon({
  className,
  ...props
}: React.ComponentProps<typeof BaseCombobox.Icon>) {
  return (
    <BaseCombobox.Icon
      data-slot="combobox-icon"
      className={cn("size-4 opacity-50", className)}
      {...props}
    />
  );
}

export {
  Combobox,
  ComboboxClear,
  ComboboxCollection,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxGroup,
  ComboboxIcon,
  ComboboxInput,
  ComboboxInputGroup,
  ComboboxItem,
  ComboboxLabel,
  ComboboxList,
  type ComboboxOption,
  ComboboxSeparator,
  ComboboxTrigger,
  ComboboxValue,
};
