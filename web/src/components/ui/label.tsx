import type * as React from "react";

import { cn } from "@/utils";

type LabelProps = React.ComponentProps<"label">;

function Label(props: LabelProps) {
  const { className, ...rest } = props;

  return (
    // biome-ignore lint/a11y/noLabelWithoutControl: The reusable component receives its association through htmlFor.
    <label
      data-slot="label"
      className={cn(
        [
          "text-sm",
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
