import { Checkbox as BaseCheckbox } from "@base-ui/react/checkbox";
import { CheckIcon } from "lucide-react";
import type * as React from "react";
import { cn } from "@/utils";

function Checkbox({
  className,
  ...props
}: React.ComponentProps<typeof BaseCheckbox.Root>) {
  return (
    <BaseCheckbox.Root
      data-slot="checkbox"
      className={cn(
        "inline-flex size-4 shrink-0 items-center justify-center rounded-sm border border-input bg-input text-primary shadow-xs outline-none transition-colors data-checked:border-primary data-checked:bg-primary data-unchecked:bg-input focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50",
        className
      )}
      {...props}
    >
      <BaseCheckbox.Indicator>
        <CheckIcon className="size-3.5 text-primary-foreground" />
      </BaseCheckbox.Indicator>
    </BaseCheckbox.Root>
  );
}

export { Checkbox };
