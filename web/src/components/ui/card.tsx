import { Slot, Slottable } from "@radix-ui/react-slot";
import type * as React from "react";

import { cn } from "@/utils";

type CardProps = React.ComponentProps<"div"> & {
  asChild?: boolean;
};

function Card(props: CardProps) {
  const { className, asChild = false, ref, children, ...rest } = props;

  const Comp = asChild ? Slot : "div";

  return (
    <Comp
      ref={ref}
      className={cn(
        [
          "rounded-lg",
          "border",
          "bg-card",
          "text-card-foreground",
          "shadow-xs",
        ],
        className
      )}
      {...rest}
    >
      <Slottable>{children}</Slottable>
    </Comp>
  );
}

export { Card };
