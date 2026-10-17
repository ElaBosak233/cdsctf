import { MarkdownHooks } from "react-markdown";
import rehypeExternalLinks from "rehype-external-links";
import rehypeKatex from "rehype-katex";
import rehypePrettyCode, {
  type Options as RehypePrettyCodeOptions,
} from "rehype-pretty-code";
import rehypeRaw from "rehype-raw";
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
import {
  type BundledLanguage,
  type BundledTheme,
  type HighlighterGeneric,
  createHighlighter as shikiCreateHighlighter,
} from "shiki";
import {
  CodeBlock,
  CodeBlockActions,
  CodeBlockContent,
  CodeBlockCopyButton,
  CodeBlockHeader,
  CodeBlockTitle,
} from "@/components/ui/code-block";

interface MarkdownRenderProps
  extends React.ComponentProps<typeof MarkdownHooks> {
  src?: string | null;
  anchors?: boolean;
  lineNumbers?: boolean | ((language?: string) => boolean);
}

const lineNumberLanguages = new Set([
  "bash",
  "c",
  "cpp",
  "css",
  "go",
  "html",
  "java",
  "javascript",
  "jsx",
  "json",
  "markdown",
  "php",
  "py",
  "python",
  "rust",
  "shell",
  "sql",
  "tsx",
  "typescript",
  "xml",
  "yaml",
]);

async function createHighlighter(): Promise<
  HighlighterGeneric<BundledLanguage, BundledTheme>
> {
  return shikiCreateHighlighter({
    themes: ["github-dark", "github-light"],
    langs: ["javascript", "typescript", "markdown", "rust", "python", "php"],
  });
}

function MarkdownRender(props: MarkdownRenderProps) {
  const { lineNumbers, src, ...rest } = props;

  const components: MarkdownRenderProps["components"] = {
    figure: ({ children, node, ...figureProps }) => {
      const isCodeFigure = Boolean(
        node?.properties && "data-rehype-pretty-code-figure" in node.properties
      );
      return isCodeFigure ? (
        children
      ) : (
        <figure {...figureProps}>{children}</figure>
      );
    },
    pre: ({ children, node, ...preProps }) => {
      const value = extractText(node);
      const { className } = preProps;
      const codeNode = node?.children?.[0];
      const codeProperties =
        codeNode && "properties" in codeNode ? codeNode.properties : undefined;
      const language =
        readLanguage(node?.properties) ??
        readLanguage(codeProperties) ??
        readLanguage(preProps);
      const normalizedLanguage = language?.toLowerCase();
      const showLineNumbers =
        typeof lineNumbers === "function"
          ? lineNumbers(language)
          : (lineNumbers ??
            (normalizedLanguage !== undefined &&
              lineNumberLanguages.has(normalizedLanguage)));

      return (
        <CodeBlock>
          <CodeBlockHeader>
            <CodeBlockTitle>{language || "Code"}</CodeBlockTitle>
            <CodeBlockActions>
              <CodeBlockCopyButton value={value} />
            </CodeBlockActions>
          </CodeBlockHeader>
          <CodeBlockContent className={className} lineNumbers={showLineNumbers}>
            {children}
          </CodeBlockContent>
        </CodeBlock>
      );
    },
  };

  return (
    <MarkdownHooks
      remarkPlugins={[
        remarkGfm,
        remarkParse,
        remarkMath,
        [remarkRehype, { allowDangerousHtml: true }],
      ]}
      rehypePlugins={[
        rehypeRaw,
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
        [rehypeStringify, { allowDangerousHtml: true }],
      ]}
      {...rest}
      components={{ ...components, ...rest.components }}
    >
      {src}
    </MarkdownHooks>
  );
}

function readLanguage(properties: Record<string, unknown> | undefined) {
  if (!properties) return undefined;

  const value =
    properties["data-language"] ??
    properties.dataLanguage ??
    properties.language;

  if (typeof value === "string") return value;

  const className = properties.className;
  if (Array.isArray(className)) {
    const languageClass = className.find(
      (name): name is string =>
        typeof name === "string" && name.startsWith("language-")
    );
    return languageClass?.slice("language-".length);
  }

  if (typeof className === "string") {
    const languageClass = className
      .split(/\s+/)
      .find((name) => name.startsWith("language-"));
    return languageClass?.slice("language-".length);
  }

  return undefined;
}

function extractText(node: { children?: unknown[] } | undefined): string {
  if (!node?.children) return "";
  return node.children
    .map((child) => {
      if (typeof child === "string") return child;
      if (
        child &&
        typeof child === "object" &&
        "value" in child &&
        typeof child.value === "string"
      ) {
        return child.value;
      }
      if (child && typeof child === "object" && "children" in child) {
        return extractText(child as { children?: unknown[] });
      }
      return "";
    })
    .join("");
}

export { MarkdownRender };
