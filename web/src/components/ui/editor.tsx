import { shikiToMonaco } from "@shikijs/monaco";
import * as monaco from "monaco-editor-core";
import EditorWorker from "monaco-editor-core/esm/vs/editor/editor.worker?worker";
import { useEffect, useRef, useState } from "react";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";
import { createHighlighter } from "./shiki";

declare global {
  interface Window {
    MonacoEnvironment?: monaco.Environment;
  }
}

(window as Window).MonacoEnvironment = {
  getWorker: (_: string, _label: string) => {
    return new EditorWorker();
  },
};

monaco.languages.register({ id: "markdown" });
monaco.languages.register({ id: "rust" });
monaco.languages.register({ id: "html" });
monaco.languages.register({ id: "css" });
monaco.languages.register({ id: "javascript" });
monaco.languages.register({ id: "typescript" });

shikiToMonaco(await createHighlighter(), monaco);

type EditorProps = Omit<React.ComponentProps<"div">, "onChange"> & {
  value?: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  lang?: string;
  tabSize?: number;
  showLineNumbers?: boolean;
  diagnostics?: Array<{
    start_line: number;
    start_column: number;
    end_line: number;
    end_column: number;
    kind: "error" | "warning";
    message: string;
  }>;
  className?: string;
};

function Editor(props: EditorProps) {
  const {
    value = "",
    onChange,
    placeholder,
    lang = "markdown",
    tabSize = 2,
    showLineNumbers = false,
    diagnostics = [],
    className,
    ...rest
  } = props;

  const valueRef = useRef<string>(value);

  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const containerRef = useRef<HTMLPreElement | null>(null);

  const [focused, setFocused] = useState<boolean>(false);
  const { computedTheme } = useApperanceStore();

  const monacoTheme = computedTheme === "dark" ? "github-dark" : "github-light";

  useEffect(() => {
    if (containerRef.current) {
      const editor = monaco.editor.create(containerRef.current, {
        value: valueRef.current,
        language: lang,
        theme: monacoTheme,
        fontSize: 15,
        fontFamily: ["Ubuntu Sans Mono"].join(","),
        lineHeight: 1.5,
        tabSize,
        insertSpaces: true,
        lineNumbers: showLineNumbers ? "on" : "off",
        glyphMargin: false,
        lineNumbersMinChars: 4,
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        automaticLayout: true,
        cursorStyle: "line",
        cursorBlinking: "smooth",
        renderWhitespace: "none",
        renderControlCharacters: false,
        hideCursorInOverviewRuler: true,
        overviewRulerLanes: 0,
        renderLineHighlight: "none",
        renderValidationDecorations: "on",
        folding: showLineNumbers,
        showFoldingControls: showLineNumbers ? "mouseover" : "never",
        matchBrackets: "never",
        selectionHighlight: false,
        codeLens: false,
        contextmenu: false,
        links: false,
        colorDecorators: false,
        scrollbar: {
          vertical: "auto",
          horizontal: "auto",
          verticalScrollbarSize: 6,
          horizontalScrollbarSize: 6,
          alwaysConsumeMouseWheel: false,
        },
        find: {
          addExtraSpaceOnTop: false,
          autoFindInSelection: "never",
          seedSearchStringFromSelection: "always",
        },
      });

      editorRef.current = editor;

      if (placeholder && !valueRef.current) {
        const decorationCollection = editor.createDecorationsCollection([
          {
            range: new monaco.Range(1, 1, 1, 1),
            options: {
              after: {
                content: placeholder,
                inlineClassName: "monaco-placeholder",
              },
            },
          },
        ]);

        const disposable = editor.onDidChangeModelContent(() => {
          const currentValue = editor.getValue();
          if (currentValue) {
            decorationCollection.clear();
            disposable.dispose();
          }
        });
      }

      editor.onDidChangeModelContent(() => {
        const currentValue = editor.getValue();
        if (onChange) onChange(currentValue);
      });

      editor.onDidFocusEditorText(() => {
        setFocused(true);
      });

      editor.onDidBlurEditorText(() => {
        setFocused(false);
      });

      return () => {
        editor.dispose();
      };
    }
  }, [lang, tabSize, showLineNumbers, placeholder, onChange, monacoTheme]);

  useEffect(() => {
    if (editorRef.current && value !== editorRef.current.getValue()) {
      editorRef.current.setValue(value);
    }

    valueRef.current = value;
  }, [value]);

  useEffect(() => {
    if (!editorRef.current) return;

    if (!diagnostics) {
      monaco.editor.setModelMarkers(
        editorRef.current.getModel()!,
        "diagnostics",
        []
      );
      return;
    }

    const model = editorRef.current.getModel();
    if (!model) return;

    const markers: monaco.editor.IMarkerData[] = diagnostics.map((d) => ({
      startLineNumber: d.start_line + 1,
      startColumn: d.start_column + 1,
      endLineNumber: d.end_line + 1,
      endColumn: d.end_column + 1,
      message: d.message,
      severity:
        d.kind === "error"
          ? monaco.MarkerSeverity.Error
          : monaco.MarkerSeverity.Warning,
    }));

    monaco.editor.setModelMarkers(model, "diagnostics", markers);
  }, [diagnostics]);

  return (
    <div
      className={cn([
        "relative",
        "w-full",
        "rounded-md",
        "border",
        "bg-input",
        "ring-offset-input",
        focused && [
          "outline-hidden",
          "ring-2",
          "ring-ring",
          "ring-offset-2",
          "border-transparent",
        ],
        className,
      ])}
      {...rest}
    >
      <div
        className={cn([
          "absolute",
          "left-0",
          "top-0",
          "bottom-0",
          "right-0",
          "inset-0",
          "p-2",
        ])}
      >
        <pre
          ref={containerRef}
          className={cn(["w-full", "h-full", "relative"])}
        />
      </div>
    </div>
  );
}

export { Editor, type EditorProps };
