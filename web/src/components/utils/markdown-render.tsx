import { MarkdownHooks } from "react-markdown";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import rehypeKatex from "rehype-katex";
import rehypeSlug from "rehype-slug";
// import rehypeAutolinkHeadings, {
//     Options as RehypeAutolinkHeadingsOptions,
// } from "rehype-autolink-headings";
import rehypePrettyCode, {
    Options as RehypePrettyCodeOptions,
} from "rehype-pretty-code";
import rehypeStringify from "rehype-stringify";
import "katex/dist/katex.min.css";
import React from "react";
import { Typography } from "@/components/ui/typography";

interface MarkdownRenderProps extends React.ComponentProps<typeof Typography> {
    src?: string;
    anchors?: boolean;
}

function MarkdownRender(props: MarkdownRenderProps) {
    const { src, anchors = true, ...rest } = props;

    return (
        <Typography {...rest}>
            <MarkdownHooks
                remarkPlugins={[
                    remarkGfm,
                    remarkParse,
                    remarkMath,
                    remarkRehype,
                ]}
                rehypePlugins={[
                    [
                        rehypePrettyCode,
                        {
                            grid: true,
                            theme: "github-dark",
                            keepBackground: false,
                            bypassInlineCode: false,
                        } satisfies RehypePrettyCodeOptions,
                    ],
                    rehypeKatex,
                    rehypeSlug,
                    // [
                    //     rehypeAutolinkHeadings,
                    //     {
                    //         behavior: "append",
                    //         properties: {
                    //             className: ["anchor"],
                    //         },
                    //         content: () => ({
                    //             type: "text",
                    //             value: "¶",
                    //         }),
                    //     } satisfies RehypeAutolinkHeadingsOptions,
                    // ],
                    rehypeStringify,
                ]}
            >
                {src}
            </MarkdownHooks>
        </Typography>
    );
}

export { MarkdownRender };
