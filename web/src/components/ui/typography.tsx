import { cn } from "@/utils";

interface TypographyProps extends React.ComponentProps<"article"> {}

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
