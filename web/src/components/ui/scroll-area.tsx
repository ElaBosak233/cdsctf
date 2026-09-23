import { ScrollArea as BaseScrollArea } from "@base-ui/react/scroll-area";
import type * as React from "react";

import { cn } from "@/utils";

type ScrollAreaProps = React.ComponentProps<typeof BaseScrollArea.Root> & {
  vertical?: boolean;
  horizontal?: boolean;
};

function ScrollArea(props: ScrollAreaProps) {
  const {
    className,
    children,
    vertical = true,
    horizontal = true,
    ...rest
  } = props;

  return (
    <BaseScrollArea.Root
      data-slot="scroll-area"
      className={cn("relative", className)}
      {...rest}
    >
      <BaseScrollArea.Viewport
        data-slot="scroll-area-viewport"
        className={cn([
          "ring-ring/10",
          "dark:ring-ring/20",
          "dark:outline-ring/40",
          "outline-ring/50",
          "size-full",
          "rounded-[inherit]",
          "transition-[color,box-shadow]",
          "focus-visible:ring-4",
          "focus-visible:outline-1",
        ])}
      >
        {children}
      </BaseScrollArea.Viewport>
      {vertical && <ScrollBar orientation={"vertical"} />}
      {horizontal && <ScrollBar orientation={"horizontal"} />}
      <BaseScrollArea.Corner />
    </BaseScrollArea.Root>
  );
}

function ScrollBar({
  className,
  orientation = "vertical",
  ...props
}: React.ComponentProps<typeof BaseScrollArea.Scrollbar>) {
  return (
    <BaseScrollArea.Scrollbar
      data-slot="scroll-area-scrollbar"
      orientation={orientation}
      className={cn(
        "flex touch-none p-px transition-colors select-none z-10",
        orientation === "vertical" &&
          "h-full w-2 border-l border-l-transparent",
        orientation === "horizontal" &&
          "h-2 flex-col border-t border-t-transparent",
        className
      )}
      {...props}
    >
      <BaseScrollArea.Thumb
        data-slot="scroll-area-thumb"
        className="bg-foreground/30 relative flex-1 rounded-full hover:bg-foreground/40 active:bg-foreground/50 transition-colors"
      />
    </BaseScrollArea.Scrollbar>
  );
}

export { ScrollArea, ScrollBar };
