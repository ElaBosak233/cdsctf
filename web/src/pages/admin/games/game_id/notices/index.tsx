import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { MessageCircleIcon, PlusCircleIcon } from "lucide-react";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";
import { getManagementGameNotices } from "@/api/management/games/game_id/notices";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  type ColumnVisibilityState,
  flexRender,
  useDataTable,
} from "@/hooks/use-data-table";
import type { AdminGameNoticeView } from "@/models/game_notice";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import { parseRouteNumericId } from "@/utils/query";
import { Context } from "../context";
import { useColumns } from "./_blocks/columns";
import { NoticeFormDialog } from "./_blocks/notice-form-dialog";

export default function Index() {
  const { t } = useTranslation();

  const sharedStore = useSharedStore();

  const { game_id } = useParams<{ game_id: string }>();
  const routeGameId = parseRouteNumericId(game_id);
  const { game } = useContext(Context);

  const [createDialogOpen, setCreateDialogOpen] = useState(false);

  const [columnVisibility, setColumnVisibility] =
    useState<ColumnVisibilityState>({
      id: false,
    });
  const gameId = routeGameId ?? game?.id;
  const noticesQuery = useQuery({
    queryKey: ["admin", "game-notices", gameId, sharedStore.refresh],
    queryFn: () => getManagementGameNotices({ game_id: gameId! }),
    enabled: gameId != null,
    placeholderData: keepPreviousData,
  });
  const notices = noticesQuery.data?.notices ?? [];
  const total = noticesQuery.data?.total ?? 0;
  const loading = noticesQuery.isFetching;

  const columns = useColumns();
  const table = useDataTable<AdminGameNoticeView>({
    data: notices,
    columns,
    rowCount: total,
    onColumnVisibilityChange: setColumnVisibility,
    state: {
      columnVisibility,
    },
  });

  return (
    <div
      className={cn([
        "overflow-hidden",
        "h-full",
        "w-full",
        "min-w-0",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-4",
        "px-4",
        "py-4",
        "sm:px-6",
        "sm:py-6",
        "lg:px-8",
        "lg:py-8",
        "xl:h-(--app-content-height)",
      ])}
    >
      <div
        className={cn([
          "flex",
          "justify-between",
          "items-center",
          "shrink-0",
          "gap-6",
        ])}
      >
        <h1
          className={cn([
            "text-2xl",
            "font-bold",
            "flex",
            "gap-2",
            "items-center",
          ])}
        >
          <MessageCircleIcon />
          {t("game:notice._")}
        </h1>
        <div
          className={cn([
            "flex",
            "flex-1",
            "justify-end",
            "items-center",
            "gap-3",
          ])}
        >
          <Button
            icon={<PlusCircleIcon />}
            variant="solid"
            onClick={() => setCreateDialogOpen(true)}
          >
            {t("common:actions.add")}
          </Button>
          <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
            <DialogContent size="wide">
              {createDialogOpen && (
                <NoticeFormDialog onClose={() => setCreateDialogOpen(false)} />
              )}
            </DialogContent>
          </Dialog>
        </div>
      </div>

      <ScrollArea
        className={cn([
          "flex-1",
          "min-h-0",
          "min-w-0",
          "w-full",
          "max-w-full",
          "overflow-hidden",
          "rounded-lg",
          "border",
          "ring-1",
          "ring-border/50",
          "shadow-sm",
        ])}
      >
        <LoadingOverlay loading={loading} />
        <Table
          className={cn([
            "w-full",
            "min-w-full",
            "table-auto",
            "max-w-none",
            "text-foreground",
          ])}
        >
          <TableHeader
            className={cn([
              "sticky",
              "top-0",
              "z-2",
              "bg-muted/80",
              "backdrop-blur-sm",
              "border-b",
            ])}
          >
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead
                      key={header.id}
                      className={cn([
                        "bg-muted/95",
                        header.column.id === "title" && "w-80",
                        header.column.id === "content" && "min-w-72",
                        header.column.id === "author" && "w-80 min-w-80",
                        header.column.id === "actions" && "w-28 min-w-28",
                      ])}
                    >
                      {!header.isPlaceholder &&
                        flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  );
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length
              ? table.getRowModel().rows.map((row) => (
                  <TableRow
                    key={row.getValue("id")}
                    data-state={row.getIsSelected() && "selected"}
                    className={cn([
                      "group",
                      "transition-colors",
                      "hover:bg-transparent",
                    ])}
                  >
                    {row.getVisibleCells().map((cell) => (
                      <TableCell
                        key={cell.id}
                        className={cn([
                          "py-3",
                          "transition-colors",
                          "group-hover:bg-muted/50",
                          cell.column.id === "actions" && "w-28",
                        ])}
                      >
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              : !loading && (
                  <TableRow>
                    <TableCell
                      colSpan={columns.length}
                      className={cn(["h-24", "text-center"])}
                    >
                      {t("game:notice.empty")}
                    </TableCell>
                  </TableRow>
                )}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  );
}
