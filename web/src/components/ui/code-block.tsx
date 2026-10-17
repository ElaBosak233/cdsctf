import { cva, type VariantProps } from "class-variance-authority";
import {
  CheckIcon,
  ClipboardIcon,
  Code2Icon,
  DownloadIcon,
} from "lucide-react";
import * as React from "react";
import { useEffect, useState } from "react";

import { Button } from "@/components/ui/button";
import { cn } from "@/utils";

const codeBlockVariants = cva(
  "group/code-block relative flex w-full min-w-0 flex-col overflow-hidden text-sm has-[>[data-slot=code-block-content]:focus-visible]:ring-2 has-[>[data-slot=code-block-content]:focus-visible]:ring-ring/30 data-[size=sm]:text-xs",
  {
    variants: {
      variant: {
        default:
          "rounded-lg border border-border/80 bg-card text-foreground shadow-xs",
        destructive:
          "rounded-lg border border-error/25 bg-error/5 text-error shadow-xs dark:bg-error/10",
      },
    },
    defaultVariants: { variant: "default" },
  }
);

function CodeBlock({
  className,
  variant = "default",
  size = "default",
  ...props
}: React.ComponentProps<"div"> &
  VariantProps<typeof codeBlockVariants> & { size?: "default" | "sm" }) {
  return (
    <div
      data-slot="code-block"
      data-variant={variant}
      data-size={size}
      className={cn(codeBlockVariants({ variant }), className)}
      {...props}
    />
  );
}

function CodeBlockHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-header"
      className={cn(
        "flex min-h-9 min-w-0 shrink-0 items-center gap-2 border-b border-border/70 bg-muted/35 px-3 text-xs text-muted-foreground group-data-[size=sm]/code-block:min-h-8 *:has-[>[data-slot=code-block-title]]:min-w-0",
        className
      )}
      {...props}
    />
  );
}

function CodeBlockTitle({
  className,
  children,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-title"
      className={cn(
        "flex min-w-0 items-center gap-1.5 truncate font-sans text-xs font-medium text-foreground group-data-[size=sm]/code-block:text-[11px]",
        className
      )}
      {...props}
    >
      <Code2Icon className="size-3.5 shrink-0 text-muted-foreground" />
      <span className="truncate">{children}</span>
    </div>
  );
}

function CodeBlockActions({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-actions"
      className={cn(
        "-my-1 ml-auto flex shrink-0 items-center gap-1 [[data-slot=code-block]>&]:absolute [[data-slot=code-block]>&]:right-3 [[data-slot=code-block]>&]:top-3 [[data-slot=code-block]>&]:z-10 [[data-slot=code-block]>&]:m-0 [[data-slot=code-block]>&]:opacity-0 [[data-slot=code-block]>&]:transition-opacity [[data-slot=code-block]>&]:group-focus-within/code-block:opacity-100 [[data-slot=code-block]>&]:group-hover/code-block:opacity-100 [[data-slot=code-block]>&]:pointer-coarse:opacity-100 [[data-slot=code-block]>&]:*:data-[slot=button]:bg-background/90 [[data-slot=code-block]>&]:*:data-[slot=button]:ring-1 [[data-slot=code-block]>&]:*:data-[slot=button]:ring-foreground/10",
        className
      )}
      {...props}
    />
  );
}

function CodeBlockCopyButton({
  value,
  className,
  onClick,
  ...props
}: React.ComponentProps<typeof Button> & { value: string }) {
  const [copied, setCopied] = useState(false);
  useEffect(() => {
    if (!copied) return;
    const timeout = setTimeout(() => setCopied(false), 2000);
    return () => clearTimeout(timeout);
  }, [copied]);
  return (
    <>
      <Button
        data-slot="code-block-copy-button"
        data-copied={copied || undefined}
        aria-label={copied ? "Copied" : "Copy code"}
        title={copied ? "Copied" : "Copy code"}
        size="sm"
        square
        variant="ghost"
        className={cn("size-7", className)}
        onClick={(event) => {
          onClick?.(event);
          navigator.clipboard
            .writeText(value)
            .then(() => setCopied(true))
            .catch(() => undefined);
        }}
        {...props}
      >
        {copied ? <CheckIcon /> : <ClipboardIcon />}
      </Button>
      <span aria-live="polite" className="sr-only">
        {copied ? "Copied" : ""}
      </span>
    </>
  );
}

