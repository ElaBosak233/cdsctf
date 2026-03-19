import { ChevronDownIcon, FilterIcon, PlusCircleIcon } from "lucide-react";
import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { cn } from "@/utils";

export type AdminListPageViewProps = {
  hasSidebar: boolean;
  title: string;
  icon: ReactNode;
  addButtonLabel: string;
  onAddClick: () => void;
  filterLabel?: string;
  filterContent: ReactNode;
  tableContent: ReactNode;
  footerContent: ReactNode | null;
  collapsibleFilterOnMobile?: boolean;
};

export function AdminListPageView({
  hasSidebar,
  title,
  icon,
  addButtonLabel,
  onAddClick,
  filterLabel,
  filterContent,
  tableContent,
  footerContent,
  collapsibleFilterOnMobile = true,
}: AdminListPageViewProps) {
  const { t } = useTranslation();
  const filterTitle = filterLabel ?? t("common:filter");

  return (
    <div className={cn("overflow-hidden flex flex-col min-h-0 h-full")}>
      <div
        className={cn(
          "flex flex-col flex-1 min-h-0 overflow-hidden",
          "px-4 py-4 sm:px-6 sm:py-6 lg:px-8 lg:py-8"
        )}
      >
        {/* 无侧栏时：标题 + 添加按钮，由 Tailwind 控制显隐 */}
        <header
          className={cn(
            "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between mb-4 sm:mb-6",
            hasSidebar && "hidden"
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

        {/* 无侧栏时的筛选区：显隐与折叠由 Tailwind + cn 控制 */}
        <div className={cn("mb-4 sm:mb-5", hasSidebar && "hidden")}>
          {collapsibleFilterOnMobile ? (
            <>
              <div className={cn("sm:hidden")}>
                <Collapsible defaultOpen={false} className="group/collapse">
                  <CollapsibleTrigger asChild>
                    <Button
                      variant="outline"
                      className={cn(
                        "w-full justify-between min-h-11 touch-manipulation",
                        "rounded-xl border bg-card/50 ring-1 ring-border/50"
                      )}
                    >
                      <span className="flex items-center gap-2 text-muted-foreground text-sm font-medium">
                        <FilterIcon className="size-4" />
                        {filterTitle}
                      </span>
                      <ChevronDownIcon className="size-4 shrink-0 transition-transform group-data-[state=open]/collapse:rotate-180" />
                    </Button>
                  </CollapsibleTrigger>
                  <CollapsibleContent>
                    <Card
                      className={cn(
                        "mt-2 rounded-xl border bg-card/50 p-4",
                        "ring-1 ring-border/50 shadow-sm"
                      )}
                    >
                      {filterContent}
                    </Card>
                  </CollapsibleContent>
                </Collapsible>
              </div>
              <div className={cn("hidden sm:block")}>
                <Card
                  className={cn(
                    "rounded-xl border bg-card/50 p-4",
                    "ring-1 ring-border/50 shadow-sm"
                  )}
                >
                  <div
                    className={cn(
                      "flex items-center gap-2 text-muted-foreground text-sm font-medium mb-3"
                    )}
                  >
                    <FilterIcon className="size-4" />
                    {filterTitle}
                  </div>
                  {filterContent}
                </Card>
              </div>
            </>
          ) : (
            <Card
              className={cn(
                "rounded-xl border bg-card/50 p-4",
                "ring-1 ring-border/50 shadow-sm"
              )}
            >
              <div
                className={cn(
                  "flex items-center gap-2 text-muted-foreground text-sm font-medium mb-3"
                )}
              >
                <FilterIcon className="size-4" />
                {filterTitle}
              </div>
              {filterContent}
            </Card>
          )}
        </div>

        <header
          className={cn(
            "flex items-center justify-between gap-3 mb-3 xl:mb-3 sm:min-h-11",
            hasSidebar && "hidden"
          )}
        >
          <Button
            icon={<PlusCircleIcon className="size-4" />}
            variant="solid"
            size="sm"
            onClick={onAddClick}
            className={cn(
              "xl:hidden min-h-10 touch-manipulation",
              "w-full sm:w-auto"
            )}
          >
            {addButtonLabel}
          </Button>
        </header>

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

          {footerContent != null && (
            <footer
              className={cn(
                "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
                "px-3 py-3 sm:px-4 sm:py-3 border-t bg-muted/30 shrink-0",
                "touch-manipulation"
              )}
            >
              {footerContent}
            </footer>
          )}
        </Card>
      </div>
    </div>
  );
}
