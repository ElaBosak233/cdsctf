import { DatabaseZapIcon, HeartCrackIcon, RefreshCcwIcon } from "lucide-react";
import { useRouteError } from "react-router";
import { ScrollArea } from "@/components/ui/scroll-area";
import { cn } from "@/utils";
import { Button } from "../ui/button";

function ErrorBoundary() {
  const error = useRouteError();
  if (!(error instanceof Error)) return null;

  function handleReload() {
    window.location.reload();
  }

  function handleCacheClear() {
    localStorage.clear();
    sessionStorage.clear();

    if (window.indexedDB && indexedDB.databases) {
      indexedDB.databases().then((dbs) => {
        dbs.forEach((db) => {
          if (db.name) indexedDB.deleteDatabase(db.name);
        });
      });
    }

    window.location.reload();
  }

  return (
    <div
      className={cn([
        "h-screen",
        "flex",
        "flex-col",
        "justify-center",
        "items-center",
      ])}
    >
      <div
        className={cn([
          "max-w-2xl",
          "flex",
          "flex-col",
          "items-center",
          "gap-5",
        ])}
      >
        <HeartCrackIcon className={cn(["size-10"])} />
        <span>发生了点错误</span>
        <ScrollArea
          className={cn(["h-96", "bg-card", "border", "rounded-lg", "p-5"])}
        >
          <pre className={cn(["text-wrap"])}>{error.stack}</pre>
        </ScrollArea>
        <div className={cn(["flex", "gap-5"])}>
          <Button
            icon={<RefreshCcwIcon />}
            variant={"solid"}
            onClick={handleReload}
            size={"lg"}
          >
            刷新
          </Button>
          <Button
            icon={<DatabaseZapIcon />}
            variant={"solid"}
            onClick={handleCacheClear}
            size={"lg"}
          >
            清除缓存
          </Button>
        </div>
      </div>
    </div>
  );
}

export { ErrorBoundary };
