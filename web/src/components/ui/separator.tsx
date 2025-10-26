import * as SeparatorPrimitive from "@radix-ui/react-separator";
import { cva } from "class-variance-authority";
import type * as React from "react";

import { cn } from "@/utils/index";

const separatorVariants = cva(["shrink-0", "bg-border"], {
  variants: {
    orientation: {
      vertical: "h-auto w-[1px]",
      horizontal: "h-[1px] w-auto",
    },
  },
});

function Separator(
  props: React.ComponentProps<typeof SeparatorPrimitive.Root>
) {
  const {
    className,
    orientation = "horizontal",
    decorative = true,
    ref,
    ...rest
  } = props;

  return (
    <SeparatorPrimitive.Root
      ref={ref}
      decorative={decorative}
      orientation={orientation}
      className={cn(separatorVariants({ orientation, className }))}
      {...rest}
    />
  );
}
Separator.displayName = SeparatorPrimitive.Root.displayName;

export { Separator };
