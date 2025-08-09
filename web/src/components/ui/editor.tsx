import { type Diagnostic, linter } from "@codemirror/lint";
import { langs, loadLanguage } from "@uiw/codemirror-extensions-langs";
import { vscodeDark, vscodeLight } from "@uiw/codemirror-theme-vscode";
import CodeMirror, { EditorView } from "@uiw/react-codemirror";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";

loadLanguage("rust");
loadLanguage("markdown");

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

  const { computedTheme } = useApperanceStore();

  const theme = computedTheme === "dark" ? vscodeDark : vscodeLight;

  function getLanguage() {
    switch (lang) {
      case "rust":
      case "rune":
        return langs.rust();
      default:
        return langs.markdown();
    }
  }

  function getDiagnosticsExtension() {
    return linter((view) => {
      const doc = view.state.doc;
      const result: Diagnostic[] =
        diagnostics?.map((d) => {
          const from = doc.line(d.start_line + 1).from + d.start_column;
          const to = doc.line(d.end_line + 1).from + d.end_column;

          return {
            from,
            to,
            severity: d.kind,
            message: d.message,
          };
        }) ?? [];

      return result;
    });
  }

  const themeOverwrite = EditorView.theme({
    "&": {
      fontSize: "14px",
      backgroundColor: "transparent",
      height: "100%",
      width: "100%",
      position: "relative",
    },
    "&.cm-editor .cm-scroller": {
      fontFamily: ["Ubuntu Sans Mono Variable"].join(","),
      lineHeight: "1.6",
    },
    ".cm-gutters": {
      backgroundColor: "transparent",
    },
    "&.cm-editor.cm-focused": {
      outline: "none",
    },
    ".cm-scroller::-webkit-scrollbar": {
      width: "6px",
      height: "6px",
    },
  });

  return (
    <div
      className={cn([
        "relative",
        "w-full",
        "rounded-md",
        "border",
        "bg-input",
        "ring-offset-input",
        "focus-within:outline-hidden",
        "focus-within:ring-2",
        "focus-within:ring-ring",
        "focus-within:ring-offset-2",
        "focus-within:border-transparent",
        className,
      ])}
      {...rest}
    >
      <CodeMirror
        basicSetup={{
          lineNumbers: showLineNumbers,
          highlightActiveLine: false,
          highlightActiveLineGutter: false,
          syntaxHighlighting: true,
          foldGutter: false,
          tabSize: tabSize,
        }}
        value={value}
        onChange={(value) => onChange?.(value)}
        theme={[themeOverwrite, theme]}
        placeholder={placeholder}
        extensions={[getLanguage(), getDiagnosticsExtension()]}
        className={cn([
          "absolute",
          "left-0",
          "top-0",
          "bottom-0",
          "right-0",
          "inset-0",
          "p-2",
        ])}
      />
    </div>
  );
}

export { Editor, type EditorProps };
