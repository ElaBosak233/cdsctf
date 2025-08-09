import {
  type BundledLanguage,
  type BundledTheme,
  type HighlighterGeneric,
  createHighlighter as shikiCreateHighlighter,
} from "shiki";

async function createHighlighter(): Promise<
  HighlighterGeneric<BundledLanguage, BundledTheme>
> {
  return shikiCreateHighlighter({
    themes: ["github-dark", "github-light"],
    langs: ["javascript", "typescript", "markdown", "rust", "python", "php"],
  });
}

export { createHighlighter };
