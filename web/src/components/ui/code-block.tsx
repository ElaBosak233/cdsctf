import {
  CheckIcon,
  ClipboardIcon,
  Code2Icon,
  DownloadIcon,
} from "lucide-react";
import type React from "react";
import { createContext, useContext } from "react";

import { Button } from "@/components/ui/button";
import { useClipboard } from "@/hooks/use-clipboard";
import { cn } from "@/utils";

type CodeBlockContextValue = {
  value: string;
  title?: React.ReactNode;
};

const CodeBlockContext = createContext<CodeBlockContextValue | null>(null);

function useCodeBlock() {
  const context = useContext(CodeBlockContext);
  if (!context) {
    throw new Error("CodeBlock components must be used inside CodeBlock");
  }
  return context;
}

type CodeBlockProps = React.ComponentProps<"div"> & {
  value: string;
  title?: React.ReactNode;
};

function CodeBlock({
  value,
  title,
  className,
  children,
  ...props
}: CodeBlockProps) {
  return (
    <CodeBlockContext.Provider value={{ value, title }}>
      <div
        data-slot="code-block"
        className={cn(
          "group/code-block relative flex min-w-0 flex-col overflow-hidden rounded-lg border border-border/80 bg-card text-sm text-foreground shadow-xs",
          "has-[>[data-slot=code-block-content]:focus-visible]:ring-2 has-[>[data-slot=code-block-content]:focus-visible]:ring-ring/30",
          className
        )}
        {...props}
      >
        {children}
      </div>
    </CodeBlockContext.Provider>
  );
}

function CodeBlockHeader({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-header"
      className={cn(
        "flex min-h-9 min-w-0 shrink-0 items-center gap-2 border-b border-border/70 bg-muted/35 px-3 text-xs text-muted-foreground",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
}

function CodeBlockTitle({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  const { title } = useCodeBlock();
  return (
    <div
      data-slot="code-block-title"
      className={cn(
        "flex min-w-0 items-center gap-1.5 truncate font-sans font-medium text-foreground",
        className
      )}
      {...props}
    >
      <Code2Icon className="size-3.5 shrink-0 text-muted-foreground" />
      <span className="truncate">{children ?? title}</span>
    </div>
  );
}

function CodeBlockActions({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-actions"
      className={cn("ml-auto flex shrink-0 items-center gap-1", className)}
      {...props}
    >
      {children}
    </div>
  );
}

function CodeBlockCopyButton({
  className,
  ...props
}: React.ComponentProps<typeof Button>) {
  const { value } = useCodeBlock();
  const { isCopied, copyToClipboard } = useClipboard();

  return (
    <Button
      aria-label={isCopied ? "Copied" : "Copy code"}
      title={isCopied ? "Copied" : "Copy code"}
      data-slot="code-block-copy-button"
      size="sm"
      square
      variant="ghost"
      className={cn("size-7", className)}
      icon={isCopied ? <CheckIcon /> : <ClipboardIcon />}
      onClick={() => void copyToClipboard(value)}
      {...props}
    />
  );
}

function CodeBlockDownloadButton({
  filename = "code.txt",
  className,
  ...props
}: React.ComponentProps<typeof Button> & { filename?: string }) {
  const { value } = useCodeBlock();

  const handleDownload = () => {
    const blob = new Blob([value], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = filename;
    anchor.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Button
      aria-label="Download code"
      title="Download code"
      data-slot="code-block-download-button"
      size="sm"
      square
      variant="ghost"
      className={cn("size-7", className)}
      icon={<DownloadIcon />}
      onClick={handleDownload}
      {...props}
    />
  );
}

function CodeBlockContent({
  className,
  children,
  ...props
}: React.ComponentProps<"pre">) {
  return (
    <pre
      data-slot="code-block-content"
      // biome-ignore lint/a11y/noNoninteractiveTabindex: Keyboard users need a focus target to scroll long code.
      tabIndex={0}
      className={cn(
        "!m-0 !max-w-none min-h-0 max-h-[30rem] min-w-0 overflow-auto !bg-background/65 !px-3 !py-3 text-start font-mono !text-xs !leading-[1.7] outline-none sm:!px-4 sm:!py-4 sm:!text-sm",
        "whitespace-pre [&>code]:grid [&>code]:min-w-max [&>code]:content-start [&_.line]:min-h-lh",
        "[&::-webkit-scrollbar]:h-2 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-foreground/15 hover:[&::-webkit-scrollbar-thumb]:bg-foreground/25",
        className
      )}
      {...props}
    >
      {children}
    </pre>
  );
}

function CodeBlockFooter({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-footer"
      className={cn(
        "flex items-center border-t bg-muted/35 px-3 py-1.5 text-xs text-muted-foreground",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
}

export {
  CodeBlock,
  CodeBlockActions,
  CodeBlockContent,
  CodeBlockCopyButton,
  CodeBlockDownloadButton,
  CodeBlockFooter,
  CodeBlockHeader,
  CodeBlockTitle,
};
