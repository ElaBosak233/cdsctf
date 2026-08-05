import type * as React from "react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";

type ScrollableNavProps = Omit<
  React.ComponentProps<typeof ScrollArea>,
  "horizontal" | "vertical"
> & {
  contentClassName?: string;
};

function ScrollableNav({
  children,
  className,
  contentClassName,
  ...props
}: ScrollableNavProps) {
  return (
    <ScrollArea
      horizontal
      vertical={false}
      className={cn(
        ["w-full", "h-15", "border-b", "bg-card/30", "shrink-0"],
        className
      )}
      {...props}
    >
      <div className={cn(["min-w-full", "py-3"])}>
        <nav
          className={cn(
            [
              "flex",
              "w-max",
              "min-w-full",
              "flex-row",
              "flex-nowrap",
              "items-center",
              "gap-2",
              "px-3",
            ],
            contentClassName
          )}
        >
          {children}
        </nav>
      </div>
    </ScrollArea>
  );
}

export { ScrollableNav };
