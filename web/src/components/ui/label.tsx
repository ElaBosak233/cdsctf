import * as RadixLabel from "@radix-ui/react-label";
import type * as React from "react";

import { cn } from "@/utils";

type LabelProps = React.ComponentProps<typeof RadixLabel.Root>;

function Label(props: LabelProps) {
  const { className, ...rest } = props;

  return (
    <RadixLabel.Root
      data-slot="label"
      className={cn(
        [
          "text-sm",
          "leading-none",
          "font-medium",
          "select-none",
          "group-data-[disabled=true]:pointer-events-none",
          "group-data-[disabled=true]:opacity-50",
          "peer-disabled:cursor-not-allowed",
          "peer-disabled:opacity-50",
        ],
        className
      )}
      {...rest}
    />
  );
}

export { Label };
