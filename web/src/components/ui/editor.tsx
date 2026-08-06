import {
  autocompletion,
  type Completion,
  type CompletionContext,
  type CompletionResult,
} from "@codemirror/autocomplete";
import { type Diagnostic, linter } from "@codemirror/lint";
import { langs, loadLanguage } from "@uiw/codemirror-extensions-langs";
import { vscodeDark, vscodeLight } from "@uiw/codemirror-theme-vscode";
import CodeMirror, { EditorView } from "@uiw/react-codemirror";
import { useApperanceStore } from "@/storages/appearance";
import { cn } from "@/utils";

const luaCompletions: Record<string, Completion[]> = {
  "": [
    { label: "checker", type: "namespace", info: "Checker Lua APIs" },
    { label: "crypto", type: "namespace", info: "Cryptographic hash helpers" },
    { label: "http", type: "namespace", info: "HTTP request helpers" },
    { label: "json", type: "namespace", info: "JSON encode/decode helpers" },
    { label: "log", type: "namespace", info: "Script logging APIs" },
    { label: "regex", type: "namespace", info: "Regular expression helpers" },
    { label: "assert", type: "function" },
    { label: "error", type: "function" },
    { label: "ipairs", type: "function" },
    { label: "pairs", type: "function" },
    { label: "pcall", type: "function" },
    { label: "select", type: "function" },
    { label: "tonumber", type: "function" },
    { label: "tostring", type: "function" },
    { label: "type", type: "function" },
    { label: "xpcall", type: "function" },
    { label: "coroutine", type: "namespace" },
    { label: "math", type: "namespace" },
    { label: "string", type: "namespace" },
    { label: "table", type: "namespace" },
    { label: "utf8", type: "namespace" },
  ],
  checker: [
    {
      label: "audit",
      type: "namespace",
      info: "Checker status and flag helpers",
    },
    { label: "fs", type: "namespace", info: "Checker file storage APIs" },
    { label: "leet", type: "namespace", info: "Leet flag encoding helpers" },
    { label: "suid", type: "namespace", info: "SUID encoding helpers" },
  ],
  "checker.audit": [
    { label: "cheat", type: "function", info: "cheat(operator_id)" },
    { label: "correct", type: "function", info: "correct()" },
    { label: "format", type: "function", info: "format(flag)" },
    { label: "incorrect", type: "function", info: "incorrect()" },
    { label: "new", type: "function", info: "new()" },
    { label: "parse", type: "function", info: "parse(content)" },
  ],
  crypto: [
    { label: "sha256", type: "function", info: "sha256(value)" },
    { label: "sha512", type: "function", info: "sha512(value)" },
  ],
  "checker.fs": [
    { label: "read_to_string", type: "function", info: "read_to_string(path)" },
    { label: "write", type: "function", info: "write(path, content)" },
  ],
  http: [
    {
      label: "request",
      type: "function",
      info: "request(method, url, headers, body)",
    },
    { label: "url_encode", type: "function", info: "url_encode(value)" },
  ],
  json: [
    { label: "decode", type: "function", info: "decode(value)" },
    { label: "encode", type: "function", info: "encode(value)" },
  ],
  "checker.leet": [
    {
      label: "decode",
      type: "function",
      info: "decode(template, payload, options?: { key?: string })",
    },
    {
      label: "encode",
      type: "function",
      info: "encode(template, operator_id, options?: { key?: string })",
    },
  ],
  regex: [
    { label: "is_match", type: "function", info: "is_match(pattern, value)" },
  ],
  "checker.suid": [
    {
      label: "decode",
      type: "function",
      info: "decode(payload, options?: { key?: string })",
    },
    {
      label: "encode",
      type: "function",
      info: "encode(data, options?: { key?: string; hyphenated?: boolean })",
    },
  ],
  log: [
    { label: "debug", type: "function", info: "debug(...)" },
    { label: "error", type: "function", info: "error(...)" },
    { label: "info", type: "function", info: "info(...)" },
    { label: "warn", type: "function", info: "warn(...)" },
  ],
  coroutine: [
    { label: "create", type: "function" },
    { label: "resume", type: "function" },
    { label: "running", type: "function" },
    { label: "status", type: "function" },
    { label: "wrap", type: "function" },
    { label: "yield", type: "function" },
  ],
  math: [
    { label: "abs", type: "function" },
    { label: "ceil", type: "function" },
    { label: "floor", type: "function" },
    { label: "max", type: "function" },
    { label: "min", type: "function" },
    { label: "random", type: "function" },
    { label: "sqrt", type: "function" },
  ],
  string: [
    { label: "byte", type: "function" },
    { label: "char", type: "function" },
    { label: "find", type: "function" },
    { label: "format", type: "function" },
    { label: "gmatch", type: "function" },
    { label: "gsub", type: "function" },
    { label: "len", type: "function" },
    { label: "match", type: "function" },
    { label: "sub", type: "function" },
    { label: "upper", type: "function" },
  ],
  table: [
    { label: "concat", type: "function" },
    { label: "insert", type: "function" },
    { label: "remove", type: "function" },
    { label: "sort", type: "function" },
    { label: "unpack", type: "function" },
  ],
  utf8: [
    { label: "char", type: "function" },
    { label: "codepoint", type: "function" },
    { label: "codes", type: "function" },
    { label: "len", type: "function" },
    { label: "offset", type: "function" },
  ],
};