function CodeBlockDownloadButton({
  value,
  filename = "code.txt",
  className,
  ...props
}: React.ComponentProps<typeof Button> & { value: string; filename?: string }) {
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

type CodeBlockContentProps = React.ComponentProps<"pre"> & {
  wrap?: boolean;
  lineNumbers?: boolean;
  startLine?: number;
};

function CodeBlockContent({
  className,
  children,
  style,
  wrap = false,
  lineNumbers = false,
  startLine = 1,
  ...props
}: CodeBlockContentProps) {
  const code = getCodeContent(children, lineNumbers);
  return (
    <pre
      data-slot="code-block-content"
      data-wrap={wrap || undefined}
      data-numbered={lineNumbers || undefined}
      dir="ltr"
      // biome-ignore lint/a11y/noNoninteractiveTabindex: Keyboard users need a focus target to scroll long code.
      tabIndex={0}
      style={{ counterReset: `line ${startLine - 1}`, ...style }}
      className={cn(
        "!m-0 !max-w-none min-h-0 max-h-[30rem] min-w-0 overflow-auto !bg-background/65 !px-3 !py-3 text-start font-mono !text-xs !leading-[1.7] outline-none sm:!px-4 sm:!py-4 sm:!text-sm",
        "data-wrap:whitespace-pre-wrap data-wrap:break-words",
        "[&::-webkit-scrollbar]:h-2 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-foreground/15 hover:[&::-webkit-scrollbar-thumb]:bg-foreground/25",
        className
      )}
      {...props}
    >
      {code}
    </pre>
  );
}

function CodeBlockLine({
  className,
  children,
  ...props
}: React.ComponentProps<"span">) {
  const empty =
    children === undefined ||
    children === null ||
    children === "" ||
    (Array.isArray(children) && children.length === 0);
  return (
    <span
      data-slot="code-block-line"
      data-variant="default"
      className={cn(
        "relative -mx-3 block min-h-lh px-3 before:min-w-10 before:text-end before:text-muted-foreground after:absolute after:start-1 after:top-0 after:text-muted-foreground after:select-none",
        "sm:-mx-4 sm:px-4",
        "in-data-numbered:col-span-2 in-data-numbered:grid in-data-numbered:grid-cols-subgrid in-data-numbered:gap-x-3 in-data-numbered:[counter-increment:line] in-data-numbered:before:content-[counter(line)] in-data-numbered:before:select-none",
        className
      )}
      {...props}
    >
      <span data-slot="code-block-line-content" className="min-w-0">
        {empty ? "\n" : children}
      </span>
    </span>
  );
}

function CodeBlockFooter({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="code-block-footer"
      className={cn(
        "flex min-w-0 shrink-0 items-center gap-2 border-t border-border/70 bg-muted/35 px-3 py-1.5 text-xs text-muted-foreground",
        className
      )}
      {...props}
    />
  );
}

function getCodeContent(children: React.ReactNode, lineNumbers: boolean) {
  const child =
    React.isValidElement(children) && children.type === "code"
      ? (children as React.ReactElement<
          React.ComponentProps<"code"> & { "data-slot"?: string }
        >)
      : null;
  if (child) {
    const codeChildren = lineNumbers
      ? getHighlightedLines(child.props.children)
      : child.props.children;

    return React.cloneElement(child, {
      "data-slot": "code-block-code",
      className: cn(
        "grid min-h-lh min-w-max content-start",
        child.props.className
      ),
      children: codeChildren,
    });
  }
  const content =
    lineNumbers && typeof children === "string"
      ? children
          .replace(/\n$/, "")
          .split("\n")
          .map((line, index) => (
            // biome-ignore lint/suspicious/noArrayIndexKey: Static source lines do not reorder.
            <CodeBlockLine key={`${index}-${line}`}>{line}</CodeBlockLine>
          ))
      : children;
  return (
    <code
      data-slot="code-block-code"
      className="grid min-h-lh min-w-max content-start in-data-numbered:grid-cols-[auto_1fr] in-data-wrap:min-w-0"
    >
      {content}
    </code>
  );
}

function getHighlightedLines(children: React.ReactNode) {
  if (typeof children === "string") {
    return children
      .replace(/\n$/, "")
      .split("\n")
      .map((line, index) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: Static source lines do not reorder.
        <CodeBlockLine key={`${index}-${line}`}>{line}</CodeBlockLine>
      ));
  }

  return React.Children.toArray(children)
    .filter((line) => !(typeof line === "string" && line.trim() === ""))
    .map((line, index) => {
      if (!React.isValidElement(line)) {
        // biome-ignore lint/suspicious/noArrayIndexKey: Highlighted source lines are static.
        return <CodeBlockLine key={`line-${index}`}>{line}</CodeBlockLine>;
      }

      const highlightedLine = line as React.ReactElement<
        React.ComponentProps<"span">
      >;

      return (
        <CodeBlockLine
          // biome-ignore lint/suspicious/noArrayIndexKey: Highlighted source lines are static.
          key={`line-${index}`}
          {...highlightedLine.props}
          className={cn("line", highlightedLine.props.className)}
        >
          {highlightedLine.props.children}
        </CodeBlockLine>
      );
    });
}

export {
  CodeBlock,
  CodeBlockActions,
  CodeBlockContent,
  CodeBlockCopyButton,
  CodeBlockDownloadButton,
  CodeBlockFooter,
  CodeBlockHeader,
  CodeBlockLine,
  CodeBlockTitle,
};
