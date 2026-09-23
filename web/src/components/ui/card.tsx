import { useRender } from "@base-ui/react/use-render";
import { cn } from "@/utils";

type CardProps = useRender.ComponentProps<"div">;

function Card(props: CardProps) {
  const { className, render, ref, ...rest } = props;
  return useRender({
    defaultTagName: "div",
    render,
    ref,
    props: {
      className: cn(
        "rounded-lg border bg-card text-card-foreground shadow-xs",
        className
      ),
      ...rest,
    },
  });
}

export { Card, type CardProps };
