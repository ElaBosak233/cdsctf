import { cn } from "@/utils";

type TypographyProps = React.ComponentProps<"article"> & {};

function Typography(props: TypographyProps) {
  const { children, className, ...rest } = props;

  return (
    <article
      className={cn([
        "prose",
        "dark:prose-invert",
        "prose-pre:p-0",
        "max-w-none",
        className,
      ])}
      {...rest}
    >
      {children}
    </article>
  );
}

export { Typography };
