import { Text, TextProps, TypographyStylesProvider } from "@mantine/core";
import katex from "katex";
import "katex/dist/katex.min.css";
import { Marked } from "marked";
import { markedHighlight } from "marked-highlight";
import Prism from "prismjs";
import { forwardRef } from "react";
import KatexExtension from "@/utils/katex";
import classes from "./MarkdownRender.module.css";

export interface MarkdownProps extends React.ComponentPropsWithoutRef<"div"> {
    src: string;
    withRightIcon?: boolean;
}

interface InlineMarkdownProps extends TextProps {
    source: string;
}

export const InlineMarkdownRender = forwardRef<
    HTMLParagraphElement,
    InlineMarkdownProps
>((props, ref) => {
    const { source, ...others } = props;
    const marked = new Marked();
    marked.use(KatexExtension({}));

    const renderer = new marked.Renderer();

    marked.setOptions({
        renderer,
        silent: true,
    });

    return (
        <Text
            ref={ref}
            {...others}
            className={classes.inline}
            dangerouslySetInnerHTML={{
                __html: marked.parseInline(source) ?? "",
            }}
        />
    );
});

export const MarkdownRender = forwardRef<HTMLDivElement, MarkdownProps>(
    (props, ref) => {
        const { src, ...others } = props;

        Prism.manual = true;

        const marked = new Marked(
            markedHighlight({
                highlight(code, lang) {
                    if (lang && Prism.languages[lang]) {
                        return Prism.highlight(
                            code,
                            Prism.languages[lang],
                            lang
                        );
                    } else {
                        return code;
                    }
                },
            })
        );

        marked.use(KatexExtension({}));
        const renderer = new marked.Renderer();

        marked.setOptions({
            renderer,
            silent: true,
        });

        return (
            <TypographyStylesProvider
                ref={ref}
                {...others}
                data-with-right-icon={props.withRightIcon || undefined}
                className={classes.root}
            >
                <div dangerouslySetInnerHTML={{ __html: marked.parse(src) }} />
            </TypographyStylesProvider>
        );
    }
);

export default MarkdownRender;
