import { Tooltip as BaseTooltip } from "@base-ui/react/tooltip";
import type * as ReactTypes from "react";
import React from "react";

import { cn } from "@/utils";

type TooltipProps = ReactTypes.ComponentProps<typeof BaseTooltip.Root> & {
  delayDuration?: number;
  children?: ReactTypes.ReactNode;
};

function Tooltip({ delayDuration = 0, children, ...props }: TooltipProps) {
  return (
    <BaseTooltip.Provider delay={delayDuration}>
      <BaseTooltip.Root data-slot="tooltip" {...props}>
        {children}
      </BaseTooltip.Root>
    </BaseTooltip.Provider>
  );
}

function TooltipTrigger({
  asChild,
  children,
  ...props
}: React.ComponentProps<typeof BaseTooltip.Trigger> & { asChild?: boolean }) {
  return (
    <BaseTooltip.Trigger
      data-slot="tooltip-trigger"
      render={asChild && React.isValidElement(children) ? children : undefined}
      {...props}
    >
      {asChild ? undefined : children}
    </BaseTooltip.Trigger>
  );
}

function TooltipContent({
  className,
  sideOffset = 0,
  side,
  children,
  ...props
}: React.ComponentProps<typeof BaseTooltip.Popup> & {
  sideOffset?: number;
  side?: React.ComponentProps<typeof BaseTooltip.Positioner>["side"];
}) {
  return (
    <BaseTooltip.Portal>
      <BaseTooltip.Positioner
        side={side}
        sideOffset={sideOffset}
        className="isolate z-50"
      >
        <BaseTooltip.Popup
          data-slot="tooltip-content"
          className={cn(
            "bg-primary text-primary-foreground animate-in fade-in-0 zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95 z-50 w-fit rounded-md px-3 py-1.5 text-xs text-balance",
            className
          )}
          {...props}
        >
          {children}
          <BaseTooltip.Arrow className="bg-primary fill-primary z-50 size-2.5 translate-y-[calc(-50%-2px)] rotate-45 rounded-[2px]" />
        </BaseTooltip.Popup>
      </BaseTooltip.Positioner>
    </BaseTooltip.Portal>
  );
}

export { Tooltip, TooltipContent, TooltipTrigger };
