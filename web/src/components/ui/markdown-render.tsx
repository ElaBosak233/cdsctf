import { MarkdownHooks } from "react-markdown";
import rehypeExternalLinks from "rehype-external-links";
import rehypeKatex from "rehype-katex";
import rehypePrettyCode, {
  type Options as RehypePrettyCodeOptions,
} from "rehype-pretty-code";
import rehypeSlug from "rehype-slug";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
// import rehypeAutolinkHeadings, {
//     Options as RehypeAutolinkHeadingsOptions,
// } from "rehype-autolink-headings";
import "katex/dist/katex.min.css";
import type React from "react";
import { createHighlighter } from "./shiki";

interface MarkdownRenderProps
  extends React.ComponentProps<typeof MarkdownHooks> {
  src?: string;
  anchors?: boolean;
}

function MarkdownRender(props: MarkdownRenderProps) {
  const { src, ...rest } = props;

  return (
    <MarkdownHooks
      remarkPlugins={[remarkGfm, remarkParse, remarkMath, remarkRehype]}
      rehypePlugins={[
        [
          rehypePrettyCode,
          {
            grid: true,
            theme: {
              dark: "github-dark",
              light: "github-light",
            },
            getHighlighter: (_options) => createHighlighter(),
            keepBackground: false,
            bypassInlineCode: false,
          } satisfies RehypePrettyCodeOptions,
        ],
        rehypeKatex,
        rehypeSlug,
        [
          rehypeExternalLinks,
          { target: "_blank", rel: ["noopener", "noreferrer"] },
        ],
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
      {...rest}
    >
      {src}
    </MarkdownHooks>
  );
}

export { MarkdownRender };
