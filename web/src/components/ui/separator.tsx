import { Separator as SeparatorPrimitive } from "@base-ui/react/separator";
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

function Separator(props: React.ComponentProps<typeof SeparatorPrimitive>) {
  const { className, orientation = "horizontal", ref, ...rest } = props;

  return (
    <SeparatorPrimitive
      ref={ref}
      orientation={orientation}
      className={cn(separatorVariants({ orientation, className }))}
      {...rest}
    />
  );
}
Separator.displayName = SeparatorPrimitive.displayName;

export { Separator };
