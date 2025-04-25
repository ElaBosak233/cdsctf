import Markdown from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import rehypeKatex from "rehype-katex";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeStringify from "rehype-stringify";
import rehypeHighlight from "rehype-highlight";
import "katex/dist/katex.min.css";
import "highlight.js/styles/atom-one-dark.min.css";
import { cn } from "@/utils";
import React from "react";
import { Typography } from "../ui/typography";

interface MarkdownRenderProps extends React.ComponentProps<typeof Typography> {
    src?: string;
}

function MarkdownRender(props: MarkdownRenderProps) {
    const { src, ...rest } = props;

    return (
        <Typography {...rest}>
            <Markdown
                remarkPlugins={[
                    remarkGfm,
                    remarkParse,
                    remarkMath,
                    remarkRehype,
                ]}
                rehypePlugins={[
                    rehypeKatex,
                    rehypeAutolinkHeadings,
                    rehypeStringify,
                    rehypeHighlight,
                ]}
                components={{
                    pre: ({ children }) => {
                        return (
                            <pre
                                className={cn([
                                    "rounded-lg",
                                    "overflow-hidden",
                                ])}
                            >
                                {children}
                            </pre>
                        );
                    },
                }}
            >
                {src}
            </Markdown>
        </Typography>
    );
}

export { MarkdownRender };