function luaCompletionSource(
  context: CompletionContext
): CompletionResult | null {
  const word = context.matchBefore(/[A-Za-z_][A-Za-z0-9_]*$/);
  const from = word?.from ?? context.pos;
  const before = context.state.sliceDoc(0, from);
  const pathMatch = before.match(
    /(?:[A-Za-z_][A-Za-z0-9_]*\.)+[A-Za-z_][A-Za-z0-9_]*\.$/
  );
  const namespace =
    pathMatch?.[0].slice(0, -1) ??
    before.match(/(?:[A-Za-z_][A-Za-z0-9_]*\.)+$/)?.[0].slice(0, -1) ??
    "";
  const options = luaCompletions[namespace];

  if (!options && !context.explicit) return null;
  return {
    from,
    options: options ?? luaCompletions[""],
    validFor: /^[A-Za-z_][A-Za-z0-9_]*$/,
  };
}

loadLanguage("lua");
loadLanguage("markdown");

type EditorProps = Omit<React.ComponentProps<"div">, "onChange"> & {
  value?: string;
  onChange?: (value: string) => void;
  onCreateEditor?: (view: EditorView) => void;
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
    onCreateEditor,
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
      case "rs":
      case "rust":
        return langs.rs();
      case "lua":
        return langs.lua();
      default:
        return langs.markdown();
    }
  }

  function getDiagnosticsExtension() {
    return linter((view) => {
      const doc = view.state.doc;
      const result: Diagnostic[] =
        diagnostics?.map((d) => {
          const position = (line: number, column: number) => {
            const lineNumber = Math.min(Math.max(line + 1, 1), doc.lines);
            const lineInfo = doc.line(lineNumber);
            const safeColumn = Math.min(Math.max(column, 0), lineInfo.length);
            return lineInfo.from + safeColumn;
          };
          const from = position(d.start_line, d.start_column);
          const to = Math.max(from, position(d.end_line, d.end_column));

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
          autocompletion: lang === "lua",
        }}
        value={value}
        onChange={(value) => onChange?.(value)}
        onCreateEditor={onCreateEditor}
        theme={[themeOverwrite, theme]}
        placeholder={placeholder}
        extensions={[
          getLanguage(),
          getDiagnosticsExtension(),
          ...(lang === "lua"
            ? [autocompletion({ override: [luaCompletionSource] })]
            : []),
        ]}
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
