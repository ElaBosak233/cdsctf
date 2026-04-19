import { PlusCircleIcon } from "lucide-react";
import type { ReactNode } from "react";
import { useContext } from "react";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { cn } from "@/utils";
import { AdminListContext } from "./context";

export type AdminListPageViewProps = {
  title: string;
  icon: ReactNode;
  addButtonLabel: string;
  onAddClick: () => void;
  filterContent: ReactNode;
  tableContent: ReactNode;
  footerContent: ReactNode;
};

export function AdminListPageView({
  title,
  icon,
  addButtonLabel,
  onAddClick,
  filterContent,
  tableContent,
  footerContent,
}: AdminListPageViewProps) {
  const hasSidebar = useContext(AdminListContext) != null;

  return (
    <div className={cn("overflow-hidden flex flex-col min-h-0 h-full")}>
      <div
        className={cn(
          "flex flex-col flex-1 min-h-0 overflow-hidden",
          "px-4 py-4 sm:px-6 sm:py-6 lg:px-8 lg:py-8"
        )}
      >
        {!hasSidebar && (
          <>
            <header
              className={cn(
                "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between mb-4 sm:mb-6"
              )}
            >
              <h1
                className={cn(
                  "text-xl sm:text-2xl font-semibold tracking-tight flex items-center gap-2.5 text-foreground"
                )}
              >
                <span
                  className={cn(
                    "flex items-center justify-center size-9 sm:size-10 rounded-lg bg-primary/10 text-primary shrink-0"
                  )}
                >
                  {icon}
                </span>
                <span className="truncate">{title}</span>
              </h1>
              <Button
                icon={<PlusCircleIcon className="size-4" />}
                variant="solid"
                onClick={onAddClick}
                className={cn(
                  "shrink-0 w-full sm:w-auto min-h-11 sm:min-h-9",
                  "touch-manipulation"
                )}
              >
                {addButtonLabel}
              </Button>
            </header>

            <div className={cn("mb-4 sm:mb-5")}>
              <Card
                className={cn(
                  "rounded-xl border bg-card/50 p-4",
                  "ring-1 ring-border/50 shadow-sm"
                )}
              >
                {filterContent}
              </Card>
            </div>
          </>
        )}

        <Card
          className={cn(
            "flex-1 min-h-0 flex flex-col overflow-hidden rounded-xl sm:rounded-xl",
            "ring-1 ring-border/50 shadow-sm"
          )}
        >
          <div
            className={cn(
              "flex-1 min-h-0 overflow-hidden rounded-b-xl",
              "overflow-x-auto min-w-0"
            )}
          >
            {tableContent}
          </div>

          <footer
            className={cn(
              "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
              "px-3 py-3 sm:px-4 sm:py-3 border-t bg-muted/30 shrink-0",
              "touch-manipulation"
            )}
          >
            {footerContent}
          </footer>
        </Card>
      </div>
    </div>
  );
}
