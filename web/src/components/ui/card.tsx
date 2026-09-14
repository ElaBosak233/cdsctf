import { useRender } from "@base-ui/react/use-render";
import React from "react";
import { cn } from "@/utils";

type CardProps = useRender.ComponentProps<"div"> & {
  asChild?: boolean;
};

function Card(props: CardProps) {
  const { className, asChild = false, render, children, ref, ...rest } = props;
  const child = React.isValidElement(children) ? children : undefined;
  return useRender({
    defaultTagName: "div",
    render: render ?? (asChild ? child : undefined),
    ref,
    props: {
      className: cn(
        "rounded-lg border bg-card text-card-foreground shadow-xs",
        className
      ),
      ...rest,
      children: child && asChild ? undefined : children,
    },
  });
}

export { Card };
