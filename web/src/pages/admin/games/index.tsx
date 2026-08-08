import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ListOrderedIcon } from "lucide-react";
import { parseAsInteger, useQueryState } from "nuqs";
import { useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { type GetGamesRequest, getGames } from "@/api/admin/games";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Field, FieldIcon } from "@/components/ui/field";
import { LoadingOverlay } from "@/components/ui/loading-overlay";
import { Pagination } from "@/components/ui/pagination";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Select } from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useDebounce } from "@/hooks/use-debounce";
import type { GameDetail } from "@/models/game";
import { useConfigStore } from "@/storages/config";
import { useSharedStore } from "@/storages/shared";
import { cn } from "@/utils";
import {
  flexRender,
  type SortingState,
  useDataTable,
} from "@/utils/data-table";
import { RowProvider, useColumns } from "./_blocks/columns";
import { CreateDialog } from "./_blocks/create-dialog";
import { GameListContext } from "./context";

function useGameQuery(params: GetGamesRequest) {
  const { refresh } = useSharedStore();

  return useQuery({
    queryKey: [
      "games",
      params.id,
      params.title,
      params.size,
      params.page,
      params.enabled,
      params.sorts,
      refresh,
    ],
    queryFn: () => getGames(params),
    select: (response) => ({
      games: response.games || [],
      total: response.total || 0,
    }),
    enabled: !!params,
    placeholderData: keepPreviousData,
  });
}

export default function Index() {
  const { t } = useTranslation();
  const configStore = useConfigStore();
  const {
    createDialogOpen,
    setCreateDialogOpen,
    columnFilters,
    setColumnFilters,
  } = useContext(GameListContext)!;

  const [page, setPage] = useQueryState("page", parseAsInteger.withDefault(1));
  const [size, setSize] = useQueryState("size", parseAsInteger.withDefault(10));
  const [sorting, setSorting] = useState<SortingState>([
    { id: "started_at", desc: true },
  ]);
  const debouncedColumnFilters = useDebounce(columnFilters, 100);

  const enabled =
    (debouncedColumnFilters.find((c) => c.id === "enabled")
      ?.value as string) !== "all"
      ? (debouncedColumnFilters.find((c) => c.id === "enabled")
          ?.value as string) === "true"
      : undefined;

  const { data: gamesData, isLoading: loading } = useGameQuery({
    id: debouncedColumnFilters.find((c) => c.id === "id")?.value as number,
    title: debouncedColumnFilters.find((c) => c.id === "title")
      ?.value as string,
    enabled,
    sorts: sorting.map((s) => (s.desc ? `-${s.id}` : s.id)).join(","),
    page,
    size,
  });

  const columns = useColumns();
  const table = useDataTable<GameDetail>({
    data: gamesData?.games || [],
    columns,
    manualPagination: true,
    rowCount: gamesData?.total,
    manualFiltering: true,
    onColumnFiltersChange: setColumnFilters,
    manualSorting: true,
    onSortingChange: (updater) => {
      setSorting(updater);
      void setPage(1);
    },
    state: { sorting, columnFilters },
  });

  return (
    <>
      <title>{`${t("game:_")} - ${configStore?.config?.meta?.title}`}</title>
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <CreateDialog onClose={() => setCreateDialogOpen(false)} />
        </DialogContent>
      </Dialog>
      <div
        className={cn([
          "overflow-hidden",
          "flex",
          "flex-col",
          "min-h-0",
          "h-full",
          "px-4",
          "py-4",
          "sm:px-6",
          "sm:py-6",
          "lg:px-8",
          "lg:py-8",
          "gap-4",
        ])}
      >
        <ScrollArea
          className={cn([
            "flex-1",
            "min-h-0",
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
              "min-w-224",
              "table-fixed",
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
                  {headerGroup.headers.map((header) => (
                    <TableHead
                      key={header.id}
                      className={cn([
                        "bg-muted/95",
                        header.column.id === "game" && ["min-w-64"],
                        header.column.id === "status" && ["w-52"],
                        header.column.id === "started_at" && ["w-80"],
                        header.column.id === "actions" && [
                          "sticky",
                          "right-0",
                          "z-3",
                          "w-24",
                          "bg-muted/95",
                        ],
                      ])}
                    >
                      {!header.isPlaceholder &&
                        flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  ))}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows?.length ? (
                table.getRowModel().rows.map((row) => (
                  <RowProvider key={row.original.id} game={row.original}>
                    <TableRow
                      data-state={row.getIsSelected() ? "selected" : undefined}
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
                            cell.column.id === "actions" && [
                              "sticky",
                              "right-0",
                              "z-1",
                              "bg-card",
                            ],
                          ])}
                        >
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext()
                          )}
                        </TableCell>
                      ))}
                    </TableRow>
                  </RowProvider>
                ))
              ) : !loading ? (
                <TableRow>
                  <TableCell
                    colSpan={columns.length}
                    className={cn([
                      "h-40",
                      "text-center",
                      "text-muted-foreground",
                    ])}
                  >
                    {t("game:empty")}
                  </TableCell>
                </TableRow>
              ) : null}
            </TableBody>
          </Table>
        </ScrollArea>
        <footer
          className={cn([
            "flex",
            "flex-col",
            "gap-3",
            "sm:flex-row",
            "sm:items-center",
            "sm:justify-between",
            "shrink-0",
          ])}
        >
          <p className={cn(["text-sm", "text-muted-foreground"])}>
            {t("game:result_count", { count: gamesData?.total ?? 0 })}
          </p>
          <div
            className={cn([
              "flex",
              "flex-wrap",
              "items-center",
              "gap-3",
              "min-h-10",
            ])}
          >
            <Pagination
              size="sm"
              value={page}
              total={Math.ceil((gamesData?.total || 0) / size)}
              onChange={setPage}
            />
            <Field size="sm" className={cn(["w-32", "sm:w-36"])}>
              <FieldIcon>
                <ListOrderedIcon className="size-4" />
              </FieldIcon>
              <Select
                options={[
                  { value: "10" },
                  { value: "20" },
                  { value: "40" },
                  { value: "60" },
                ]}
                value={String(size)}
                onValueChange={(value) => {
                  void setSize(Number(value));
                  void setPage(1);
                }}
              />
            </Field>
          </div>
        </footer>
      </div>
    </>
  );
}
