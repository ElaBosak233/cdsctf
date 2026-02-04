import type { EditorView } from "@uiw/react-codemirror";
import {
  BoldIcon,
  CodeIcon,
  HeadingIcon,
  ImageIcon,
  ItalicIcon,
  ListIcon,
  ListOrderedIcon,
} from "lucide-react";
import type { ChangeEvent } from "react";
import { useCallback, useEffect, useRef } from "react";
import { Button } from "@/components/ui/button";
import { Editor, type EditorProps } from "@/components/ui/editor";
import { cn } from "@/utils";
import { uploadFile } from "@/utils/file";

type MarkdownEditorProps = Omit<EditorProps, "lang"> & {
  toolbarClassName?: string;
};

const toolbarItems = [
  {
    key: "heading",
    icon: <HeadingIcon />,
  },
  {
    key: "bold",
    icon: <BoldIcon />,
  },
  {
    key: "italic",
    icon: <ItalicIcon />,
  },
  {
    key: "list",
    icon: <ListIcon />,
  },
  {
    key: "ordered-list",
    icon: <ListOrderedIcon />,
  },
  {
    key: "code",
    icon: <CodeIcon />,
  },
  {
    key: "image",
    icon: <ImageIcon />,
  },
] as const;

function MarkdownEditor(props: MarkdownEditorProps) {
  const { value = "", onChange, className, toolbarClassName, ...rest } = props;

  const viewRef = useRef<EditorView | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);

  const insertAtCursor = useCallback(
    (text: string) => {
      const view = viewRef.current;
      if (view) {
        const { from, to } = view.state.selection.main;
        view.dispatch({
          changes: { from, to, insert: text },
          selection: { anchor: from + text.length },
        });
        return { from, to: from + text.length };
      }

      const source = value ?? "";
      const prefix = source.length > 0 && !source.endsWith("\n") ? "\n" : "";
      const next = `${source}${prefix}${text}`;
      onChange?.(next);
      return {
        from: next.length - text.length,
        to: next.length,
      };
    },
    [onChange, value]
  );

  const applyLinePrefix = useCallback(
    (prefix: string, mode: "heading" | "list" | "ordered") => {
      const view = viewRef.current;
      if (!view) {
        insertAtCursor(prefix);
        return;
      }

      const { from, to } = view.state.selection.main;
      const startLine = view.state.doc.lineAt(from);
      const endLine = view.state.doc.lineAt(to);
      const changes: { from: number; to: number; insert: string }[] = [];

      let index = 1;
      for (let lineNo = startLine.number; lineNo <= endLine.number; lineNo++) {
        const line = view.state.doc.line(lineNo);
        const lineText = line.text;
        const indent = lineText.match(/^\s*/)?.[0] ?? "";
        const rest = lineText.slice(indent.length);

        let next = rest;
        if (mode === "heading") {
          if (!rest.startsWith("# ")) {
            next = `# ${rest}`;
          }
        } else if (mode === "list") {
          if (!/^(?:[-*+]\s+)/.test(rest)) {
            next = `- ${rest}`;
          }
        } else {
          next = rest.replace(/^\d+\.\s+/, "");
          next = `${index}. ${next}`;
          index += 1;
        }

        changes.push({
          from: line.from,
          to: line.to,
          insert: `${indent}${next}`,
        });
      }

      view.dispatch({ changes });
    },
    [insertAtCursor]
  );

  const wrapSelection = useCallback(
    (left: string, right: string = left) => {
      const view = viewRef.current;
      if (!view) {
        insertAtCursor(`${left}${right}`);
        return;
      }

      const { from, to } = view.state.selection.main;
      if (from === to) {
        const insert = `${left}${right}`;
        view.dispatch({
          changes: { from, to, insert },
          selection: { anchor: from + left.length },
        });
        return;
      }

      const selected = view.state.doc.sliceString(from, to);
      const insert = `${left}${selected}${right}`;
      view.dispatch({
        changes: { from, to, insert },
        selection: {
          anchor: from + left.length,
          head: from + left.length + selected.length,
        },
      });
    },
    [insertAtCursor]
  );

  const handleImageClick = useCallback(() => {
    fileInputRef.current?.click();
  }, []);

  const applyToolbarAction = useCallback(
    (key: (typeof toolbarItems)[number]["key"]) => {
      switch (key) {
        case "heading":
          applyLinePrefix("# ", "heading");
          return;
        case "list":
          applyLinePrefix("- ", "list");
          return;
        case "ordered-list":
          applyLinePrefix("1. ", "ordered");
          return;
        case "code":
          wrapSelection("`");
          return;
        case "bold":
          wrapSelection("**");
          return;
        case "italic":
          wrapSelection("*");
          return;
        case "image":
          handleImageClick();
          return;
      }
    },
    [applyLinePrefix, wrapSelection, handleImageClick]
  );

  const uploadAndInsert = useCallback(
    async (file: File) => {
      const placeholder = "![Uploading](...)";
      const { from, to } = insertAtCursor(placeholder);

      try {
        const res = await uploadFile("/api/media", [file]);
        const hash = res?.data as string | undefined;
        if (!hash) return;

        const view = viewRef.current;
        const replacement = `![${hash}](/api/media?hash=${hash})`;
        if (view) {
          const safeFrom = Math.min(from, view.state.doc.length);
          const safeTo = Math.min(to, view.state.doc.length);
          const current = view.state.doc.sliceString(safeFrom, safeTo);
          if (current === placeholder) {
            view.dispatch({
              changes: { from: safeFrom, to: safeTo, insert: replacement },
              selection: { anchor: safeFrom + replacement.length },
            });
            return;
          }
        }

        const source = value ?? "";
        onChange?.(source.replace(placeholder, replacement));
      } catch {
        const view = viewRef.current;
        if (view) {
          const safeFrom = Math.min(from, view.state.doc.length);
          const safeTo = Math.min(to, view.state.doc.length);
          const current = view.state.doc.sliceString(safeFrom, safeTo);
          if (current === placeholder) {
            view.dispatch({
              changes: { from: safeFrom, to: safeTo, insert: "" },
              selection: { anchor: safeFrom },
            });
            return;
          }
        }

        const source = value ?? "";
        onChange?.(source.replace(placeholder, ""));
      }
    },
    [insertAtCursor, onChange, value]
  );

  const handleImageSelected = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    event.target.value = "";
    if (!file) return;

    await uploadAndInsert(file);
  };

  const handlePaste = useCallback(
    (event: ClipboardEvent) => {
      const items = event.clipboardData?.items;
      if (!items || items.length === 0) return;

      const files: File[] = [];
      for (const item of items) {
        if (item.kind === "file") {
          const file = item.getAsFile();
          if (file?.type?.startsWith("image/")) {
            files.push(file);
          }
        }
      }

      if (files.length === 0) return;
      event.preventDefault();

      void (async () => {
        for (const file of files) {
          await uploadAndInsert(file);
        }
      })();
    },
    [uploadAndInsert]
  );

  useEffect(() => {
    const view = viewRef.current;
    if (!view) return;
    view.dom.addEventListener("paste", handlePaste as EventListener);
    return () => {
      view.dom.removeEventListener("paste", handlePaste as EventListener);
    };
  }, [handlePaste]);

  return (
    <div className={cn(["relative", "w-full", "flex", "flex-col", className])}>
      <div
        className={cn([
          "flex",
          "items-center",
          "gap-1",
          "border-b",
          "bg-primary/20",
          "rounded-t-md",
          "p-1",
          toolbarClassName,
        ])}
      >
        {toolbarItems.map((item) => (
          <Button
            key={item.key}
            size="sm"
            square
            variant="ghost"
            icon={item.icon}
            onClick={() => applyToolbarAction(item.key)}
          />
        ))}
      </div>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        className="hidden"
        onChange={handleImageSelected}
      />
      <Editor
        lang="markdown"
        value={value}
        onChange={onChange}
        onCreateEditor={(view) => {
          viewRef.current = view;
        }}
        className={cn(["h-full", "rounded-b-md", "rounded-t-none"])}
        {...rest}
      />
    </div>
  );
}

export { MarkdownEditor, type MarkdownEditorProps };
