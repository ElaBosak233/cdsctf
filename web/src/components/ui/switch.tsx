import * as RadixSwitch from "@radix-ui/react-switch";
import type * as React from "react";

import { cn } from "@/utils";

function Switch({
  className,
  ...props
}: React.ComponentProps<typeof RadixSwitch.Root>) {
  return (
    <RadixSwitch.Root
      data-slot="switch"
      className={cn(
        [
          "peer",
          "data-[state=checked]:bg-primary",
          "data-[state=unchecked]:bg-input",
          "focus-visible:border-ring",
          "focus-visible:ring-ring/50",
          "inline-flex",
          "h-5",
          "w-9",
          "shrink-0",
          "items-center",
          "rounded-full",
          "border",
          "shadow-xs",
          "transition-all",
          "outline-none",
          "focus-visible:ring-[3px]",
          "disabled:cursor-not-allowed",
          "disabled:opacity-50",
          "cursor-pointer",
        ],
        className
      )}
      {...props}
    >
      <RadixSwitch.Thumb
        data-slot="switch-thumb"
        className={cn([
          "bg-background",
          "pointer-events-none",
          "block",
          "size-4",
          "rounded-full",
          "ring-0",
          "shadow-lg",
          "transition-transform",
          "data-[state=checked]:translate-x-4",
          "data-[state=unchecked]:translate-x-0",
          "data-[state=checked]:bg-input",
          "data-[state=unchecked]:bg-primary",
        ])}
      />
    </RadixSwitch.Root>
  );
}

export { Switch };
